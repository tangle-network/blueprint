use blueprint_store_local_database::{Error, LocalDatabase};
use serde::{Deserialize, Serialize, Serializer};
use std::cell::Cell;
use std::collections::HashMap;
use std::fs;

thread_local! {
    static REJECT_SERIALIZATION: Cell<bool> = const { Cell::new(false) };
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
struct Value(u32);

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if REJECT_SERIALIZATION.get() {
            return Err(serde::ser::Error::custom("injected serialization failure"));
        }
        self.0.serialize(serializer)
    }
}

type Mutation = fn(&LocalDatabase<Value>) -> Result<(), Error>;

const MUTATIONS: [(&str, Mutation); 4] = [
    ("set", |db| db.set("target", Value(9))),
    ("update", |db| {
        db.update("target", |value| value.0 = 9).map(|_| ())
    }),
    ("remove", |db| db.remove("target").map(|_| ())),
    ("replace", |db| {
        db.replace(HashMap::from([("new".into(), Value(9))]))
    }),
];

fn failed_mutations_preserve_state(serialization_failure: bool) {
    let mut changed = Vec::new();
    for (name, mutate) in MUTATIONS {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("database.json");
        let db = LocalDatabase::<Value>::open(&path).unwrap();
        db.set("target", Value(1)).unwrap();
        // Retain a value so remove must still serialize the resulting map.
        db.set("retained", Value(2)).unwrap();
        let before: HashMap<_, _> = db.entries().unwrap().into_iter().collect();
        let disk_before = fs::read(&path).unwrap();
        if serialization_failure {
            REJECT_SERIALIZATION.set(true);
        } else {
            // Deterministic even when tests run as root; no permission tricks.
            fs::create_dir(path.with_extension("tmp")).unwrap();
        }
        let result = mutate(&db);
        REJECT_SERIALIZATION.set(false);
        if serialization_failure {
            assert!(matches!(result, Err(Error::Serialization(_))), "{name}");
        } else {
            assert!(matches!(result, Err(Error::Io(_))), "{name}");
        }
        assert_eq!(fs::read(&path).unwrap(), disk_before, "{name}");
        let reopened = LocalDatabase::<Value>::open(&path).unwrap();
        assert_eq!(
            reopened
                .entries()
                .unwrap()
                .into_iter()
                .collect::<HashMap<_, _>>(),
            before,
            "{name}"
        );
        if db.entries().unwrap().into_iter().collect::<HashMap<_, _>>() != before {
            changed.push(name);
        }
    }
    assert!(
        changed.is_empty(),
        "failed mutations published in memory: {changed:?}"
    );
}

#[test]
fn serialization_failure_preserves_memory_and_disk() {
    failed_mutations_preserve_state(true);
}

#[test]
fn temporary_file_failure_preserves_memory_and_disk() {
    failed_mutations_preserve_state(false);
}

#[test]
fn missing_key_mutations_do_not_flush_or_call_update() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("database.json");
    let db = LocalDatabase::<Value>::open(&path).unwrap();
    db.set("retained", Value(2)).unwrap();
    let before = fs::read(&path).unwrap();
    fs::create_dir(path.with_extension("tmp")).unwrap();
    assert_eq!(db.remove("missing").unwrap(), None);
    assert!(
        !db.update("missing", |_| panic!("missing update called"))
            .unwrap()
    );
    assert_eq!(fs::read(&path).unwrap(), before);
    assert_eq!(db.get("retained").unwrap(), Some(Value(2)));
}

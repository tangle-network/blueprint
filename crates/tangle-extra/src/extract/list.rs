use core::fmt::Debug;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

// TODO(serial): Document, warn against using `Vec<T>` in tangle jobs
#[derive(Deserialize, Copy, Eq, Hash)]
#[serde(transparent)]
pub struct List<T: Default>(pub Vec<T>);

impl<T: PartialEq + Default> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Debug + Default> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: PartialOrd + Default> PartialOrd for List<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord + Default> Ord for List<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Clone + Default> Clone for List<T> {
    fn clone(&self) -> Self {
        List(self.0.clone())
    }
}

impl<T: Default> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T: Default + Serialize> Serialize for List<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.is_empty() {
            let mut s = serializer.serialize_seq(Some(1))?;
            s.serialize_element(&T::default())?;
            return s.end();
        }

        <Vec<T>>::serialize(&self.0, serializer)
    }
}

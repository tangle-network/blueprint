use core::fmt::Debug;
use serde::{Deserialize, Serialize, Serializer};

// TODO(serial): Document, warn against using `Option<T>` in tangle jobs
#[derive(Deserialize, Copy, Eq, Hash)]
#[serde(transparent)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct Optional<T: Default>(pub Option<T>);

impl<T: PartialEq + Default> PartialEq for Optional<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Debug + Default> Debug for Optional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: PartialOrd + Default> PartialOrd for Optional<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord + Default> Ord for Optional<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: Clone + Default> Clone for Optional<T> {
    fn clone(&self) -> Self {
        Optional(self.0.clone())
    }
}

impl<T: Default> Default for Optional<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Default> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T: Default + Serialize> Serialize for Optional<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.is_none() {
            return serializer.serialize_some(&T::default());
        }

        <Option<T>>::serialize(&self.0, serializer)
    }
}

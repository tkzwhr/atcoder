use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

pub struct THHashMap<K, V>(pub HashMap<K, HashSet<V>>);

#[allow(dead_code)]
impl<K, V> THHashMap<K, V>
where
    K: Hash + Eq,
    V: Hash + Eq,
{
    pub fn insert(&mut self, k: K, v: V) {
        if let Some(handle) = self.0.get_mut(&k) {
            handle.insert(v);
        } else {
            let value = HashSet::from([v]);
            self.0.insert(k, value);
        }
    }
}

pub struct THBTreeMap<K, V>(pub BTreeMap<K, BTreeSet<V>>);

#[allow(dead_code)]
impl<K, V> THBTreeMap<K, V>
where
    K: Ord,
    V: Ord,
{
    pub fn insert(&mut self, k: K, v: V) {
        if let Some(handle) = self.0.get_mut(&k) {
            handle.insert(v);
        } else {
            let value = BTreeSet::from([v]);
            self.0.insert(k, value);
        }
    }
}

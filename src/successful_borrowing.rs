#![allow(warnings)]

pub struct Map<K: Eq, V> {
    elements: Vec<(K, V)>,
}

impl<K: Eq, V> Map<K, V> {
    pub fn new() -> Self {
        Map { elements: vec![] }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.elements.push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.elements.iter().rev().find(|pair| pair.0 == *key).map(|pair| &pair.1)
    }

    pub fn remove(&mut self, key: &K) {
        for (index, pair) in self.elements.iter().enumerate() {
            if pair.0 == *key {
                // PROMPT self.elements.remove(index);
                // PROMPT return;
            }
        }
    }
}


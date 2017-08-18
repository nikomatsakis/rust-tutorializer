#![allow(warnings)]

// **Exercise 1.** Modify the signature of `get` to use an explicit
// lifetime name in the return type.
//
// **Exercise 2.** Change the signature of `get` to:
//
//     fn get<'a>(&'a mut self, key: &'a K) -> Option<&'a V>
//
// - Which test(s) fails to compile?
// - Can you explain why?
//
// **Exercise 3.** Rewrite `insert` so that it doesn't just push
// a tuple onto `self.elements`, but instead searches to see if there
// is already an entry with that same key and, if so, overwrites that
// entry in place.

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
}

#[test]
fn basic() {
    let mut map = Map::new();

    map.insert('a', format!("alpha"));
    map.insert('b', format!("beta"));
    map.insert('g', format!("gamma"));

    assert_eq!(map.get(&'a'), Some(&format!("alpha")));
    assert_eq!(map.get(&'b'), Some(&format!("beta")));
    assert_eq!(map.get(&'g'), Some(&format!("gamma")));

    map.insert('a', format!("xxx"));

    assert_eq!(map.get(&'a'), Some(&format!("xxx")));
    assert_eq!(map.get(&'b'), Some(&format!("beta")));
    assert_eq!(map.get(&'g'), Some(&format!("gamma")));
}

#[test]
fn lock_receiver() {
    let mut map = Map::new();
    let mut string = format!("alpha");
    map.insert('a', string.clone());
    let r = map.get(&'a');
    assert_eq!(r, Some(&string));
}

#[test]
fn overwrites_len() {
    let mut map = Map::new();
    map.insert('a', format!("alpha"));
    map.insert('b', format!("beta"));
    map.insert('a', format!("alpha2"));

    assert_eq!(map.get(&'a'), Some(&format!("alpha2")));
    assert_eq!(map.get(&'b'), Some(&format!("beta")));

    // Here we break the abstraction barrier and observe that
    // `insert()` is appending even if an entry with the given key
    // already exists. When you complete exercise 3, this assertion
    // should fail, and you will have to change it to assert that the
    // length is only 2.
    assert_eq!(map.elements.len(), 3);
}

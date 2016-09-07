#![allow(warnings)]

// **Exercise 1.** For the method `get`, identify at least 4 lifetimes
// that must be inferred.
//
// **Exercise 2.** Modify the signature of `get` such that the method
// `get` fails to compile with a lifetime inference error.
//
// **Exercise 3.** Modify the signature of `get` such that the
// `do_not_compile` test fails to compile with a lifetime error
// (but `get` does not have any errors).
//
// **Exercise 4.** There are actually two ways to achieve Exercise 3.
// Can you find the other one?

pub struct Map<K: Eq, V> {
    elements: Vec<(K, V)>,
}

impl<K: Eq, V> Map<K, V> {
    pub fn new() -> Self {
        Map { elements: vec![] }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let matching_pair: Option<&(K, V)> = <[_]>::iter(&self.elements)
            .rev()
            .find(|pair| pair.0 == *key);
        matching_pair.map(|pair| &pair.1)
    }
}

#[test]
// START SOLUTION
#[should_panic]
// END SOLUTION
fn do_not_compile() {
    let map: Map<char, String> = Map::new();
    let r;
    let key = &'c';
    r = map.get(key);
    panic!("If this test is running, your program compiled, and that's bad!");
}

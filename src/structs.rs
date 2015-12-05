#![allow(dead_code)]

use std::collections::HashMap;

struct Store {
    name: String,
    prices: HashMap<String, f32>,
}

impl Store {
    fn new(name: String) -> Store {
        Store {
            name: name,
            prices: HashMap::new(),
        }
    }

    fn add_item(&mut self, name: String, price: f32) {
        self.prices.insert(name, price);
    }

    fn price(&self, item_name: &str) -> f32 {
        self.prices[item_name]
    }

    fn total_price(&self, shopping_list: &[String]) -> f32 {
        // START SOLUTION
        shopping_list.iter()
                     .map(|name| self.price(name))
                     .fold(0.0, |a, b| a + b)
        // END SOLUTION
    }
}

fn build_store() -> Store {
    let mut store = Store::new(format!("Rustmart"));
    store.add_item(format!("chocolate"), 5.0);
    store.add_item(format!("socks"), 23.0);
    store.add_item(format!("plush Mozilla dinosaur"), 13.0);
    store
}

#[test]
fn total_price() {
    let store = build_store();
    let list = vec![format!("chocolate"),
                    format!("plush Mozilla dinosaur")];
    assert_eq!(store.total_price(&list), 18.0);
}


#![allow(dead_code)]

#[derive(Debug)]
struct Store {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    name: &'static str,
    price: f32,
    tax_rate: TaxRate,
}

#[derive(Debug)]
enum TaxRate {
    TaxExempt,
    SalesTax(f32),
}

impl Store {
    fn new(name: String) -> Store {
        Store {
            name: name,
            items: vec![],
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn price(&self, item_name: &str) -> f32 {
        for item in &self.items {
            if item.name == item_name {
                let base_price = item.price;
                // Goal 1. Adjust for `item.tax_rate`
                //
                // Goal 2. Once you have things working, try removing
                // one of the arms in your `match` expression and
                // see what happens.

                // PROMPT return base_price;
                // START SOLUTION
                return match item.tax_rate {
                    TaxRate::TaxExempt =>
                        base_price,
                    TaxRate::SalesTax(percent) =>
                        base_price + base_price * percent,
                };
                // END SOLUTION
            }
        }

        panic!("no such item {:?}", item_name);
    }

    fn total_price(&self, shopping_list: &[&str]) -> f32 {
        shopping_list.iter()
                     .map(|name| self.price(name))
                     .fold(0.0, |a, b| a + b)
    }
}

fn build_store() -> Store {
    use self::TaxRate::*;

    let mut store = Store::new(format!("Rustmart"));
    store.add_item(Item {
        name: "chocolate",
        price: 5.0,
        tax_rate: TaxExempt,
    });
    store.add_item(Item {
        name: "socks",
        price: 23.0,
        tax_rate: SalesTax(0.05),
    });
    store.add_item(Item {
        name: "plush Mozilla dinosaur",
        price: 13.0,
        tax_rate: SalesTax(0.05),
    });
    store
}

#[test]
fn total_price() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur"];
    assert_eq!(store.total_price(&list), 18.65);
}


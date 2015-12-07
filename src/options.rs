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

    fn price(&self, item_name: &str) -> Option<f32> {
        for item in &self.items {
            if item.name == item_name {
                let base_price = item.price;
                let adjusted_price = match item.tax_rate {
                    TaxRate::TaxExempt =>
                        base_price,
                    TaxRate::SalesTax(percent) =>
                        base_price + base_price * percent,
                };
                return Some(adjusted_price);
            }
        }

        None
    }

    fn total_price(&self, shopping_list: &[&str]) -> Option<f32> {
        let mut sum = 0.0;
        // PROMPT for name in shopping_list {
        // PROMPT     sum += self.price(name);
        // PROMPT }
        // PROMPT sum
        // START SOLUTION
        for name in shopping_list {
            sum += match self.price(name) {
                Some(s) => s,
                None => return None
            };
        }
        Some(sum)
        // END SOLUTION
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
    assert_eq!(store.total_price(&list), Some(18.65));
}

#[test]
fn missing_item() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur", "milk"];
    assert_eq!(store.total_price(&list), None);
}


use std::collections::HashMap;
use itertools::Itertools;

/// https://www.codewars.com/kata/54dc6f5a224c26032800005c/train/rust
pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() {
        return "".to_string();
    }

    let stock_items: Vec<StockItem> = list_art.iter()
        .map(|item_str| item_str.to_string())
        .map(StockItem::new)
        .collect();

    let stock = create_stock(stock_items);

    let format = |category: String| -> String {
        let quantity = stock.get(category.as_str())
            .unwrap_or(&0);
        let quantity_as_string = quantity.to_string();

        format!("({} : {})", category, quantity_as_string)
    };

    return list_cat.iter()
        .map(|category_str| category_str.to_string())
        .map(format)
        .join(" - ");
}

fn create_stock(stock_items: Vec<StockItem>) -> HashMap<String, i32> {
    let mut stock: HashMap<String, i32> = HashMap::new();

    for stock_item in stock_items {
        let key = stock_item.category.as_str();

        if stock.contains_key(key) {
            let amount = stock.get(key).unwrap();
            stock.insert(stock_item.category, *amount + stock_item.quantity);
        } else {
            stock.insert(stock_item.category, stock_item.quantity);
        }
    }

    stock
}

struct StockItem {
    category: String,
    quantity: i32,
}

impl StockItem {
    pub fn new(input: String) -> Self {
        let split: Vec<&str> = input.split(' ').collect();

        let category = split.first()
            .unwrap()
            .to_string()
            .chars()
            .nth(0)
            .unwrap()
            .to_string();

        let quantity = split.get(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        Self {
            category,
            quantity,
        }
    }
}

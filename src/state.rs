use cw_storage_plus::Item;

pub const PAYMENT: Item<Vec<String>> = Item::new("payment"); // allowed payment currencies

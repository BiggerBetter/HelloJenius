use super::user::User;
use super::product::Product;
use crate::traits::display::Display;

#[derive(Clone)]
pub struct Order {
    pub customer: User,
    pub items: Vec<Product>,
}

impl Order {
    pub fn new(customer: User, items: Vec<Product>) -> Self {
        Self { customer, items }
    }
}

impl Display for Order {
    fn info(&self) -> String {
        let item_list: Vec<String> = self.items.iter().map(|item| item.info()).collect();
        format!(
            "Order for {}:\n{}\n",
            self.customer.name,
            item_list.join("\n")
        )
    }
}
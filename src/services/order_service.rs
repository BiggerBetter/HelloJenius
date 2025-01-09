use crate::models::order::Order;

pub struct OrderService;

impl OrderService {
    pub fn new() -> Self {
        Self
    }

    pub fn process_order(&self, order: &Order) {
        println!("Processing order for customer: {}", order.customer.name);
        println!("Items in the order:");
        for item in &order.items {
            println!(" - {}", item.name);
        }
        println!("Order processed successfully!");
    }
}
mod models;
mod services;
mod image;
mod traits;

use models::user::User;
use models::product::Product;
use models::order::Order;
use services::order_service::OrderService;
use traits::display::Display;

fn main() {
    let user = User::new("Alice", 30);
    println!("{}", user.greet());

    let product = Product::new("Laptop", 1500.0);
    println!("{}", product.info());

    let order = Order::new(user.clone(), vec![product.clone()]);
    println!("{}", order.info());

    let order_service = OrderService::new();
    order_service.process_order(&order);
}
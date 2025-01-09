mod models;
mod services;

use models::user::User;
use services::user_service::UserService;

fn main() {
    let user = User::new("Alice", 30);
    println!("{}", user.greet());

    let user_service = UserService::new();
    user_service.print_user_info(&user);
}
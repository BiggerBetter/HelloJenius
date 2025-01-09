use crate::models::user::User;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }

    pub fn print_user_info(&self, user: &User) {
        println!("User Info: Name = {}, Age = {}", user.name, user.age);
    }
}
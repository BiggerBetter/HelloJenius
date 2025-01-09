pub struct Product {
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new(name: &str, price: f64) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }

    pub fn info(&self) -> String {
        format!("Product: {}, Price: ${:.2}", self.name, self.price)
    }
}
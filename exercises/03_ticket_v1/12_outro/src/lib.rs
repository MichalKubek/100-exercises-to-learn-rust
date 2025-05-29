// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

use core::panic;

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}


impl Order {

    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Self {
        let mut ord: Order = Order { product_name: String::new(), quantity: 0, unit_price: 0};
        ord.set_product_name(product_name);
        ord.set_quantity(quantity);
        ord.set_unit_price(unit_price);
        ord
    }
    pub fn set_product_name(&mut self, product_name: String) {
        if product_name.is_empty() || product_name.len() > 300 {
            panic!("Invalid product name");
        }

        self.product_name = product_name
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn set_quantity(&mut self, quantity: i32)
    {
        if quantity <= 0 {
            panic!("Quantity need to be bigger then 0");
        }
        self.quantity = quantity

    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity 
    }

    pub fn set_unit_price(&mut self, price: i32) {
        if price <= 0 {
            panic!("Unit price need to be bigger than 0");
        }
        self.unit_price = price
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    pub fn total(&self) -> i32 {
        self.unit_price * self.quantity as i32
    }
    
}

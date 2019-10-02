pub mod models;

use self::models::Contact;

fn main() {
    println!("Hello, world!");
    let mut contact = Contact::new();
    contact.set_firstname("Li".to_string());

    println!("{}", contact.to_string())
}

struct GroceryItem {
    stock: i32,
    price: f64,
}
fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    print!("Stock: {:?}", cereal.stock);
    print!("Price: {:?}", cereal.price);
}

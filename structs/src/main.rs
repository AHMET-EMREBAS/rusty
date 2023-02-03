#[derive(Debug)]
struct Product {
    name: String,
    active: bool,
    likes: u32,
    cost: f32,
    price: f32,
}

impl Product {
    fn profit(&self) {
        let profit = self.price - self.cost;
        println!("Profit for each product is {profit}");
    }
}

#[derive(Debug)]
struct Color(u32, u32, u32);

#[derive(Debug)]
struct Point(u32, u32, u32);

#[derive(Debug)]
struct Dimention(u32, u32);

fn main() {
    let mut product = new_product("New Product");

    println!("The product is {:?}", product);
    product.profit();

    let color = Color(10, 101, 100);
    let point = Point(10, 101, 100);
    let dx = Dimention(40, 100);

    println!("The dimentaion is {:?}", dx);
    let d = area(&dx);
    let d1 = area(&dx);

    println!("The dimention of 40x100 is {d}");
    println!("The dimention of 40x100 is {d1}");
}

fn new_product(name: &str) -> Product {
    Product {
        name: String::from(name),
        active: true,
        likes: 0,
        price: 300.99,
        cost: 40.22,
    }
}

fn area(dimensions: &Dimention) -> u32 {
    dimensions.0 * dimensions.1
}

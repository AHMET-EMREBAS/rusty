use std::fmt::{format, Display};

use interfaces::{intent::Indent, summary::Summary};

struct Product {
    name: String,
    barcode: String,
}

impl Indent for Product {
    fn indent(&self) -> String {
        format!("||||||||| {}", self.summary())
    }
}

impl Summary for Product {
    fn summary(&self) -> String {
        format!("{}:{}", self.name, self.barcode)
    }
}

fn main() {
    println!("Hello, world!");

    let product = Product {
        name: String::from("Product name"),
        barcode: String::from("12312471924"),
    };

    let summary_value = summurize(&product);
    let indent_value = indents(&product);

    println!("Product summary : {}", &summary_value);
    println!("Product indent :  {}", &indent_value);
}

fn summurize<T: Summary>(item: &T) -> String {
    item.summary()
}

fn indents<T: Indent + Summary>(item: &T) -> String {
    item.indent()
}

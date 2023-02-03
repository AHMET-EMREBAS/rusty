fn main() {
    let mut v: Vec<i32> = Vec::new();
    add(2, &mut v);
    add(1, &mut v);
    add(3, &mut v);

    let first_element = get(0, &v);
    let second_element = get(1, &v);

    let third_element = v[2];

    println!("Vector List: {:?}", v);

    println!("First element -> {:?}", first_element);
    println!("Second element -> {:?}", second_element);
    println!("Third element -> {:?}", third_element);

    println!("------------------------------------------");

    let mut v1 = Vec::from([3, 5, 1, 2, 14]);

    println!("Unsorted: {:?}", v1);
    v1.sort();
    println!("Sorted: {:?}", v1);
}

fn add(value: i32, v: &mut Vec<i32>) {
    v.push(value);
}

fn get(index: usize, v: &Vec<i32>) -> Option<&i32> {
    v.get(index)
}

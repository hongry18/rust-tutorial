use std::slice::SliceIndex;

fn main() {
    let arr = vec![1, 2, 3];

    get_value(&arr, 1);
    get_value(&arr, 2);
    get_value(&arr, 3);
}

fn get_value(arr: &Vec<i32>, find: usize) {
    match arr.get(find) {
        Some(x) => println!("x is {}", x),
        None => println!("oops, out of bounds"),
    }
}

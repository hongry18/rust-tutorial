// pub trait Iterator {
//     type Item;

//     fn any<F>(&mut self, f: F) -> bool
//     where
//         F: FnMut(Self::Item) -> bool,
//     {
//     }
// }
/// iterator::any
fn ex01() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];

    println!("2 in arr1: {}", arr1.iter().any(|&x| x == 2));
    println!("2 in arr2: {}", arr2.into_iter().any(|&x| x == 2));
}

/// searching through iterators
fn ex02() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&&x| x == 2)
    );
}

pub fn run() {
    ex01();
    ex02();
}

use std::collections::HashMap;

fn main() {
    let _weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    let _byte_buffer = [0_u8; 512];

    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("first element of the array {}", letters[0]);
    println!("second element of the array {}", letters[1]);

    //println!("invalid array access: {}", letters[99]);

    // Vector
    let three_numbers = vec![1,2,3];
    println!("Initial vector: {:?}", three_numbers);

    let ten_zeros = vec![0; 10];
    println!("Ten zeros: {:?}", ten_zeros);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let mut v = vec![1,2];
    let two = v.pop();
    println!("poped number: {:?}", two);

    // vector index
    let mut v = vec![1,2,3];
    let _three = v[2];
    v[1] = v[1] + 5;

    let _v = vec![1,2,3,4,5];
    //let does_not_exist = v[100];

    // HashMap
    
    let mut book_reviews: HashMap<String, String> = HashMap::new();

    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );

    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );

    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );

    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    println!("hashMap: {:?}", book_reviews);

    if !book_reviews.contains_key("Lee Miserables") {
        println!("We've got {} reviews, but Les Miserables ain't one.",
        book_reviews.len(),
        );
    }

    println!("Review for Jane: {}", book_reviews["Pride And Prejudice"]);
    // println!("Review for Herman: {}", book_reviews["Moby Dick"]); // panics!

    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), true);
}

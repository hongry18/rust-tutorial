fn main() {
    let numbers = (1,2,3);
    //let second = todo!("Replace with the tuple indexing syntax");
    let second = numbers.0;

    assert_eq!(2, second, "This is not the 2nd number in the tuple: {}", second);

    indexing_array();
}

fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    //let letter_d = todo!("Replace with the array indexing syntax");
    let letter_d = characters[2];

    assert_eq!('d', letter_d, "This is not the character for the letter d: {}", letter_d);
}

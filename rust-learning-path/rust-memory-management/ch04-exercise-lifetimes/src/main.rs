// TODO modify only this function
fn copy_and_return(vector: &mut Vec<String>, value: &str) -> String {
    vector.push(String::from(value));

    let mut result: String = String::from("");
    for name in vector.into_iter() {
        match name {
            value => result = value.to_string(),
        }
    }

    result
}

fn copy_and_return2<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();
    assert_eq!("Joe", copy_and_return2(&mut names, &name1));
    assert_eq!("Chris", copy_and_return2(&mut names, &name2));
    assert_eq!("Anne", copy_and_return2(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    );
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic]
fn add_works2() {
    assert_eq!(add(5, -2), 7);
}

#[test]
#[ignore = "not yet reviewed by the Q.A. team"]
fn add_works3() {
    assert_eq!(add(5, -2), 7);
}

#[cfg(test)]
mod add_functions_tests {
    use super::*;

    #[test]
    fn add_workds() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_works2() {
        assert_eq!(add(5, -2), 7);
    }

    #[test]
    #[ignore = "not yet reviewed by the Q.A. team"]
    fn add_works3() {
        assert_eq!(add(5, -2), 7);
    }
}

fn main() {
    println!("hello world");
}

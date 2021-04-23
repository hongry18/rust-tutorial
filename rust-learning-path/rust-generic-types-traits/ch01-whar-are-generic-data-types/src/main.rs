struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point {
        x: "high",
        y: "low",
    };

    // panic
    //let wont_work = Point { x: 25, y: true };
    let wont_work = Point2 { x: 25, y: true };
}

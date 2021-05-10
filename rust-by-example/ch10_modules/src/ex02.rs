mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }

        pub fn print(self) -> T {
            self.contents
        }
    }
}

pub fn run() {
    let open_box = my::OpenBox {
        contents: "public information",
    };

    println!("The open box contains: {}", open_box.contents);

    let _closed_box = my::ClosedBox::new("classified information");
    println!("The closed box contains: {}", _closed_box.print());
}

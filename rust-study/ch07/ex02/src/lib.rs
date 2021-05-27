mod front_of_houest {
    mod hosting {
        fn add_to_waitlist() {
            println!("add_to_waitlist")
        }

        fn seat_at_table() {
            println!("seat_at_table")
        }
    }

    mod serving {
        fn take_order() {
            println!("Take Order")
        }

        fn serve_order() {
            println!("Serve Order")
        }

        fn take_payment() {
            println!("Take Payment")
        }
    }
}

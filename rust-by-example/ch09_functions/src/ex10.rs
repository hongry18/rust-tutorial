#![feature(never_type)]

fn some_fn() {
    ()
}

fn ex01() {
    let a: () = some_fn();
    println!("This function returns and you can see this line.")
}

fn ex02() {
    // let x: ! = panic!("this call never returns");
    println!("You will never see this line!");
}

fn ex03() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;

        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }

        acc
    }

    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}

pub fn run() {
    ex01();
    ex02();
    ex03();
}

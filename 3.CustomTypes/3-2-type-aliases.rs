enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x+y,
            Self::Substract => x-y,
        }
    }
}

fn main() {
    let x = Operations::Add;
    println!("{}", x.run(3,2));
}

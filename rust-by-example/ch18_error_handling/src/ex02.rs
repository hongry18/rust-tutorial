fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_royal(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s !!!!!!", inside);
}

// Option & unwrap
fn ex01() {
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_royal(bird);
    give_royal(nothing);
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

// Unpacking options with
fn ex02() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 43922222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}

#[allow(dead_code)]
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

/// Combinators: map
fn ex03() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

#[derive(Debug)]
enum Food2 {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(food: Food2) -> Option<Food2> {
    match food {
        Food2::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food2) -> Option<Food2> {
    match food {
        Food2::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food2) -> Option<Food2> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: Food2) -> Option<Food2> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat2(food: Food2, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

/// Combinators: and_then
fn ex04() {
    let (cordon_bleu, steak, sushi) = (Food2::CordonBleu, Food2::Steak, Food2::Sushi);
    eat2(cordon_bleu, Day::Monday);
    eat2(steak, Day::Tuesday);
    eat2(sushi, Day::Wednesday);
}

/// Option & unwrap
pub fn main() {
    // ex01();
    ex02();
    ex03();
    ex04();
}

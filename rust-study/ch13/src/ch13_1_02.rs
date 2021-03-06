use std::thread;
use std::time::Duration;

// 제너릭 파라미터와 Fn 트레잇을 사용하여 클로저 저장하기
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn closer_capture() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let z1 = equal_to_x(4);
    let z2 = equal_to_x(5);

    println!("z1 is {}, z2 is {}", z1, z2);
}

#[test]
fn closer_capture_2() {
    // let x = 4;

    // fn equal_to_x(z: i32) -> bool { z == x }

    // let y = 4;

    // assert!(equal_to_x(y));
}
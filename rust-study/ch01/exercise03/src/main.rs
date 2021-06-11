use std::io;

fn main() {
    println!("Please input numbers.");
    let mut str_nums = String::new();

    io::stdin()
        .read_line(&mut str_nums)
        .expect("Failed to read line");

    // 마지막 10, 13 처리 방식을 잘 모르겠어서 그냥 이렇게 함
    let nums = match str_nums[0..str_nums.len() - 1].parse::<i64>() {
        Ok(n) => n,
        Err(_) => panic!("numbers only allowed"),
    };

    let item = comma_sperator_number(nums);
    // string formatting 이 생각보다 난해 함.
    println!("{:0>20}", item);
}

// comma sperator crate를 사용하려 했지만 그냥 함수 만들어서 사용
fn comma_sperator_number(i: i64) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (k, v) in a {
        if k != 0 && k % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, v);
    }

    s
}

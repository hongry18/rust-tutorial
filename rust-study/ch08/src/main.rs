fn main() {
<<<<<<< HEAD
    ex01();
    ex02();
    ex03();
    ex04();
}

/// 기본 사용법과 스코프
fn ex01() {
    let mut v: Vec<i32> = Vec::new();

    // let v2 = vec![1,2,3];
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    {
        let v = vec![1, 2, 3, 4];
        println!("{:?}", v);
    }
}

/// reading elements of vectors
fn ex02() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("the third element is {}", third);
    match v.get(2) {
        Some(third) => println!("get The third element is {}", third),
        None => println!("There is no third element"),
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        // 직접 확인은 에러가 난다
        // let _does_not_exist1 = &v[100];
        // 이렇게 Option으로 얻어오면 패닉이 일어나지 않는다
        let _does_not_exist2 = v.get(100);
    }
}

/// iterating over the values in a vector
fn ex03() {
    let v = vec![100, 32, 56];
    for i in &v {
        println!("{}", i)
    }

    let mut v = vec![100, 32, 56];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v)
}

/// using an enum to store multiple types
fn ex04() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text("test".to_string()),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row)
}
=======
    println!("Hello, world!");

    let s1 = String::from("s1");
    let s2 = String::from("s2");

    println!("s3 is {}", sum_str_plus_operator(&s1, &s2));
    println!("s3 is {}", sum_str_plus_operator(&s1, &s2));

}

fn sum_str_plus_operator(x: &String, y: &String) -> String {
    return "sum: ".to_string() + x + y
}
>>>>>>> d317dfcc665da8362a7e7f31b046b63cf30e6217

fn ex01() {
    let ss = vec!["tofu", "93", "18"];

    let ns: Vec<_> = ss.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", ns);
}

fn ex02() {
    let ss = vec!["tofu", "93", "18"];

    let ns: Vec<_> = ss
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", ns);
}

fn ex03() {
    let ss = vec!["tofu", "93", "18"];

    let ns: Result<Vec<_>, _> = ss.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", ns);
}

fn ex04() {
    let ss = vec!["tofu", "93", "18"];
    let (ns, es): (Vec<_>, Vec<_>) = ss
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", ns);
    println!("Errors: {:?}", es);
}

fn ex05() {
    let ss = vec!["tofu", "93", "18"];
    let (ns, es): (Vec<_>, Vec<_>) = ss
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let ns: Vec<_> = ns.into_iter().map(Result::unwrap).collect();
    let es: Vec<_> = es.into_iter().map(Result::unwrap_err).collect();

    println!("Numbers: {:?}", ns);
    println!("Errors: {:?}", es);
}

pub fn main() {
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
}

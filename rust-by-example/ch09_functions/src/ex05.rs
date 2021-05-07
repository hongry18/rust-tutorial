/// 'F' must be generic.
fn apply1<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply2<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn ex01() {
    let x = 7;
    let print = || println!("{}", x);

    apply1(print);
    apply2(print);
}

/// Type anonymity
pub fn run() {
    ex01();
}

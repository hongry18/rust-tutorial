fn main() {
    
}

// 댕글링 방지
fn ex01() {
    let r;         // -------+-- 'a
                   //        |
    {              //        |
        let x = 5; // -+-----+-- 'b
        r = &x;    //  |     |
    }              // -+     |
                   //        |
    println!("r: {}", r); // |
                   //        |
                   // -------+
}

fn ex02() {
    {
        let x = 5;            // -----+-- 'b
                              //      |
        let r = &x;           // --+--+-- 'a
                              //   |  |
        println!("r: {}", r); //   |  |
                              // --+  |
    }                         // -----+
}

// 함수에서 제네릭 라이프타임
fn ex03() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let result = longest(str1.as_str(), str2);
    println!("The longest string is {}", result);
}

fn longest<'c>(x: &'c str, y: &'c str) -> &'c str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
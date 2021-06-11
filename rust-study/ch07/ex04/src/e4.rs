use std::collections::HashMap;
use std::fmt;
use std::io;

//use std::fmt::Result
//use std::io::Result
// 같은 이름의 모듈을 가져오는것을 허용하지 않는다.
// 그래서 부모의 이름인 fmt::Result, io::Result 를 사용한다

// 그럼에도 정 같은 모듈명을 사용하고싶다면 aliasing을 하면 된다

use std::fmt::Result;
use std::io::Result as IoResult;

// 기타 항목을 가져올때는 전체 경로를 지정하는것이 관용적입니다.

pub fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{:?}", map)
}

fn f1() -> fmt::Result {
    Ok(())
}

fn f2() -> io::Result<()> {
    Ok(())
}

fn f3() -> Result {
    Ok(())
}

fn f4() -> IoResult<()> {
    Ok(())
}
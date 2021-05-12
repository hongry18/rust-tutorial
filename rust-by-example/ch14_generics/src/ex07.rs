#[derive(Debug)]
struct Years(i64);
#[derive(Debug)]
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

/// New Type Idiom
pub fn run() {
    let age = Years(5);
    let age_days = age.to_days();

    println!("age {:?}", age);
    println!("age_days {:?}", age_days);
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    let years = Years(42);
    let years_as_primitive: i64 = years.0;
    println!("{}", years_as_primitive);
}

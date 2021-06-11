struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = todo!("Replace this with an actual Car instance");
    //let car: Car = Car{color: color, transmission: transmission, convertible: convertible, mileage: 0};

    assert_eq!(car.mileage, 0);
    return car;
}

fn main() {
    let client_req_1 = car_factory(String::from("red"), Transmission::Manual, false);
    assert_eq!(client_req_1.color, "red");
    assert_eq!(client_req_1.transmission, Transmission::Manual);
    assert_eq!(client_req_1.convertible, false);

    let client_req_2 = car_factory(String::from("silver"), Transmission::Automatic, true);
    assert_eq!(client_req_2.color, "silver");
    assert_eq!(client_req_2.transmission, Transmission::Automatic);
    assert_eq!(client_req_2.convertible, true);

    let client_req_2 = car_factory(String::from("yellow"), Transmission::SemiAuto, false);
    assert_eq!(client_req_2.color, "yellow");
    assert_eq!(client_req_2.transmission, Transmission::SemiAuto);
    assert_eq!(client_req_2.convertible, false);

    println!("# end");
}

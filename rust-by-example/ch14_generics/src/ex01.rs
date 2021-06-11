/// A concrete type 'A'
struct A;

struct Single(A);

struct SingleGen<T>(T);


pub fn run() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SignleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
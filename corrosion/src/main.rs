mod square;
mod dir;

use self::square::Square;

fn main() {
    //let mut sq: Square = Square {i: 0, rank: 0, file: 0, loc: 0 };
    let ref mut sqref2 = Square {i: 0, rank: 0, file: 0, loc: 0 };
    some_func(sqref2);
    println!("{:?}", sqref2);
//    let sq2 = square::H1;
//    let sq3 = square::H1;
//    let ref sqref = square::H1;
//    first2(sq2);
////    first(sq);
//    second(sqref);
//    second(sqref);
//    //third(&mut sq);
//    println!("{:?}", sq3);
}

fn some_func(mut input_ref: &Square) {
    input_ref = &square::H1;
}

fn first2(square: Square) {
    println!("{:?}", square);
}

fn first(mut square: Square) {
    square.i = 5;
    println!("{:?}", square);
}

fn second(square: &Square) {
    println!("{:?}", square);
}

fn third(square: &mut Square) {
    square.i = 5;
    println!("{:?}", square);
}

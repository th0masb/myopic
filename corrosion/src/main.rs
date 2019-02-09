mod square;
mod dir;

use self::square::Square;

fn main() {
//    println!("{:?}", square::H1.all_squares(&dir::N));
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

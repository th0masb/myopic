mod square;
mod dir;

use self::square::Square;

fn main() {
    //let sq: Square = Square {i: 0, rank: 0, file: 0, loc: 0 };
    let sq = square::H1;
    //first2(sq);
//    first(sq);
    //second(&sq);
    //third(&mut sq);
    println!("{:?}", sq);
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

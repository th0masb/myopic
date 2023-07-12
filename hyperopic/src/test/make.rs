use crate::moves::Move;
use crate::position::{Discards, Position};
use crate::constants::corner;

fn execute_test(from_fen: &str, m: Move, dest_fen: &str) {
    let mut from: Position = from_fen.parse().unwrap();
    let mut dest: Position = dest_fen.parse().unwrap();
    dest.history.push(from.create_discards());
    let from_clone = from.clone();
    from.make(m.clone()).unwrap();
    assert_eq!(from, dest);
    from.unmake().unwrap();
    assert_eq!(from, from_clone);
}

#[test]
fn white_kingside() {
    execute_test(
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQkq - 0 1",
        Move::Castle { corner: corner::WK },
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R4RK1 b kq - 1 1"
    );
}





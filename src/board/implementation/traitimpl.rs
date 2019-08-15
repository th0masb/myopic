use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::Board;
use crate::board::BoardImpl;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::ReversalData;
use crate::pieces::Piece;
use crate::board::Termination;

impl Board for BoardImpl {
    fn evolve(&mut self, action: &Move) -> ReversalData {
        self.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: ReversalData) {
        self.devolve(action, discards)
    }

    fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move> {
        self.compute_moves_impl(computation_type)
    }

    fn compute_termination_status(&self) -> Option<Termination> {
        unimplemented!()
    }

    fn hash(&self) -> u64 {
        self.hashes.head()
    }

    fn active(&self) -> Side {
        self.active
    }

    fn enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.castling.status(side)
    }

    fn locs(&self, piece: Piece) -> BitBoard {
        self.pieces.locations(piece)
    }

    fn king(&self, side: Side) -> Square {
        self.pieces.king_location(side)
    }

    fn sides(&self) -> (BitBoard, BitBoard) {
        (
            self.pieces.side_locations(Side::White),
            self.pieces.side_locations(Side::Black),
        )
    }

    fn piece(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    fn half_move_clock(&self) -> usize {
        self.clock
    }

    fn history_count(&self) -> usize {
        self.hashes.position_count()
    }
}

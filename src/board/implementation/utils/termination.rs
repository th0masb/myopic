use crate::board::BoardImpl;
use crate::board::Termination;
use crate::board::Board;
use crate::board::implementation::utils::control;
use crate::base::Reflectable;


impl BoardImpl {
    pub fn compute_termination(&self) -> Option<Termination> {
        let (active, passive) = (self.active, self.active.reflect());
        let active_king = self.king(active);
        let passive_control = self.compute_control(passive);

        unimplemented!()
    }
}

use crate::actor::Actor;
use crate::math::Vec2i;

pub trait Action {
    fn clone_dyn(&self) -> Box<dyn Action>;
    fn perform(&self, actor: &mut dyn Actor);
}

pub struct NullAction {

}

impl Action for NullAction {
    fn clone_dyn(&self) -> Box<dyn Action> {
        return Box::new(NullAction{});
    }
    fn perform(&self, _actor: &mut dyn Actor) {}
}

pub struct WalkAction {
    pub offset: Vec2i
}

impl Action for WalkAction {
    fn clone_dyn(&self) -> Box<dyn Action> {
        return Box::new(WalkAction{offset: self.offset});
    }

    fn perform(&self, actor: &mut dyn Actor) {
        let old_pos = actor.get_position();
        let new_pos = old_pos + self.offset;
        actor.set_position(new_pos);
    }
}
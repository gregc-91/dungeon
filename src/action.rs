
use crate::actor::Actor;
use crate::game::Direction;
use crate::game::Level;
use crate::math::Vec2i;

pub trait Action {
    fn clone_dyn(&self) -> Box<dyn Action>;
    fn perform(&self, level: &Level, actor: &mut dyn Actor) -> ActionResult;
}

pub struct ActionResult {
    pub succeeded: bool
}

pub struct NullAction {

}

impl Action for NullAction {
    fn clone_dyn(&self) -> Box<dyn Action> {
        return Box::new(NullAction{});
    }
    fn perform(&self, _level: &Level, _actor: &mut dyn Actor) -> ActionResult {
        ActionResult { succeeded: true }
    }
}

pub struct WalkAction {
    pub direction: Direction
}

impl Action for WalkAction {
    fn clone_dyn(&self) -> Box<dyn Action> {
        return Box::new(WalkAction{direction: self.direction});
    }

    fn perform(&self, level: &Level, actor: &mut dyn Actor) -> ActionResult {
        let old_pos = actor.get_position();
        let new_pos = match self.direction {
            Direction::North => old_pos + Vec2i::new(0, -1),
            Direction::South => old_pos + Vec2i::new(0,  1),
            Direction::East => old_pos + Vec2i::new( 1,  0),
            Direction::West => old_pos + Vec2i::new(-1,  0),
        };

        if level.can_walk(new_pos) {
            actor.set_position(new_pos);
            return ActionResult { succeeded: true };
        } else {
            return ActionResult { succeeded: false };
        }
    }
}
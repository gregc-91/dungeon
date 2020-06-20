use crate::actor::Actor;
use crate::action::Action;
use crate::drawable::Drawable;
use crate::math::Vec2i;
use crate::action::NullAction;

#[derive(Copy, Clone)]
pub struct Monster {
    drawable: Drawable
}

impl Actor for Monster {
    fn get_drawable(&self) -> Drawable {
        self.drawable
    }

    fn get_position(&self) -> Vec2i {
        self.drawable.pos
    }

    fn set_position(&mut self, pos: Vec2i) {
        self.drawable.pos = pos;
    }

    fn get_action(&self) -> Box<dyn Action> {
        Box::new(NullAction{})
    }
}
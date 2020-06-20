use crate::math::Vec2i;
use crate::action::Action;
use crate::drawable::Drawable;

pub trait Actor: {
    fn get_drawable(&self) -> Drawable;
    fn get_position(&self) -> Vec2i;
    fn set_position(&mut self, pos: Vec2i);
    fn get_action(&self) -> Box<dyn Action>;
}


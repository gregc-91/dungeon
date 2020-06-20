use crate::game::Colour;
use crate::math::Vec2i;

#[derive(Copy, Clone)]
pub struct Drawable {
    pub pos: Vec2i,
    pub colour: Colour
}
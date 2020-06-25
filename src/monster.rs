use crate::actor::Actor;
use crate::action::Action;
use crate::drawable::Drawable;
use crate::game::Colour;
use crate::math::Vec2i;
use crate::action::NullAction;

#[derive(Copy, Clone)]
pub struct Breed <'a> {
    name: &'a str,
    health: u32,
    speed: u32,
    clustered: bool
}

#[derive(Copy, Clone)]
pub struct Monster<'a> {
    drawable: Drawable,
    breed: Breed<'a>
}

impl <'a> Actor for Monster<'a> {
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

impl <'a> Monster<'a> {
    pub fn goblin((x,y) : (i32, i32)) -> Monster<'a> {
        Monster {
            drawable: Drawable {
                pos: Vec2i::new(x,y),
                colour: Colour::new(65, 146, 75, 255)
            },
            breed: Breed {
                name: "Goblin",
                health: 10,
                speed: 1,
                clustered: false
            }
        }
    }
}
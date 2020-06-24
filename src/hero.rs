
use crate::actor::Actor;
use crate::action::Action;
use crate::action::NullAction;
use crate::drawable::Drawable;
use crate::game::Colour;
use crate::math::Vec2i;

pub struct Hero {
    drawable: Drawable,
    next_action: Box<dyn Action>
}

impl Actor for Hero {
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
        self.next_action.clone_dyn()
    }
}

impl Hero {
    pub fn new((x, y): (i32, i32)) -> Hero {
        Hero {
            drawable: Drawable {
                pos: Vec2i{x:x, y:y},
                colour: Colour{r:255, g:128, b:128, a:255}
            },
            next_action: Box::new(NullAction{})
        }
    }

    pub fn set_next_action(&mut self, action: Box<dyn Action>) {
        self.next_action = action;
    }
}
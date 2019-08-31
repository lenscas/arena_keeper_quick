use crate::structs::gui_2::finalize::Interaction;
use quicksilver::geom::Vector;
use quicksilver::input::MouseButton;
use quicksilver::lifecycle::Window;
use quicksilver::input::ButtonState;

#[derive(Default)]
pub struct Context<'a> {
    elements: Vec<Box<dyn Widget + 'a>>,
}
impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
    pub fn get_interaction<T:'a>(&self, widget: &mut T, window : &Window ) -> Interaction
    where
        T: Widget + Sized,
    {
        let mouse = window.mouse();
         let mouse_pos = mouse.pos();
        let clicked = mouse[MouseButton::Left] == ButtonState::Pressed;
        let is_contained = widget.contains(mouse_pos);
         let interaction = if is_contained {
            if clicked {
                Interaction::Clicked
            } else {
                Interaction::Hover
            }
        } else {
            Interaction::None
        };
        widget.set_interaction(interaction);
        interaction
    }
    pub fn push<T: 'a>(&mut self, widget: T)
    where
        T: Widget + Sized,
    {
        self.elements.push(Box::new(widget));
    }
    pub fn render(&mut self, window: &mut Window, at: &mut u32) {
        self.elements.iter().for_each(|v| {
            *at+=1;
            v.render(window,at)
        })
    }
}

pub trait Widget {
    fn render(&self, window: &mut Window, at : &mut u32);
    fn contains(&self, point: Vector) -> bool;
    fn set_interaction(&mut self, interaction : Interaction);
}

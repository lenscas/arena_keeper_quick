use crate::structs::gui_2::finalize::Interaction;
use quicksilver::geom::Vector;
use quicksilver::input::MouseButton;
use quicksilver::lifecycle::Window;

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
    pub fn push<T: 'a>(&mut self, widget: T, window: &Window) -> Interaction
    where
        T: Widget + Sized,
    {
        let mouse = window.mouse();
        let clicked = mouse[MouseButton::Left].is_down();
        let mouse_pos = mouse.pos();
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
        self.elements.push(Box::new(widget));
        interaction
    }
    pub fn render(&mut self, window: &mut Window) {
        let mouse = window.mouse();
        let clicked = mouse[MouseButton::Left].is_down();
        let mouse_pos = mouse.pos();
        self.elements.iter().for_each(|v| {
            let is_contained = v.contains(mouse_pos);
            let interaction = if is_contained {
                if clicked {
                    Interaction::Clicked
                } else {
                    Interaction::Hover
                }
            } else {
                Interaction::None
            };
            v.render(window, interaction);
        })
    }
}

pub trait Widget {
    fn render(&self, window: &mut Window, interaction: Interaction);
    fn contains(&self, point: Vector) -> bool;
}

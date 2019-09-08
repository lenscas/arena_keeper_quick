use quicksilver::{
    geom::{Rectangle, Vector},
    lifecycle::Window,
};

use super::{context::Widget, Interaction};

#[derive(Clone)]
pub struct Combined<T, A>
where
    T: Widget,
    A: Widget,
{
    location: Rectangle,
    widget: T,
    background: A,
}
impl<T: Widget, A: Widget> Widget for Combined<T, A> {
    fn render(&self, window: &mut Window, at: &mut u32) {
        self.background.render(window, at);
        *at += 1;
        self.widget.render(window, at);
    }
    fn contains(&self, point: Vector) -> bool {
        self.background.contains(point) || self.widget.contains(point)
    }
    fn set_interaction(&mut self, interaction: Interaction) {
        self.background.set_interaction(interaction);
        self.widget.set_interaction(interaction)
    }
    fn set_pos(&mut self, pos: Rectangle) {
        let mut widget_rec = self.widget.get_pos().to_owned();
        widget_rec.pos = widget_rec.pos - self.location.pos + pos.pos;
        self.widget.set_pos(widget_rec);

        let mut background_rec = self.background.get_pos().to_owned();
        background_rec.pos = background_rec.pos - self.location.pos + pos.pos;
        self.background.set_pos(background_rec);
        self.location = pos;
    }
    fn get_pos(&self) -> &Rectangle {
        &self.location
    }
}
impl<T, A> Combined<T, A>
where
    T: Widget,
    A: Widget,
{
    pub fn new(mut widget: T, location: Rectangle, mut background: A) -> Self {
        let mut pos_widget = widget.get_pos().to_owned();
        pos_widget.pos += location.pos;
        widget.set_pos(pos_widget);
        let mut pos_background = background.get_pos().to_owned();
        pos_background.pos += location.pos;
        background.set_pos(pos_background);
        Self {
            location,
            widget,
            background,
        }
    }
}

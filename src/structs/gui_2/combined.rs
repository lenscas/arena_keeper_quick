use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, Surface},
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
        window.flush().unwrap();
        let surface =
            Surface::new(self.location.size.x as u32, self.location.size.y as u32).unwrap();
        surface
            .render_to(window, |v| {
                self.background.render(v, at);
                *at += 1;
                self.widget.render(v, at);
                Ok(())
            })
            .unwrap();
        *at += 1;
        window.draw_ex(
            &self.location,
            Img(surface.image()),
            Transform::IDENTITY,
            *at,
        );
    }
    fn contains(&self, point: Vector) -> bool {
        let new_point: Vector =
            (point.x - self.location.pos.x, point.y - self.location.pos.y).into();
        self.background.contains(new_point) || self.widget.contains(new_point)
    }
    fn set_interaction(&mut self, interaction: Interaction) {
        self.background.set_interaction(interaction);
        self.widget.set_interaction(interaction)
    }
}
impl<T, A> Combined<T, A>
where
    T: Widget,
    A: Widget,
{
    pub fn new(widget: T, location: Rectangle, background: A) -> Self {
        Self {
            location,
            widget,
            background,
        }
    }
}

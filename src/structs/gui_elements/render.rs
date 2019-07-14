use immi::DrawContext;
use immi::UiState;
use quicksilver::{
    graphics::{create_immi_ctx, Font, ImmiRender, ImmiStatus},
    lifecycle::{Asset, Window},
    Result,
};

type StyleFunc = Fn(&mut FnMut(DrawContext<ImmiRender>) -> (), DrawContext<ImmiRender>) -> ();

pub struct Render<A> {
    //to_render : Box<>,
    to_style: Box<StyleFunc>,
    asset: Asset<(Font, A)>,
}
impl<A> Render<A> {
    pub fn new(to_style: Box<StyleFunc>, asset: Asset<(Font, A)>) -> Self {
        Self { to_style, asset }
    }
}
impl<A> Render<A> {
    pub fn render<F>(
        &mut self,
        window: &mut Window,
        state: &mut UiState,
        mut to_render: F,
    ) -> Result<()>
    where
        F: FnMut(DrawContext<ImmiRender>, &mut UiState, &mut A) -> (),
    {
        let to_style = &self.to_style;
        self.asset.execute(|(font, button)| {
            let ui_status = ImmiStatus::new(window);
            let mut ui_render = ImmiRender::new_with_window(window, font);
            let mut render = |draw: DrawContext<ImmiRender>| to_render(draw, state, button);
            to_style(
                &mut Box::new(&mut render),
                create_immi_ctx(ui_status, &mut ui_render),
            );
            Ok(())
        })
    }
}

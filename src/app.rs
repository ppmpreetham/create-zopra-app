use gpui_kit::{Context, IntoElement, ParentElement, Render, Window, div};

pub struct App;

impl Render for App {
    // TODO: later make this use zopra
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Hello, GPUI!")
    }
}

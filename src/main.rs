mod app;
mod assets;
mod components;
mod config;

use assets::AppAssets;
use gpui_kit::component::{Root, TitleBar};
use gpui_kit::*;
use app::app;

pub struct Main;
impl Render for Main {
  fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    app(window, cx)
  }
}

fn main() {
  let app = gpui_kit::application().with_assets(AppAssets);
  let config = config::Config::new();

  app.run(move |cx| {
    gpui_kit::init(cx);

    let window_size = WindowOptions {
      window_bounds: Some(config.window_size),
      ..TitleBar::window_options()
    };

    cx.open_window(window_size, |window, cx| {
      let view = cx.new(|_| Main);
      cx.new(|cx| Root::new(view, window, cx))
    })
    .expect("failed to open window");
    cx.activate(true);
  });
}


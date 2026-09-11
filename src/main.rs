mod app;
mod assets;
mod components;
mod config;

use app::App;

use assets::AppAssets;
use gpui_kit::component::Root;
use gpui_kit::*;

fn main() {
  let app = gpui_kit::application().with_assets(AppAssets);
  let config = config::Config::new();

  app.run(move |cx| {
    gpui_kit::init(cx);

    let window_size = WindowOptions {
      window_bounds: Some(config.window_size),
      ..Default::default()
    };

    cx.open_window(window_size, |window, cx| {
      let view = cx.new(|_| App);
      cx.new(|cx| Root::new(view, window, cx))
    })
    .expect("failed to open window");
    cx.activate(true);
  });
}

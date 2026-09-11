mod app;
mod assets;

use app::App;

use assets::Assets;
use gpui_kit::component::Root;
use gpui_kit::*;

fn main() {
    let app = gpui_kit::application().with_assets(Assets);

    app.run(|cx| {
        gpui_kit::init(cx);

        let window_size = WindowOptions {
            window_bounds: Some(WindowBounds::Maximized(Bounds {
                origin: point(px(0.0), px(0.0)),
                size: size(px(1920.0), px(1080.0)),
            })),
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

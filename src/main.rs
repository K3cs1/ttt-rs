use slint::{Model, Weak};
use log::{info, warn};

slint::include_modules!();

slint::slint! {
    import { AppWindow } from "ui/appwindow.slint";
}

fn main() -> Result<(), slint::PlatformError> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new()?;
    let ui_handle: Weak<AppWindow> = ui.as_weak();

    let mut ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
    ttt_tiles.extend(ttt_tiles.clone());


    ui.on_computer_turn(move |id: i32| {
        info!("{}", id);

        let ui: AppWindow = ui_handle.unwrap();
        ui.set_computer_clicked(true);
    });

    ui.run()
}

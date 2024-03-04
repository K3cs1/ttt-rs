use std::rc::Rc;

use log::info;
use slint::{Model, VecModel, Weak};

//slint::include_modules!();

slint::slint! {
    import { AppWindow } from "ui/appwindow.slint";
}

fn main() {
    //-> Result<(), slint::PlatformError>
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new().unwrap();
    let ui_weak: Weak<AppWindow> = ui.as_weak();

    let mut ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
    ttt_tiles.extend(ttt_tiles.clone());

    let tiles_model = Rc::new(VecModel::from(ttt_tiles));

    ui.set_ttt_tiles(tiles_model.clone().into());

    ui.on_computer_turn(move |id: i32| {
        info!("{}", id);

        let ui: AppWindow = ui_weak.unwrap();

        let mut empty_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.empty);

        empty_tiles.for_each(|(_i, mut tile_data)| {
            if id == tile_data.id {
                tile_data.player_clicked = true;
                //tile_data.computer_clicked = true;
                tile_data.empty = false;
                tiles_model.set_row_data(_i, tile_data);
            }
        });

        // if let Some((tile_idx, mut tile_data)) =
        //     empty_tiles.next()
        // {
        //     if tile_data.id == id {
        //         tile_data.player_clicked = true;
        //         tile_data.empty = false;
        //         tiles_model.set_row_data(tile_idx, tile_data);
        //     }
        // }
    });

    ui.run().unwrap();
}

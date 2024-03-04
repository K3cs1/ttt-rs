use std::rc::Rc;

use log::info;
use slint::{Brush, Color, Model, VecModel, Weak};

const PLAYER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const COMPUTER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

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

        //let ui: AppWindow = ui_weak.unwrap();

        let mut empty_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.empty);

        //Player turn    
        empty_tiles.for_each(|(_i, mut tile_data)| {
            if id == tile_data.id {
                tile_data.player_clicked = true;
                tile_data.empty = false;
                //tile_data.win_color = COMPUTER_WIN_COLOR;
                tiles_model.set_row_data(_i, tile_data);
            }
        });

        //Computer turn
        let mut new_empty_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.empty);

        let mut free_ids = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.empty)
            .map(|(_, tile_data)| tile_data.id);

        let computer_id = free_ids.next().unwrap();
        new_empty_tiles.for_each(|(_i, mut tile_data)| {
            if computer_id == tile_data.id {
                tile_data.computer_clicked = true;
                tile_data.empty = false;
                //tile_data.win_color = COMPUTER_WIN_COLOR;
                tiles_model.set_row_data(_i, tile_data);            
            }
        });

    });

    ui.run().unwrap();
}

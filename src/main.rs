use log::{info, warn};
use slint::ComponentHandle;
use slint::{Brush, Color, Model, VecModel};
use std::rc::Rc;
use ttt_rs::{get_machine_win_combos, search_next_step};
use ttt_rs::AppWindow;
use ttt_rs::TileData;

const HUMAN_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const MACHINE_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

//slint::include_modules!();

// slint::slint! {
//     import { AppWindow } from "ui/appwindow.slint";
// }

fn main() {
    //-> Result<(), slint::PlatformError>
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new().unwrap();
    // let _ui_weak: Weak<AppWindow> = ui.as_weak();

    let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
    //ttt_tiles.extend(ttt_tiles.clone());

    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

    ui.set_ttt_tiles(tiles_model.clone().into());

    ui.on_process(move |id: i32| {
        info!("Selected id: {}", id);

        //let ui: AppWindow = ui_weak.unwrap();

        //Human turn
        tiles_model.iter().enumerate().for_each(|(_i, mut tile_data)| {
            if id == tile_data.id {
                tile_data.human_clicked = true;
                tile_data.empty = false;
                tiles_model.set_row_data(_i, tile_data);
            }
        });

        //Machine turn
        let machine_tiles = tiles_model.iter().enumerate();
        let founded_state_vec = search_next_step(&tiles_model);
        let machine_next_tile = founded_state_vec.get(founded_state_vec.len() - 1);
        match machine_next_tile {
            Some(mn_tile) => {
                machine_tiles.for_each(|(_i, mut tile_data)| {
                    if tile_data.id == mn_tile.field_id {
                        tile_data.machine_clicked = true;
                        tile_data.empty = false;
                        tiles_model.set_row_data(_i, tile_data);
                    }
                });
            }
            None => {
                warn!("Machine next tile not found!");
            }
        }

        let win_combo = get_machine_win_combos(&tiles_model);
        if !win_combo.is_empty() {
            tiles_model.iter().enumerate().for_each(|(_i, mut tile_data)| {
                if win_combo.contains(&tile_data.id) && tile_data.machine_clicked == true {
                    tile_data.win_color = MACHINE_WIN_COLOR;
                    tiles_model.set_row_data(_i, tile_data);
                }
            });
        }
    });

    ui.run().unwrap();
}

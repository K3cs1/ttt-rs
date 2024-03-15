use log::{info, warn};
use slint::{Brush, Color, ComponentHandle};
use slint::{Model, ModelNotify, VecModel};
use std::process::exit;
use std::rc::Rc;
use ttt_rs::TileData;
use ttt_rs::{has_winner, search_next_step};
use ttt_rs::{random_machine_start, AppWindow};

const DEFAULT_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(255, 255, 0));

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new().unwrap();

    let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();

    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

    random_machine_start(&tiles_model);

    ui.set_ttt_tiles(tiles_model.clone().into());

    ui.on_process(move |id: i32| {
        info!("Selected id: {}", id);

        if has_winner(&tiles_model) {
            return;
        }

        //Human turn
        for (_i, mut tile_data) in tiles_model.iter().enumerate() {
            if id == tile_data.id {
                tile_data.human_clicked = true;
                tile_data.empty = false;
                tiles_model.set_row_data(_i, tile_data);
                break;
            }
        }

        if has_winner(&tiles_model) {
            return;
        }

        //Machine turn
        let machine_tiles = tiles_model.iter().enumerate();
        let founded_state_vec = search_next_step(&tiles_model);
        if !founded_state_vec.is_empty() {
            let machine_next_tile = founded_state_vec.get(founded_state_vec.len() - 1);
            match machine_next_tile {
                Some(mn_tile) => {
                    for (_i, mut tile_data) in machine_tiles {
                        if tile_data.id == mn_tile.field_id {
                            tile_data.machine_clicked = true;
                            tile_data.empty = false;
                            tiles_model.set_row_data(_i, tile_data);
                            break;
                        }
                    }
                }
                None => {
                    warn!("Machine next tile not found!");
                }
            }
        }

        if has_winner(&tiles_model) {
            return;
        }
    });

    ui.on_exit(move || {
        exit(0);
    });

    let ui_weak = ui.as_weak();
    ui.on_restart_game(move || {
        info!("Restart the game");
        let ui = ui_weak.unwrap();

        let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();

        let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

        for (_i, mut tile_data) in tiles_model.iter().enumerate() {
            tile_data.machine_clicked = false;
            tile_data.human_clicked = false;
            tile_data.empty = true;
            tile_data.win_color = DEFAULT_COLOR;
            tiles_model.set_row_data(_i, tile_data);
        }

        random_machine_start(&tiles_model);

        ui.set_ttt_tiles(tiles_model.clone().into());

        let model_notifiy = ModelNotify::default();
        model_notifiy.reset();
    });

    ui.run().unwrap();
}

use log::{info, warn};
use rand::Rng;
use slint::ComponentHandle;
use slint::{Model, VecModel};
use std::rc::Rc;
use ttt_rs::{has_winner, search_next_step};
use ttt_rs::AppWindow;
use ttt_rs::TileData;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new().unwrap();

    let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();

    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

    // Where does Machine start the game? Middle or top LHS ?
    let mut rng = rand::thread_rng();
    let middle_or_top_right = rng.gen_range(0..2);
    if middle_or_top_right == 1 {
        for (_i, mut tile_data) in tiles_model.iter().enumerate() {
            if tile_data.id == 4 {
                tile_data.machine_clicked = true;
                tile_data.empty = false;
                tiles_model.set_row_data(_i, tile_data);
                break;
            }
        }
    } else {
        for (_i, mut tile_data) in tiles_model.iter().enumerate() {
            if tile_data.id == 0 {
                tile_data.machine_clicked = true;
                tile_data.empty = false;
                tiles_model.set_row_data(_i, tile_data);
                break;
            }
        }
    }

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
    });

    ui.run().unwrap();
}

mod game_logic;
mod win_graph;
use game_logic::{AppWindow, GameLogic, Sequence, Tile, TileData};
use log::{info, trace};
use slint::{Brush, Color, ComponentHandle, SharedString};
use slint::{Model, ModelNotify, VecModel};
use std::process::exit;
use std::rc::Rc;

const DEFAULT_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(255, 255, 0));

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let ui = AppWindow::new().unwrap();

    let ui_weak = ui.as_weak();

    let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();

    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

    let sequence: Vec<Sequence> = ui.get_sequence().iter().collect();
    let sequence_model: Rc<VecModel<Sequence>> = Rc::new(VecModel::from(sequence));

    GameLogic::random_machine_start(&tiles_model, &sequence_model);

    ui.set_ttt_tiles(tiles_model.clone().into());

    ui.set_sequence(sequence_model.clone().into());

    ui.on_process(move |id: i32| {
        trace!("Selected id: {}", id);

        let ui = ui_weak.unwrap();

        let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();

        let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

        if GameLogic::has_winner(&tiles_model) {
            trace!("Has winner!");
        } else {
            //Human turn
            for (_i, mut tile_data) in tiles_model.iter().enumerate() {
                if id == tile_data.id {
                    tile_data.human_clicked = true;
                    tile_data.empty = false;
                    tiles_model.set_row_data(_i, tile_data);
                    sequence_model.push(Sequence {
                        id: id,
                        player: SharedString::from("H"),
                    });
                    break;
                }
            }
            ui.set_ttt_tiles(tiles_model.clone().into());

            if GameLogic::has_winner(&tiles_model) {
                trace!("Has winner!");
            } else {
                //Machine turn
                let mut founded_state_vec: Vec<Tile> =
                    GameLogic::search_next_step(&tiles_model, &sequence_model);
                info!("founded_state_vec: {:?}", founded_state_vec);
                ui.set_sequence(sequence_model.clone().into());
                if !founded_state_vec.is_empty() {
                    let machine_next_tile: Option<Tile> = founded_state_vec.pop();
                    match machine_next_tile {
                        Some(mn_tile) => {
                            for (_i, mut tile_data) in tiles_model.iter().enumerate() {
                                if tile_data.id == mn_tile.field_id && tile_data.empty == true {
                                    tile_data.machine_clicked = true;
                                    tile_data.empty = false;
                                    tiles_model.set_row_data(_i, tile_data);
                                    info!("Machine step id: {:?}", mn_tile.field_id);
                                    sequence_model.push(Sequence {
                                        id: mn_tile.field_id,
                                        player: SharedString::from("M"),
                                    });
                                    break;
                                }
                            }
                            info!(
                                "Size of sequence_model in main: {:?}",
                                sequence_model.iter().count()
                            );
                            ui.set_ttt_tiles(tiles_model.clone().into());
                        }
                        None => {
                            info!("Machine next tile not found!");
                        }
                    }
                }

                if GameLogic::has_winner(&tiles_model) {
                    trace!("Has winner!");
                }
            }
        }

        ui.set_sequence(sequence_model.clone().into());
        ui.set_ttt_tiles(tiles_model.clone().into());
    });

    ui.on_exit(move || {
        exit(0);
    });

    let ui_weak = ui.as_weak();

    ui.on_restart_game(move || {
        trace!("Restart the game");
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

        let mut sequence: Vec<Sequence> = ui.get_sequence().iter().collect();

        sequence.clear();

        let sequence_model: Rc<VecModel<Sequence>> = Rc::new(VecModel::from(sequence));

        GameLogic::random_machine_start(&tiles_model, &sequence_model);

        info!(
            "Size of sequence_model in restart: {:?}",
            sequence_model.iter().count()
        );
        ui.set_sequence(sequence_model.into());

        ui.set_ttt_tiles(tiles_model.clone().into());

        let model_notifiy = ModelNotify::default();
        model_notifiy.reset();
    });

    ui.run().unwrap();
}

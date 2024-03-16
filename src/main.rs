use log::{info, warn};
use slint::{Brush, Color, ComponentHandle, SharedString};
use slint::{Model, ModelNotify, VecModel};
use std::process::exit;
use std::rc::Rc;
use ttt_rs::{has_winner, search_next_step};
use ttt_rs::{random_machine_start, AppWindow};
use ttt_rs::{Sequence, TileData};

const DEFAULT_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(255, 255, 0));

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new().unwrap();

    let ui_weak = ui.as_weak();

    let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();

    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

    let sequence: Vec<Sequence> = ui.get_sequence().iter().collect();
    let sequence_model: Rc<VecModel<Sequence>> = Rc::new(VecModel::from(sequence));

    random_machine_start(&tiles_model, &sequence_model);

    ui.set_ttt_tiles(tiles_model.clone().into());

    ui.set_sequence(sequence_model.clone().into());

    ui.on_process(move |id: i32| {
        info!("Selected id: {}", id);

        let ui = ui_weak.unwrap();

        let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();

        let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

        if has_winner(&tiles_model) {
            info!("Has winner!");
        } else {
            let sequence: Vec<Sequence> = ui.get_sequence().iter().collect();
            let sequence_model: Rc<VecModel<Sequence>> = Rc::new(VecModel::from(sequence));

            //Human turn
            for (_i, mut tile_data) in tiles_model.iter().enumerate() {
                if id == tile_data.id {
                    tile_data.human_clicked = true;
                    tile_data.empty = false;
                    tiles_model.set_row_data(_i, tile_data);
                    sequence_model.insert(
                        sequence_model.row_count(),
                        Sequence {
                            id: id,
                            player: SharedString::from("H"),
                        },
                    );
                    break;
                }
            }
            ui.set_sequence(sequence_model.clone().into());

            if has_winner(&tiles_model) {
                info!("Has winner!");
            } else {
                //Machine turn
                let founded_state_vec = search_next_step(&tiles_model, &sequence_model);
                ui.set_sequence(sequence_model.clone().into());
                if !founded_state_vec.is_empty() {
                    let machine_next_tile = founded_state_vec.get(founded_state_vec.len() - 1);
                    let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
                    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));
                    match machine_next_tile {
                        Some(mn_tile) => {
                            for (_i, mut tile_data) in tiles_model.iter().enumerate() {
                                if tile_data.id == mn_tile.field_id && tile_data.empty == true {
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
                    info!("Has winner!");
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

        ui.set_sequence(Rc::new(VecModel::from(Vec::new())).into());

        let sequence: Vec<Sequence> = ui.get_sequence().iter().collect();

        let sequence_model: Rc<VecModel<Sequence>> = Rc::new(VecModel::from(sequence));

        random_machine_start(&tiles_model, &sequence_model);

        ui.set_sequence(sequence_model.clone().into());

        ui.set_ttt_tiles(tiles_model.clone().into());

        let model_notifiy = ModelNotify::default();
        model_notifiy.reset();
    });

    ui.run().unwrap();
}

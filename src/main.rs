mod game_logic;
mod win_graph;

use game_logic::{AppWindow, GameLogic, Sequence, TileData};
use log::{info, trace};
use slint::{Brush, Color, ComponentHandle, SharedString};
use slint::{Model, ModelNotify, VecModel};
use std::panic;
use std::process::exit;
use std::rc::Rc;

const DEFAULT_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(255, 255, 0));

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    // So any panic prints a backtrace in the JS console (when using wasm).
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Initialize logger, so trace/info calls appear in debug console
    env_logger::init();

    // Create the main window from Slint
    let ui = AppWindow::new().unwrap();
    let ui_weak = ui.as_weak();

    // Fetch the current tile and sequence data from the UI (Slint).
    // Because game_logic::TileData == slint_generatedAppWindow::TileData now,
    // this collects them into the same type we use in game_logic.
    let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
    let tiles_model = Rc::new(VecModel::from(ttt_tiles));

    let sequence: Vec<Sequence> = ui.get_sequence().iter().collect();
    let sequence_model = Rc::new(VecModel::from(sequence));

    // Machine starts the game with a random move
    GameLogic::random_machine_start(&tiles_model, &sequence_model);

    // Push the updated models back to the UI
    ui.set_ttt_tiles(tiles_model.clone().into());
    ui.set_sequence(sequence_model.clone().into());

    // Handle clicks on the board
    ui.on_process(move |id: i32| {
        trace!("Selected id: {}", id);
        let ui = ui_weak.unwrap();

        // Reload current tile states from UI
        let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
        let tiles_model = Rc::new(VecModel::from(ttt_tiles));

        if GameLogic::has_winner(&tiles_model) {
            trace!("We already have a winner, ignoring human move");
        } else {
            // Human move
            for (index, mut tile_data) in tiles_model.iter().enumerate() {
                if id == tile_data.id {
                    tile_data.human_clicked = true;
                    tile_data.empty = false;
                    tiles_model.set_row_data(index, tile_data);

                    // Add move to sequence
                    sequence_model.push(Sequence {
                        id,
                        player: SharedString::from("H"),
                    });
                    break;
                }
            }
            ui.set_ttt_tiles(tiles_model.clone().into());

            // Check if Human just won
            if GameLogic::has_winner(&tiles_model) {
                trace!("Human wins!");
            } else {
                // Machine's turn
                let mut founded_state_vec =
                    GameLogic::search_next_step(&tiles_model, &sequence_model);
                info!("Machine next-step path: {:?}", founded_state_vec);

                // Sync the sequence model to UI before we do the actual machine move
                ui.set_sequence(sequence_model.clone().into());

                if !founded_state_vec.is_empty() {
                    // The machine's next move is the last tile in the path
                    let machine_next_tile = founded_state_vec.pop();
                    if let Some(mn_tile) = machine_next_tile {
                        // Apply the machine move to the board
                        for (index, mut tile_data) in tiles_model.iter().enumerate() {
                            if tile_data.id == mn_tile.field_id && tile_data.empty {
                                tile_data.machine_clicked = true;
                                tile_data.empty = false;
                                tiles_model.set_row_data(index, tile_data);
                                info!("Machine moves on tile id: {:?}", mn_tile.field_id);

                                // Add move to sequence model
                                sequence_model.push(Sequence {
                                    id: mn_tile.field_id,
                                    player: SharedString::from("M"),
                                });
                                break;
                            }
                        }
                        ui.set_ttt_tiles(tiles_model.clone().into());
                    } else {
                        info!("Machine's next move not found!");
                    }
                }

                // Check if Machine just won
                if GameLogic::has_winner(&tiles_model) {
                    trace!("Machine wins!");
                }
            }
        }

        // Update UI with final models
        ui.set_sequence(sequence_model.clone().into());
        ui.set_ttt_tiles(tiles_model.clone().into());
    });

    // Handle exit
    ui.on_exit(move || {
        exit(0);
    });

    // Handle "restart game" button
    let ui_weak = ui.as_weak();
    ui.on_restart_game(move || {
        trace!("Restarting the game");
        let ui = ui_weak.unwrap();

        // Re-fetch the tile data from UI
        let ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
        let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

        // Reset each tile
        for (index, mut tile_data) in tiles_model.iter().enumerate() {
            tile_data.machine_clicked = false;
            tile_data.human_clicked = false;
            tile_data.empty = true;
            tile_data.win_color = DEFAULT_COLOR;
            tiles_model.set_row_data(index, tile_data);
        }

        // Clear the sequence
        let mut sequence: Vec<_> = ui.get_sequence().iter().collect();
        sequence.clear();
        let sequence_model = Rc::new(VecModel::from(sequence));

        // Have the Machine make its first move again
        GameLogic::random_machine_start(&tiles_model, &sequence_model);

        info!(
            "Sequence model size after restart: {}",
            sequence_model.row_count()
        );

        // Write back to UI
        ui.set_sequence(sequence_model.into());
        ui.set_ttt_tiles(tiles_model.clone().into());

        // Force UI to refresh
        let model_notify = ModelNotify::default();
        model_notify.reset();
    });

    // Show the UI
    ui.run().unwrap();
}

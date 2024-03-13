use log::{info, trace, warn};
use slint::{Brush, Color, Model, VecModel, Weak};
use ttt_rs::search_next_step;
use ttt_rs::AppWindow;
use ttt_rs::TileData;
use std::rc::Rc;
use slint::ComponentHandle;

const PLAYER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const COMPUTER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

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

    let mut ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
    //ttt_tiles.extend(ttt_tiles.clone());

    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

    ui.set_ttt_tiles(tiles_model.clone().into());

    ui.on_process(move |id: i32| {
        info!("Selected id: {}", id);

        //let ui: AppWindow = ui_weak.unwrap();
        //let human_win_combo = has_human_win_combo(tiles_model.clone());
        //let machine_win_combo = has_machine_win_combo(tiles_model.clone());

        let empty_tiles = tiles_model.iter().enumerate();
        //.filter(|(_, tile)| tile.empty);

        //Human turn
        empty_tiles.for_each(|(_i, mut tile_data)| {
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

        //TODO Is there a winner?
        

        // let mut new_empty_tiles = tiles_model
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, tile)| tile.empty);

        // let mut free_ids = tiles_model
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, tile)| tile.empty)
        //     .map(|(_, tile_data)| tile_data.id);

        // let computer_id = free_ids.next().unwrap();
        // new_empty_tiles.for_each(|(_i, mut tile_data)| {
        //     if computer_id == tile_data.id {
        //         tile_data.machine_clicked = true;
        //         tile_data.empty = false;
        //         //tile_data.win_color = COMPUTER_WIN_COLOR;
        //         tiles_model.set_row_data(_i, tile_data);
        //     }
        // });
    });

    ui.run().unwrap();
}

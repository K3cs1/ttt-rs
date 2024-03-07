use log::info;
use slint::{Brush, Color, Model, VecModel, Weak};
use std::collections::LinkedList;
use std::{cell::RefCell, rc::Rc};

const PLAYER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const COMPUTER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

//slint::include_modules!();

slint::slint! {
    import { AppWindow } from "ui/appwindow.slint";
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Player {
    Machine,
    Human,
    Nobody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    field_id: i32,
    player: Player,
}

impl Tile {
    fn init() -> Self {
        Self {
            field_id: -1,
            player: Player::Nobody,
        }
    }

    fn new(field_id: i32, player: Player) -> Self {
        Self { field_id, player }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WinTree<T> {
    value: T,
    left: Option<Box<WinTree<T>>>,
    right: Option<Box<WinTree<T>>>,
}

impl<T> WinTree<T> {
    pub fn new(value: T) -> Self {
        WinTree {
            value,
            left: None,
            right: None,
        }
    }
    pub fn left(mut self, node: WinTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: WinTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

fn init_win_tree() -> WinTree<Vec<Tile>> {
    let mut win_tree = WinTree::new(vec![Tile::init()])
        .left(WinTree::new(vec![Tile::new(4, Player::Machine)]))
        .left(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
        ]))
        .left(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
        ]))
        .right(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
        ]))
        .right(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(1, Player::Machine),
        ]))
        .right(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(1, Player::Machine),
            Tile::new(8, Player::Human),
        ]))
        .right(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(1, Player::Machine),
            Tile::new(8, Player::Human),
            Tile::new(3, Player::Machine),
        ]))
        .left(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(0, Player::Human),
        ]))
        .left(WinTree::new(vec![
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
        ]));

    win_tree
}

fn machine_next_step(tiles_model: Rc<VecModel<TileData>>) -> WinTree<Vec<Tile>> {
    let mut queue: LinkedList<WinTree<Vec<Tile>>> = LinkedList::new();

    let actual_state: WinTree<Vec<Tile>> = build_state_from_model(tiles_model);

    let the_win_tree: WinTree<Vec<Tile>> = init_win_tree();

    let mut current_node: WinTree<Vec<Tile>>;

    let mut result: WinTree<Vec<Tile>>;

    let mut visited: Vec<bool> = vec![];

    loop {
        if !queue.is_empty() {
            current_node = queue.pop_front().unwrap();
            let current_values: Vec<Tile> = current_node.value;
            let the_win_tree_values = the_win_tree.value;

            for tree_val in the_win_tree_values {
                if actual_state.value.contains(&tree_val) {
                    return current_node;
                } else {
                    if current_node.left.is_some() {
                        queue.push_back(*current_node.left.unwrap())
                    }
                    
                }
            }

            // for tile_step in current_value {
            //     if let Some((tile_idx, mut tile)) = all_tiles.next() {
            //         let title_field_id = usize::try_from(tile_step.field_id).unwrap();
            //         if (tile_step.field_id == 2 && title_field_id == tile_idx)
            //             || (tile_step.field_id == 4 && title_field_id == tile_idx)
            //             || (tile_step.field_id == 6 && title_field_id == tile_idx)
            //         {
            //             match tile_step.player {
            //                 Player::Machine => {
            //                     if tile.computer_clicked == true {
            //                         machine_win_combo.push(tile_step.field_id);
            //                     }
            //                 }
            //                 _ => {}
            //             }
            //         }
            //     };

            // }
            // if machine_win_combo.contains(&2)
            //     && machine_win_combo.contains(&4)
            //     && machine_win_combo.contains(&6)
            // {
            //     //result = machine_win_combo;
            // }
        }
    }
}

fn build_state_from_model(tiles_model: Rc<VecModel<TileData>>) -> WinTree<Vec<Tile>> {
    let mut all_tiles = tiles_model.iter().enumerate();

    let mut states: Vec<Tile> = vec![];
    if let Some((tile_idx, mut tile_data)) = all_tiles.next() {
        //let title_field_id = usize::try_from(tile_data.id).unwrap();
        if tile_data.player_clicked == true {
            states.push(Tile::new(tile_data.id, Player::Human));
        } else {
            states.push(Tile::new(tile_data.id, Player::Machine));
        }
    }
    WinTree::new(states)
}

fn has_human_win_combo(tiles_model: Rc<VecModel<TileData>>) -> WinTree<Vec<Tile>> {
    todo!()
}

fn main() {
    //-> Result<(), slint::PlatformError>
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new().unwrap();
    let ui_weak: Weak<AppWindow> = ui.as_weak();

    let mut ttt_tiles: Vec<TileData> = ui.get_ttt_tiles().iter().collect();
    ttt_tiles.extend(ttt_tiles.clone());

    let tiles_model: Rc<VecModel<TileData>> = Rc::new(VecModel::from(ttt_tiles));

    ui.set_ttt_tiles(tiles_model.clone().into());

    let win_tree: WinTree<Vec<Tile>> = init_win_tree();

    ui.on_process(move |id: i32| {
        info!("{}", id);

        //let ui: AppWindow = ui_weak.unwrap();

        let human_win_combo = has_human_win_combo(tiles_model.clone());

        let machine_win_combo = has_machine_win_combo(tiles_model.clone());

        let mut empty_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.empty);

        //Human turn
        empty_tiles.for_each(|(_i, mut tile_data)| {
            if id == tile_data.id {
                tile_data.player_clicked = true;
                tile_data.empty = false;
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

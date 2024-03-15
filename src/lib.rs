use log::{info, warn};
use petgraph::csr::IndexType;
use petgraph::graph::Neighbors;
use petgraph::prelude::NodeIndex;
use petgraph::visit::Bfs;
use rand::prelude::ThreadRng;
use rand::Rng;
use slint::{Brush, Color, Model, VecModel};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;
use wingraph::WinGraph;
mod wingraph;

const HUMAN_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const MACHINE_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

slint::include_modules!();

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Player {
    Machine,
    Human,
    Nobody,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Tile {
    pub field_id: i32,
    player: Player,
}

impl Tile {
    pub fn init() -> Self {
        Self {
            field_id: -1,
            player: Player::Nobody,
        }
    }

    fn new(field_id: i32, player: Player) -> Self {
        Self { field_id, player }
    }
}

pub fn search_next_step(tiles_model: &Rc<VecModel<TileData>>) -> Vec<Tile> {
    let actual_state: Vec<Tile> = build_steps_from_model(&tiles_model);
    let steps_map: HashMap<&str, Vec<Tile>> = WinGraph::init_steps_map();
    let graph = WinGraph::build_graph();
    let mut founded_key: Option<&str> = None;
    let mut next_state: Option<&str> = None;
    let mut rng: ThreadRng = rand::thread_rng();

    for entry in steps_map.clone() {
        if vec_tile_compare(&entry.1, &actual_state) {
            founded_key = Some(entry.0);
            info!("Founded {:?}", founded_key);
            break;
        }
    }

    for start in graph.node_indices() {
        let mut bfs = Bfs::new(&graph, start);

        while let Some(nx) = bfs.next(&graph) {
            match founded_key {
                Some(key) => {
                    if graph[nx].eq(key) {
                        let neighbours: Neighbors<'_, &str> = graph.neighbors(nx);

                        let count: usize = neighbours.clone().count();
                        info!("Count: {}", count);
                        let mut neighbour_index: usize = 0;
                        if count >= 1 {
                            neighbour_index = rng.gen_range(0..count + 1);
                        }
                        info!("Random neighbour_index: {:?}", neighbour_index);

                        neighbours.for_each(|nb_node_index: NodeIndex| {
                            info!("next_node_index: {:?}", neighbour_index.index());
                            next_state = Some(graph[nb_node_index]);
                        });
                        break;
                    }
                }
                None => (),
            }
        }
    }

    match next_state {
        Some(key) => {
            info!("Next state key: {:?}", key);
            let hash_map = steps_map.clone();
            let result = &hash_map.get(key);
            return result.unwrap().to_owned();
        }
        None => {
            warn!("State not found, machine will random step");
            let empty_tile_ids = tiles_model
                .iter()
                .enumerate()
                .filter(|(_, tile)| tile.empty == true)
                .map(|(_, tile_data)| tile_data.id)
                .collect::<Vec<i32>>();
            info!("empty_tile_ids: {:?}", empty_tile_ids);

            if empty_tile_ids.is_empty() {
                return Vec::new();
            }
            let rnd_tile_idx = rng.gen_range(0..empty_tile_ids.len());
            info!("rnd_tile_idx: {:?}", rnd_tile_idx);
            let rnd_tile_id = empty_tile_ids.get(rnd_tile_idx).unwrap();
            info!("rnd_tile_id: {:?}", rnd_tile_id);

            for (_i, mut tile_data) in tiles_model.iter().enumerate() {
                if tile_data.id == *rnd_tile_id && tile_data.empty == true {
                    tile_data.machine_clicked = true;
                    tile_data.empty = false;
                    tiles_model.set_row_data(_i, tile_data);
                    break;
                }
            }
            let next_step = build_steps_from_model(&tiles_model);
            for entry in steps_map.clone() {
                if vec_tile_compare(&entry.1, &next_step) {
                    info!("Founded {:?}", &entry.0);
                    return steps_map.get(&entry.0).unwrap().to_vec();
                }
            }
            Vec::new()
        }
    }
}

fn build_steps_from_model(tiles_model: &Rc<VecModel<TileData>>) -> Vec<Tile> {
    let mut steps: Vec<Tile> = Vec::new();
    for (_i, tile_data) in tiles_model.iter().enumerate() {
        if tile_data.human_clicked == true && tile_data.empty == false {
            steps.push(Tile::new(tile_data.id, Player::Human));
        } else if tile_data.machine_clicked == true && tile_data.empty == false {
            steps.push(Tile::new(tile_data.id, Player::Machine));
        }
    }
    info!("build_steps_from_model steps: {:?}", &steps);
    steps
}

fn vec_tile_compare(vector_a: &Vec<Tile>, vector_b: &Vec<Tile>) -> bool {
    if vector_a.len() == vector_b.len() {
        let a_set: HashSet<_> = vector_a.iter().copied().collect();
        return vector_b.iter().all(|item| a_set.contains(item));
    }
    false
}

pub fn get_win_combos(tiles_model: &Rc<VecModel<TileData>>, player: Player) -> Vec<i32> {
    let mut counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 0 && tile.machine_clicked == true)
                        || (tile.id == 1 && tile.machine_clicked == true)
                        || (tile.id == 2 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 0 && tile.human_clicked == true)
                        || (tile.id == 1 && tile.human_clicked == true)
                        || (tile.id == 2 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![0, 1, 2];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 3 && tile.machine_clicked == true)
                        || (tile.id == 4 && tile.machine_clicked == true)
                        || (tile.id == 5 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 3 && tile.human_clicked == true)
                        || (tile.id == 4 && tile.human_clicked == true)
                        || (tile.id == 5 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![3, 4, 5];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 6 && tile.machine_clicked == true)
                        || (tile.id == 7 && tile.machine_clicked == true)
                        || (tile.id == 8 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 6 && tile.human_clicked == true)
                        || (tile.id == 7 && tile.human_clicked == true)
                        || (tile.id == 8 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![6, 7, 8];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 0 && tile.machine_clicked == true)
                        || (tile.id == 3 && tile.machine_clicked == true)
                        || (tile.id == 6 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 0 && tile.human_clicked == true)
                        || (tile.id == 3 && tile.human_clicked == true)
                        || (tile.id == 6 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![0, 3, 6];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 1 && tile.machine_clicked == true)
                        || (tile.id == 4 && tile.machine_clicked == true)
                        || (tile.id == 7 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 1 && tile.human_clicked == true)
                        || (tile.id == 4 && tile.human_clicked == true)
                        || (tile.id == 7 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![1, 4, 7];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 2 && tile.machine_clicked == true)
                        || (tile.id == 5 && tile.machine_clicked == true)
                        || (tile.id == 8 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 2 && tile.human_clicked == true)
                        || (tile.id == 5 && tile.human_clicked == true)
                        || (tile.id == 8 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![2, 5, 8];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 0 && tile.machine_clicked == true)
                        || (tile.id == 4 && tile.machine_clicked == true)
                        || (tile.id == 8 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 0 && tile.human_clicked == true)
                        || (tile.id == 4 && tile.human_clicked == true)
                        || (tile.id == 8 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![0, 4, 8];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| match player {
            Player::Machine => {
                if tile.empty == false
                    && ((tile.id == 2 && tile.machine_clicked == true)
                        || (tile.id == 4 && tile.machine_clicked == true)
                        || (tile.id == 6 && tile.machine_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Human => {
                if tile.empty == false
                    && ((tile.id == 2 && tile.human_clicked == true)
                        || (tile.id == 4 && tile.human_clicked == true)
                        || (tile.id == 6 && tile.human_clicked == true))
                {
                    return true;
                }
                false
            }
            Player::Nobody => false,
        })
        .count();

    if counter == 3 {
        return vec![2, 4, 6];
    }

    Vec::new()
}

pub fn has_winner(tiles_model: &Rc<VecModel<TileData>>) -> bool {
    let win_combo = get_win_combos(&tiles_model, Player::Machine);
    if !win_combo.is_empty() {
        tiles_model
            .iter()
            .enumerate()
            .for_each(|(_i, mut tile_data)| {
                if win_combo.contains(&tile_data.id)
                    && tile_data.machine_clicked == true
                    && tile_data.empty == false
                {
                    tile_data.win_color = MACHINE_WIN_COLOR;
                    tiles_model.set_row_data(_i, tile_data);
                }
            });
        return true;
    } else {
        let win_combo = get_win_combos(&tiles_model, Player::Human);
        if !win_combo.is_empty() {
            tiles_model
                .iter()
                .enumerate()
                .for_each(|(_i, mut tile_data)| {
                    if win_combo.contains(&tile_data.id)
                        && tile_data.human_clicked == true
                        && tile_data.empty == false
                    {
                        tile_data.win_color = HUMAN_WIN_COLOR;
                        tiles_model.set_row_data(_i, tile_data);
                    }
                });
            return true;
        }
    }
    false
}

pub fn random_machine_start(tiles_model: &Rc<VecModel<TileData>>) {
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
}

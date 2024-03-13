use log::{info, warn};
use petgraph::csr::IndexType;
use petgraph::visit::Bfs;
use rand::Rng;
use slint::{Model, VecModel};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;
use wingraph::WinGraph;
mod wingraph;

slint::include_modules!();
// slint::slint! {
//     import { AppWindow } from "ui/appwindow.slint";
// }

// slint::slint! {
//     import { AppWindow } from "ui/appwindow.slint";
// }

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Player {
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
    let mut next_state = None;

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
                        let neighbours = graph.neighbors(nx);
                        let mut rng = rand::thread_rng();
                        let count: usize = neighbours.clone().count();
                        info!("Count: {}", count);
                        let mut neighbour_index: usize = 0;
                        if count >= 1 {
                            neighbour_index = rng.gen_range(0..count);
                        }
                        info!("Random neighbour_index: {:?}", neighbour_index);

                        neighbours.for_each(|nb_node_index| {
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

    let mut next_state_vec = Vec::new();
    match next_state {
        Some(key) => {
            info!("Next state key: {:?}", key);
            let hash_map = steps_map.clone();
            let result = &hash_map.get(key);
            return result.unwrap().to_owned();
        }
        None => {
            warn!("State not found");
            return next_state_vec;
        }
    }
}

fn build_steps_from_model(tiles_model: &Rc<VecModel<TileData>>) -> Vec<Tile> {
    let mut steps: Vec<Tile> = Vec::new();
    tiles_model.iter().enumerate().for_each(|(_i, tile_data)| {
        if tile_data.human_clicked == true {
            steps.push(Tile::new(tile_data.id, Player::Human));
        }
        if tile_data.machine_clicked == true {
            steps.push(Tile::new(tile_data.id, Player::Machine));
        }
    });

    steps
}

fn vec_tile_compare(vector_a: &Vec<Tile>, vector_b: &Vec<Tile>) -> bool {
    if vector_a.len() == vector_b.len() {
        let a_set: HashSet<_> = vector_a.iter().copied().collect();
        return vector_b.iter().all(|item| a_set.contains(item));
    }
    false
}

pub fn get_machine_win_combos(tiles_model: &Rc<VecModel<TileData>>) -> Vec<i32> {
    let mut counter = 0;
    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 0 && tile.machine_clicked == true)
                || (tile.id == 1 && tile.machine_clicked == true)
                || (tile.id == 2 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![0,1,2];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 3 && tile.machine_clicked == true)
                || (tile.id == 4 && tile.machine_clicked == true)
                || (tile.id == 5 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![3,4,5];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 6 && tile.machine_clicked == true)
                || (tile.id == 7 && tile.machine_clicked == true)
                || (tile.id == 8 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![6,7,8];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 0 && tile.machine_clicked == true)
                || (tile.id == 3 && tile.machine_clicked == true)
                || (tile.id == 6 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![0,3,6];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 1 && tile.machine_clicked == true)
                || (tile.id == 4 && tile.machine_clicked == true)
                || (tile.id == 7 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![1,4,7];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 2 && tile.machine_clicked == true)
                || (tile.id == 5 && tile.machine_clicked == true)
                || (tile.id == 8 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![2,5,8];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 0 && tile.machine_clicked == true)
                || (tile.id == 4 && tile.machine_clicked == true)
                || (tile.id == 8 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![0,4,8];
    }

    counter = tiles_model
        .iter()
        .filter(|tile| {
            if (tile.id == 2 && tile.machine_clicked == true)
                || (tile.id == 4 && tile.machine_clicked == true)
                || (tile.id == 6 && tile.machine_clicked == true)
            {
                return true;
            }
            false
        })
        .count();

    if counter == 3 {
        return vec![2,4,6];
    }

    Vec::new()
}

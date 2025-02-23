use crate::win_graph;
use log::trace;
use petgraph::prelude::NodeIndex;
use petgraph::visit::Bfs;
use rand::prelude::ThreadRng;
use rand::Rng;
use slint::{Brush, Color, Model, SharedString, VecModel};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

slint::include_modules!();

const HUMAN_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const MACHINE_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

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

    pub fn new(field_id: i32, player: Player) -> Self {
        Self { field_id, player }
    }
}

pub struct GameLogic;

// A constant list of winning tile combinations.
const WIN_COMBINATIONS: [[i32; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl GameLogic {
    /// Computes the next step for the machine.
    pub fn search_next_step(
        tiles_model: &Rc<VecModel<TileData>>,
        sequence_model: &Rc<VecModel<Sequence>>,
    ) -> Vec<Tile> {
        trace!("Sequence model size: {:?}", sequence_model.iter().count());

        let mut actual_state = Self::build_steps_from_model(sequence_model);
        let steps_map: HashMap<&str, Vec<Tile>> = win_graph::WinGraph::init_steps_map();
        let graph = win_graph::WinGraph::build_graph();
        let mut founded_key: Option<&str> = None;
        let mut next_state: Option<&str> = None;
        let mut rng: ThreadRng = rand::thread_rng();

        // Identify the current state in the steps map.
        for (key, tiles) in &steps_map {
            if Self::tiles_equal_unordered(tiles, &actual_state) {
                founded_key = Some(*key);
                trace!("Found key: {:?}", founded_key);
                break;
            }
        }

        // Use BFS on the graph to find a next state.
        if let Some(key) = founded_key {
            'outer: for start in graph.node_indices() {
                let mut bfs = Bfs::new(&graph, start);
                while let Some(nx) = bfs.next(&graph) {
                    if graph[nx] == key {
                        let neighbors: Vec<NodeIndex> = graph.neighbors(nx).collect();
                        let count = neighbors.len();
                        trace!("Found {} neighbor(s) for key {:?}", count, key);
                        if count > 0 {
                            let random_index = rng.gen_range(0..count);
                            next_state = Some(graph[neighbors[random_index]]);
                        }
                        break 'outer;
                    }
                }
            }
        }

        // Return the next state's tile configuration if available.
        if let Some(key) = next_state {
            trace!("Next state key: {:?}", key);
            if let Some(result) = steps_map.get(key) {
                return result.to_owned();
            }
        }

        // Fallback: perform a random move.
        trace!("State not found, machine will perform a random step");
        let empty_tile_ids: Vec<i32> = tiles_model
            .iter()
            .filter(|tile| tile.empty && !tile.machine_clicked && !tile.human_clicked)
            .map(|tile_data| tile_data.id)
            .collect();
        trace!("Empty tile IDs: {:?}", empty_tile_ids);

        if empty_tile_ids.is_empty() {
            return Vec::new();
        }
        let rnd_tile_id = empty_tile_ids[rng.gen_range(0..empty_tile_ids.len())];
        trace!("Randomly selected tile ID: {:?}", rnd_tile_id);

        actual_state.push(Tile::new(rnd_tile_id, Player::Machine));

        // Try to match the updated state with a valid next step.
        for (key, tiles) in &steps_map {
            if Self::tiles_equal_unordered(tiles, &actual_state) {
                trace!("Found next step with key {:?}", key);
                return tiles.to_vec();
            }
        }

        actual_state
    }

    /// Builds the list of played tiles from the sequence model.
    fn build_steps_from_model(sequence_model: &Rc<VecModel<Sequence>>) -> Vec<Tile> {
        let steps: Vec<Tile> = sequence_model
            .iter()
            .filter_map(|sequence_data| match sequence_data.player.as_str() {
                "H" => Some(Tile::new(sequence_data.id, Player::Human)),
                "M" => Some(Tile::new(sequence_data.id, Player::Machine)),
                _ => None,
            })
            .collect();
        trace!("Built steps from model: {:?}", steps);
        steps
    }

    /// Compares two tile vectors regardless of order.
    fn tiles_equal_unordered(a: &[Tile], b: &[Tile]) -> bool {
        a.len() == b.len()
            && a.iter().copied().collect::<HashSet<_>>()
                == b.iter().copied().collect::<HashSet<_>>()
    }

    /// Returns the winning combination for a given player, if any.
    pub fn get_win_combos(tiles_model: &Rc<VecModel<TileData>>, player: Player) -> Vec<i32> {
        // Helper closure to check if a tile is claimed by the given player.
        let is_claimed_by = |tile: &TileData| {
            !tile.empty
                && match player {
                    Player::Machine => tile.machine_clicked,
                    Player::Human => tile.human_clicked,
                    Player::Nobody => false,
                }
        };

        for combo in WIN_COMBINATIONS.iter() {
            // Check that every tile in the combo is claimed by the player.
            if combo.iter().all(|&id| {
                tiles_model
                    .iter()
                    .any(|tile| tile.id == id && is_claimed_by(&tile))
            }) {
                return combo.to_vec();
            }
        }
        Vec::new()
    }

    /// Checks if there is a winner, and updates the win color accordingly.
    pub fn has_winner(tiles_model: &Rc<VecModel<TileData>>) -> bool {
        for (player, win_color) in &[
            (Player::Machine, MACHINE_WIN_COLOR),
            (Player::Human, HUMAN_WIN_COLOR),
        ] {
            let win_combo = Self::get_win_combos(tiles_model, *player);
            if !win_combo.is_empty() {
                for (i, mut tile_data) in tiles_model.iter().enumerate() {
                    if win_combo.contains(&tile_data.id)
                        && !tile_data.empty
                        && ((matches!(player, Player::Machine) && tile_data.machine_clicked)
                            || (matches!(player, Player::Human) && tile_data.human_clicked))
                    {
                        tile_data.win_color = win_color.clone();
                        tiles_model.set_row_data(i, tile_data);
                    }
                }
                return true;
            }
        }
        false
    }

    /// Starts the game by having the machine make the first move.
    pub fn random_machine_start(
        tiles_model: &Rc<VecModel<TileData>>,
        sequence_model: &Rc<VecModel<Sequence>>,
    ) {
        // Clear the sequence model (removing items in reverse order to avoid shifting issues).
        for i in (0..sequence_model.iter().count()).rev() {
            sequence_model.remove(i);
        }

        let mut rng = rand::thread_rng();
        // Decide whether to start in the center (tile id 4) or top-left (tile id 0).
        let start_tile_id = if rng.gen_range(0..2) == 1 { 4 } else { 0 };

        // Update the chosen tile in the tiles model.
        if let Some((i, mut tile_data)) = tiles_model
            .iter()
            .enumerate()
            .find(|(_, tile)| tile.id == start_tile_id)
        {
            tile_data.machine_clicked = true;
            tile_data.empty = false;
            tiles_model.set_row_data(i, tile_data);
        }

        // Insert the starting move into the sequence model.
        sequence_model.insert(
            0,
            Sequence {
                id: start_tile_id,
                player: SharedString::from("M"),
            },
        );
    }
}

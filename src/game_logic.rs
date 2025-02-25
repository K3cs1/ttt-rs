use crate::win_graph; // We will bridge to win_graph::Tile and win_graph::Player
use log::trace;
use petgraph::prelude::NodeIndex;
use petgraph::visit::Bfs;
use rand::prelude::ThreadRng;
use rand::Rng;
use slint::{Brush, Color, Model, SharedString, VecModel};
use std::collections::HashSet;
use std::rc::Rc;

slint::include_modules!();

const HUMAN_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const MACHINE_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

/// Local "which player" type, used by your UI logic.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Player {
    Machine,
    Human,
    Nobody,
}

/// Local tile type for your UI logic.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Tile {
    pub field_id: i32,
    pub player: Player,
}

impl Tile {
    /*
        pub fn init() -> Self {
            Self {
                field_id: -1,
                player: Player::Nobody,
            }
        }
    */
    pub fn new(field_id: i32, player: Player) -> Self {
        Self { field_id, player }
    }
}

/// A constant list of winning tile combinations for quick local detection.
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

/// --------- BRIDGING TYPES TO/FROM `win_graph.rs` ---------
use win_graph::Player as WinPlayer;
use win_graph::Tile as WinTile;

/// Convert our local `Player` to `win_graph::Player` (skip "Nobody").
fn to_win_player(p: Player) -> Option<WinPlayer> {
    match p {
        Player::Machine => Some(WinPlayer::Machine),
        Player::Human => Some(WinPlayer::Human),
        Player::Nobody => None, // No equivalent in `win_graph`, so skip
    }
}

/// Convert `win_graph::Player` back to local `Player` (we omit "Nobody").
fn from_win_player(wp: WinPlayer) -> Player {
    match wp {
        WinPlayer::Machine => Player::Machine,
        WinPlayer::Human => Player::Human,
    }
}

/// Convert our local `Tile` into a `win_graph::Tile` (skipping if "Nobody").
fn to_win_tile(t: Tile) -> Option<WinTile> {
    // If player is "Nobody," there's no valid move in win_graph.
    let wplayer = to_win_player(t.player)?;
    // If field_id < 0, skip as well.
    if t.field_id < 0 {
        return None;
    }
    Some(WinTile::new(t.field_id as usize, wplayer))
}

/// Convert a `win_graph::Tile` back to our local `Tile`.
fn from_win_tile(wt: &WinTile) -> Tile {
    let p = from_win_player(wt.player);
    Tile::new(wt.idx as i32, p)
}

/// Compare two slices of `win_graph::Tile` ignoring order, by turning them into sets.
fn tiles_equal_unordered_win(a: &[WinTile], b: &[WinTile]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a == set_b
}

/// Build a `Vec<win_graph::Tile>` (WinTile) from the current game’s sequence model.
/// We only convert moves that have a valid Machine/Human player. "Nobody" is ignored.
fn build_win_tiles_from_model(sequence_model: &Rc<VecModel<Sequence>>) -> Vec<WinTile> {
    let mut steps = Vec::new();
    for seq in sequence_model.iter() {
        let local_player = match seq.player.as_str() {
            "M" => Player::Machine,
            "H" => Player::Human,
            _ => Player::Nobody,
        };
        let local_tile = Tile::new(seq.id, local_player);
        if let Some(wt) = to_win_tile(local_tile) {
            steps.push(wt);
        }
    }
    steps
}

/// Convert a `Vec<win_graph::Tile>` back to `Vec<Tile>`.
fn from_win_tiles(seq: &[WinTile]) -> Vec<Tile> {
    seq.iter().map(from_win_tile).collect()
}

/// ----------------------------------------------------------

pub struct GameLogic;

impl GameLogic {
    /// Computes the next move sequence for the Machine using `win_graph`.
    /// 1) Convert our local board-sequence to `Vec<WinTile>` for BFS lookups.
    /// 2) Find a matching key in `win_graph`’s map.
    /// 3) Use BFS on the graph of string keys.
    /// 4) If found, convert that resulting `Vec<WinTile>` back to our local `Vec<Tile>`.
    /// 5) Otherwise do a random move, bridging again as needed.
    pub fn search_next_step(
        tiles_model: &Rc<VecModel<TileData>>,
        sequence_model: &Rc<VecModel<Sequence>>,
    ) -> Vec<Tile> {
        trace!("Sequence model size: {}", sequence_model.row_count());

        // 1) Convert current move sequence to WinTile
        let mut actual_state_win = build_win_tiles_from_model(sequence_model);

        // 2) Load the big map + graph from win_graph.
        let steps_map = win_graph::WinGraph::init_steps_map();
        let graph = win_graph::WinGraph::build_graph();

        let mut founded_key: Option<String> = None;
        let mut next_state: Option<String> = None;
        let mut rng: ThreadRng = rand::rng();

        // 3) Identify which key in the map corresponds to our "actual_state_win."
        //    We'll do an unordered comparison: if steps_map[key] == actual_state_win, we found it.
        for (key, tiles) in &steps_map {
            if tiles_equal_unordered_win(tiles, &actual_state_win) {
                founded_key = Some(key.clone());
                trace!("Found matching key: {}", key);
                break;
            }
        }

        // 4) If found a key, BFS to find a child sequence
        if let Some(ref current_key) = founded_key {
            'outer: for start in graph.node_indices() {
                let mut bfs = Bfs::new(&graph, start);
                while let Some(nx) = bfs.next(&graph) {
                    if graph[nx] == *current_key {
                        let neighbors: Vec<NodeIndex> = graph.neighbors(nx).collect();
                        trace!(
                            "Found {} neighbor(s) for key {}",
                            neighbors.len(),
                            current_key
                        );
                        if !neighbors.is_empty() {
                            let random_index = rng.random_range(0..neighbors.len());
                            // This is the *next* state's string key
                            next_state = Some(graph[neighbors[random_index]].clone());
                        }
                        break 'outer;
                    }
                }
            }
        }

        // 5) If we found a next_state key, return that sequence in *our* Tile type.
        if let Some(ref key) = next_state {
            trace!("Next state key: {}", key);
            if let Some(seq_win_tiles) = steps_map.get(key) {
                // Convert from WinTile -> local Tile
                let seq_local = from_win_tiles(seq_win_tiles);
                return seq_local;
            }
        }

        // 6) Otherwise, do a random move.
        trace!("No BFS match found; picking random empty tile");
        let empty_tile_ids: Vec<i32> = tiles_model
            .iter()
            .filter(|tile| tile.empty && !tile.machine_clicked && !tile.human_clicked)
            .map(|td| td.id)
            .collect();

        if empty_tile_ids.is_empty() {
            // No moves left
            return from_win_tiles(&actual_state_win);
        }

        let rnd_tile_id = empty_tile_ids[rng.random_range(0..empty_tile_ids.len())];
        trace!("Machine picks random tile {}", rnd_tile_id);

        // Add that random move to our *win_graph* style state:
        // (We convert "Machine" -> WinPlayer::Machine, i32->usize)
        actual_state_win.push(win_graph::Tile::new(
            rnd_tile_id as usize,
            WinPlayer::Machine,
        ));

        // Try to see if that updated "actual_state_win" matches a known key in steps_map
        for (key, tiles) in &steps_map {
            if tiles_equal_unordered_win(tiles, &actual_state_win) {
                trace!("Found next step with key: {}", key);
                // Convert from WinTile -> local Tile
                let seq_local = from_win_tiles(tiles);
                return seq_local;
            }
        }

        // If even that didn't match, just return the local version of `actual_state_win`.
        from_win_tiles(&actual_state_win)
    }

    /// Checks if the given player has a 3-in-a-row, for local highlighting logic.
    pub fn get_win_combos(tiles_model: &Rc<VecModel<TileData>>, player: Player) -> Vec<i32> {
        let is_claimed_by = |tile: &TileData| {
            !tile.empty
                && match player {
                    Player::Machine => tile.machine_clicked,
                    Player::Human => tile.human_clicked,
                    Player::Nobody => false,
                }
        };

        for combo in WIN_COMBINATIONS {
            if combo.iter().all(|&id| {
                tiles_model
                    .iter()
                    .any(|tile_data| tile_data.id == id && is_claimed_by(&tile_data))
            }) {
                return combo.to_vec();
            }
        }
        Vec::new()
    }

    /// True if either Machine or Human has a winning line; also updates tile colors in the UI.
    pub fn has_winner(tiles_model: &Rc<VecModel<TileData>>) -> bool {
        for (player, brush) in &[
            (Player::Machine, MACHINE_WIN_COLOR),
            (Player::Human, HUMAN_WIN_COLOR),
        ] {
            let combo = Self::get_win_combos(tiles_model, *player);
            if !combo.is_empty() {
                // Color the winning tiles
                for (i, mut tile_data) in tiles_model.iter().enumerate() {
                    if combo.contains(&tile_data.id)
                        && ((tile_data.machine_clicked && *player == Player::Machine)
                            || (tile_data.human_clicked && *player == Player::Human))
                    {
                        tile_data.win_color = brush.clone();
                        tiles_model.set_row_data(i, tile_data);
                    }
                }
                return true;
            }
        }
        false
    }

    /// Clears the sequence and places a random opening move for the Machine (tile 4 or 0).
    pub fn random_machine_start(
        tiles_model: &Rc<VecModel<TileData>>,
        sequence_model: &Rc<VecModel<Sequence>>,
    ) {
        // Clear any existing moves
        for i in (0..sequence_model.row_count()).rev() {
            sequence_model.remove(i);
        }

        // Choose center or top-left corner
        let mut rng = rand::rng();
        let first_move = if rng.random_range(0..2) == 1 { 4 } else { 0 };

        // Mark tile in the UI
        if let Some((idx, mut tile_data)) = tiles_model
            .iter()
            .enumerate()
            .find(|(_, t)| t.id == first_move)
        {
            tile_data.machine_clicked = true;
            tile_data.empty = false;
            tiles_model.set_row_data(idx, tile_data);
        }

        // Also store it in the sequence model as a Machine move
        sequence_model.insert(
            0,
            Sequence {
                id: first_move,
                player: SharedString::from("M"),
            },
        );
    }
}

use petgraph::Graph;
use std::collections::HashMap;

/// Same minimal definitions.
/// If you already define `Player` in `game_logic.rs`, remove these and `use` them from there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Machine,
    Human,
}

/// A single move: which cell was taken, by which player.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub idx: usize,
    pub player: Player,
}

impl Tile {
    pub fn new(idx: usize, player: Player) -> Self {
        Tile { idx, player }
    }
}

/// Board is array of 9 Option<Player>; None means cell is free.
type Board = [Option<Player>; 9];

/// Checks if there's a winner on this board (Machine or Human).
/// Returns Some(winner) if found, else None.
fn check_winner(board: &Board) -> Option<Player> {
    const LINES: &[[usize; 3]] = &[
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for line in LINES {
        let [a, b, c] = *line;
        if let (Some(p1), Some(p2), Some(p3)) = (board[a], board[b], board[c]) {
            if p1 == p2 && p2 == p3 {
                return Some(p1);
            }
        }
    }
    None
}

/// Limit the maximum depth of sequences to keep memory small.
/// If the Machine hasn't won by this depth, we stop and do not store deeper sequences.
const MAX_DEPTH: usize = 5;

/// Recursively collect all sequences up to `MAX_DEPTH` where Machine eventually wins.
/// We store *short* paths and do not explore further after the Machine wins or we hit the depth limit.
fn collect_machine_wins_limited(
    board: &mut Board,
    path: &mut Vec<Tile>,
    turn: Player,
    results: &mut Vec<Vec<Tile>>,
) {
    // If someone already won, store if it's Machine
    if let Some(w) = check_winner(board) {
        if w == Player::Machine {
            results.push(path.clone());
        }
        return;
    }

    // If we've hit the depth limit, stop
    // (the Machine hasn't won yet, so we do not store path)
    if path.len() >= MAX_DEPTH {
        return;
    }

    // If board is full (draw), stop
    if board.iter().all(|c| c.is_some()) {
        return;
    }

    // Otherwise, try all free cells
    for idx in 0..9 {
        if board[idx].is_none() {
            // Make a move
            board[idx] = Some(turn);
            path.push(Tile::new(idx, turn));

            // Next turn
            let next_player = match turn {
                Player::Machine => Player::Human,
                Player::Human => Player::Machine,
            };

            collect_machine_wins_limited(board, path, next_player, results);

            // Undo
            path.pop();
            board[idx] = None;
        }
    }
}

/// Build a minimal set of (path -> machine wins) up to MAX_DEPTH.
fn generate_limited_machine_wins() -> Vec<Vec<Tile>> {
    let mut board = [None; 9];
    let mut path = Vec::new();
    let mut results = Vec::new();

    // Machine always moves first
    collect_machine_wins_limited(&mut board, &mut path, Player::Machine, &mut results);
    results
}

/// Generate a string key for a path, e.g. "M0->H4->M1->..."
fn sequence_key(seq: &[Tile]) -> String {
    if seq.is_empty() {
        return "empty".to_string();
    }
    let mut parts = Vec::new();
    for tile in seq {
        let letter = match tile.player {
            Player::Machine => "M",
            Player::Human => "H",
        };
        parts.push(format!("{}{}", letter, tile.idx));
    }
    parts.join("->")
}

/// Our main struct
pub struct WinGraph {}

impl WinGraph {
    /// Returns a HashMap: path_key -> the path of up to 5 moves
    /// in which the Machine eventually wins.
    pub fn init_steps_map() -> HashMap<String, Vec<Tile>> {
        let all_wins = generate_limited_machine_wins();
        let mut map = HashMap::new();
        for seq in &all_wins {
            let key = sequence_key(seq);
            // Insert if not yet present
            map.entry(key).or_insert_with(|| seq.clone());
        }
        map
    }

    /// Build a petgraph from the same limited set of sequences.
    /// Because we cap at 5 moves, the graph is fairly small.
    pub fn build_graph() -> Graph<String, ()> {
        let all_wins = generate_limited_machine_wins();

        // Make a map key->sequence
        let mut seq_map = HashMap::new();
        for seq in &all_wins {
            seq_map.insert(sequence_key(seq), seq.clone());
        }

        let mut graph = Graph::<String, ()>::new();
        let mut node_index_map = HashMap::new();

        // Add each sequence as a node
        for (k, _) in &seq_map {
            let idx = graph.add_node(k.clone());
            node_index_map.insert(k.clone(), idx);
        }

        // Link shorter->longer if the longer is exactly the same path plus 1 move
        for (key, seq) in &seq_map {
            let len = seq.len();
            for (cand_key, cand_seq) in &seq_map {
                if cand_seq.len() == len + 1 && cand_seq.starts_with(seq) {
                    let from_idx = node_index_map[key];
                    let to_idx = node_index_map[cand_key];
                    graph.add_edge(from_idx, to_idx, ());
                }
            }
        }

        graph
    }
}

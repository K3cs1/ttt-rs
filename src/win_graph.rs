use petgraph::Graph;
use std::collections::{HashMap};

/// For demonstration, we define our own Player enum here. 
/// If your code already defines it in game_logic, remove this and use your existing version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Machine,
    Human,
}

/// Represents a single move: e.g. "Player X took cell idx".
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

/// We represent a board position as a fixed array of Option<Player>.
/// None = cell not taken. Some(Machine) or Some(Human) = taken by that player.
type Board = [Option<Player>; 9];

/// Helper function: check if the board is won and by whom.
/// Returns Some(Player::Machine) if Machine has won, Some(Player::Human) if Human has won,
/// or None if no winner yet.
fn check_winner(board: &Board) -> Option<Player> {
    // All winning lines on a 3x3 board:
    // rows:    (0,1,2), (3,4,5), (6,7,8)
    // columns: (0,3,6), (1,4,7), (2,5,8)
    // diagonals: (0,4,8), (2,4,6)
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

/// Recursively generate all move sequences (from an empty board) that end with a Machine win.
/// We store each move as a Tile { idx, player } in a path.
/// 
/// - `board` = current state of the board
/// - `path` = moves made so far
/// - `turn` = whose turn it is (Machine or Human)
/// - `results` = a list of all winning sequences found
fn backtrack_all_machine_wins(
    board: &mut Board,
    path: &mut Vec<Tile>,
    turn: Player,
    results: &mut Vec<Vec<Tile>>,
) {
    // Check if we already have a winner
    if let Some(winner) = check_winner(board) {
        // If it's the Machine, store the path. If it's the Human, discard.
        if winner == Player::Machine {
            results.push(path.clone());
        }
        return;
    }

    // If the board is full (draw) and no winner, just stop
    if board.iter().all(|cell| cell.is_some()) {
        return;
    }

    // It's "turn" player's move. Let's try all empty cells as possible moves.
    for idx in 0..9 {
        if board[idx].is_none() {
            // Make the move
            board[idx] = Some(turn);
            path.push(Tile::new(idx, turn));

            // Next player's turn
            let next_player = match turn {
                Player::Machine => Player::Human,
                Player::Human => Player::Machine,
            };

            // Recurse
            backtrack_all_machine_wins(board, path, next_player, results);

            // Undo
            path.pop();
            board[idx] = None;
        }
    }
}

/// Generate *all* sequences (Vec<Tile>) in which the Machine eventually wins.
/// Return them in a vector.
fn generate_all_machine_win_sequences() -> Vec<Vec<Tile>> {
    let mut board = [None; 9];
    let mut path = Vec::new();
    let mut results = Vec::new();
    // Machine always moves first
    backtrack_all_machine_wins(&mut board, &mut path, Player::Machine, &mut results);
    results
}

/// Create a unique key for a move sequence so we can store it in a map.
/// For example: "M0->H4->M1->..."
fn sequence_key(moves: &[Tile]) -> String {
    if moves.is_empty() {
        return String::from("empty");
    }
    let mut parts = Vec::with_capacity(moves.len());
    for tile in moves {
        let letter = match tile.player {
            Player::Machine => "M",
            Player::Human => "H",
        };
        parts.push(format!("{}{}", letter, tile.idx));
    }
    parts.join("->")
}

/// We’ll store each sequence under a string key in a HashMap.
/// Then, in build_graph, we’ll create a node for each sequence
/// and add edges from sequence_n to sequence_{n+1} if the latter extends the former by 1 move.
pub struct WinGraph {}

impl WinGraph {
    pub fn init_steps_map() -> HashMap<String, Vec<Tile>> {
        let all_wins = generate_all_machine_win_sequences();

        let mut map = HashMap::new();
        for seq in &all_wins {
            let key = sequence_key(seq);
            map.insert(key, seq.clone());
        }
        map
    }

    /// Build a petgraph Graph of all winning sequences.
    /// Each node is identified by the sequence key, and there's
    /// a directed edge from keyA -> keyB if B = A + one extra move.
    pub fn build_graph() -> Graph<String, ()> {
        let all_wins = generate_all_machine_win_sequences();

        // We'll store each sequence in a map: key -> the sequence of moves
        let mut seq_map = HashMap::new();
        for seq in &all_wins {
            seq_map.insert(sequence_key(seq), seq.clone());
        }

        // Create a node in the graph for each sequence
        let mut graph = Graph::<String, ()>::new();
        let mut node_index_map = HashMap::new();
        for (key, _) in &seq_map {
            let idx = graph.add_node(key.clone());
            node_index_map.insert(key.clone(), idx);
        }

        // For each sequence, see if there's a "child" sequence that is exactly one move longer
        // and starts with the current sequence's moves.
        // We'll add a directed edge from the shorter to the longer.
        for (key, seq) in &seq_map {
            let len = seq.len();
            // The next sequence must have len+1 moves, same first `len` moves
            // so let's build a prefix to look for
            for (candidate_key, candidate_seq) in &seq_map {
                if candidate_seq.len() == len + 1 && candidate_seq.starts_with(seq) {
                    // Then we link key -> candidate_key
                    let from_idx = node_index_map[&key[..]];
                    let to_idx = node_index_map[&candidate_key[..]];
                    graph.add_edge(from_idx, to_idx, ());
                }
            }
        }

        graph
    }
}

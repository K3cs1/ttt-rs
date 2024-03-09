use log::{info, trace, warn};
use slint::{Brush, Color, Model, VecModel, Weak};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::option::Option;
use std::rc::Rc;

const PLAYER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const COMPUTER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

//slint::include_modules!();

slint::slint! {
    import { AppWindow } from "ui/appwindow.slint";
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Player {
    Machine,
    Human,
    Nobody,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    value: Vec<Tile>,
    neighbors: Vec<Node>,
}

impl Node {
    pub fn new(value: Vec<Tile>) -> Self {
        Node {
            value,
            neighbors: Vec::new(),
        }
    }

    pub fn get_value(&self) -> Vec<Tile> {
        return self.value.clone();
    }

    pub fn get_neighbors(&mut self) -> &mut Vec<Node> {
        &mut self.neighbors
    }

    pub fn connect(&mut self, node: &Node) {
        //if self == node {}
        self.neighbors.push(node.clone());
        self.neighbors.push(self.clone());
    }
}

fn init_nodes() -> Node {
    let mut start = Node::new(vec![Tile::init()]);
    let mut level_1_1 = Node::new(vec![Tile::new(4, Player::Machine)]);
    start.connect(&level_1_1);

    let mut level_1_2 = Node::new(vec![Tile::new(0, Player::Machine)]);
    start.connect(&level_1_2);

    let mut level_2_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
    ]);
    level_1_1.connect(&level_2_1);

    let mut level_2_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
    ]);
    level_1_1.connect(&level_2_2);

    let mut level_2_3 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(1, Player::Human),
    ]);
    level_1_2.connect(&level_2_3);

    let mut level_2_4 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(2, Player::Human),
    ]);
    level_1_2.connect(&level_2_4);

    let mut level_2_5 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
    ]);
    level_1_2.connect(&level_2_5);

    let mut level_3_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(6, Player::Machine),
    ]);
    level_2_1.connect(&level_3_1);

    let mut level_3_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
    ]);
    level_2_2.connect(&level_3_2);

    let mut level_3_3 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(6, Player::Machine),
    ]);
    level_2_4.connect(&level_3_3);

    let mut level_3_4 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
    ]);
    level_2_5.connect(&level_3_4);

    let mut level_4_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(0, Player::Human),
    ]);
    level_3_1.connect(&level_4_1);

    let mut level_4_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(2, Player::Human),
    ]);
    level_3_1.connect(&level_4_2);

    let mut level_4_3 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(3, Player::Human),
    ]);
    level_3_2.connect(&level_4_3);

    let mut level_4_4 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
    ]);
    level_3_2.connect(&level_4_4);

    let mut level_4_5 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(3, Player::Human),
    ]);
    level_3_3.connect(&level_4_5);

    let mut level_4_6 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(1, Player::Human),
    ]);
    level_3_3.connect(&level_4_6);

    let mut level_4_7 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
    ]);
    level_3_4.connect(&level_4_7);

    let mut level_4_8 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(6, Player::Human),
    ]);
    level_3_4.connect(&level_4_8);

    let mut level_5_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
    ]);
    level_4_1.connect(&level_5_1);

    let mut level_5_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(0, Player::Machine),
    ]);
    level_4_2.connect(&level_5_2);

    let mut level_5_3 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(6, Player::Machine),
    ]);
    level_4_3.connect(&level_5_3);

    let mut level_5_4 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
    ]);
    level_4_4.connect(&level_5_4);

    let mut level_5_5 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(8, Player::Machine),
    ]);
    level_4_5.connect(&level_5_5);

    let mut level_5_6 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(3, Player::Machine),
    ]);
    level_4_6.connect(&level_5_6);

    let mut level_5_7 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
    ]);
    level_4_7.connect(&level_5_7);

    let mut level_5_8 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(2, Player::Machine),
    ]);
    level_4_8.connect(&level_5_8);

    let mut level_6_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(0, Player::Machine),
        Tile::new(8, Player::Human),
    ]);
    level_5_2.connect(&level_6_1);

    let mut level_6_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(5, Player::Human),
    ]);
    level_5_4.connect(&level_6_2);

    let mut level_6_3 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(7, Player::Human),
    ]);
    level_5_4.connect(&level_6_3);

    let mut level_6_4 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(7, Player::Human),
    ]);
    level_5_5.connect(&level_6_4);

    let mut level_6_5 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(2, Player::Human),
    ]);
    level_5_7.connect(&level_6_5);

    let mut level_6_6 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(6, Player::Human),
    ]);
    level_5_7.connect(&level_6_6);

    let mut level_6_7 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(1, Player::Human),
    ]);
    level_5_8.connect(&level_6_7);

    let mut level_7_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(0, Player::Machine),
        Tile::new(8, Player::Human),
        Tile::new(3, Player::Machine),
    ]);
    level_6_1.connect(&level_7_1);

    let mut level_7_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(5, Player::Human),
        Tile::new(1, Player::Machine),
    ]);
    level_6_2.connect(&level_7_2);

    let mut level_7_3 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(7, Player::Human),
        Tile::new(5, Player::Machine),
    ]);
    level_6_3.connect(&level_7_3);

    let mut level_7_4 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(7, Player::Human),
        Tile::new(4, Player::Machine),
    ]);
    level_6_4.connect(&level_7_4);

    let mut level_7_5 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(6, Player::Machine),
    ]);
    level_6_5.connect(&level_7_5);

    let mut level_7_6 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(2, Player::Machine),
    ]);
    level_6_6.connect(&level_7_6);

    let mut level_7_7 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(5, Player::Machine),
    ]);
    level_6_7.connect(&level_7_7);

    let mut level_8_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(5, Player::Human),
        Tile::new(1, Player::Machine),
        Tile::new(8, Player::Human),
    ]);
    level_7_2.connect(&level_8_1);

    let mut level_8_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(5, Player::Human),
        Tile::new(1, Player::Machine),
        Tile::new(7, Player::Human),
    ]);
    level_7_2.connect(&level_8_2);

    let mut level_8_3 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(1, Player::Human),
    ]);
    level_7_5.connect(&level_8_3);

    let mut level_8_4 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(7, Player::Human),
    ]);
    level_7_5.connect(&level_8_4);

    let mut level_9_1 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(5, Player::Human),
        Tile::new(1, Player::Machine),
        Tile::new(8, Player::Human),
        Tile::new(7, Player::Machine),
    ]);
    level_8_1.connect(&level_9_1);

    let mut level_9_2 = Node::new(vec![
        Tile::new(4, Player::Machine),
        Tile::new(0, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(3, Player::Machine),
        Tile::new(5, Player::Human),
        Tile::new(1, Player::Machine),
        Tile::new(7, Player::Human),
        Tile::new(8, Player::Machine),
    ]);
    level_8_1.connect(&level_9_2);

    let mut level_9_3 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(6, Player::Human),
        Tile::new(2, Player::Machine),
        Tile::new(1, Player::Human),
        Tile::new(7, Player::Machine),
    ]);
    level_8_3.connect(&level_9_3);

    let mut level_9_4 = Node::new(vec![
        Tile::new(0, Player::Machine),
        Tile::new(4, Player::Human),
        Tile::new(8, Player::Machine),
        Tile::new(3, Player::Human),
        Tile::new(5, Player::Machine),
        Tile::new(2, Player::Human),
        Tile::new(6, Player::Machine),
        Tile::new(7, Player::Human),
        Tile::new(1, Player::Machine),
    ]);
    level_8_4.connect(&level_9_4);

    start
}

fn search_step(tiles_model: Rc<VecModel<TileData>>) -> Option<Node> {
    let mut queue: VecDeque<Node> = VecDeque::new();
    let actual_step: Node = build_state_from_model(tiles_model);
    let start_node: Node = init_nodes();
    let mut current_node: Node = Node::new(vec![Tile::init()]);
    let mut current_values: Vec<Tile> = Vec::new();
    let mut already_visited: HashSet<Node> = HashSet::new();

    queue.push_back(start_node);
    loop {
        if !queue.is_empty() {
            match queue.pop_front() {
                Some(node) => {
                    current_node = node;
                    current_values = current_node.get_value();
                },
                None => warn!("Can not get element from queue!"),
            }
            trace!("Visited node: ");
            for t in &current_values {
                trace!("===");
                trace!("{:?}", t.field_id);
                trace!("{:?}", t.player);
            }
            if vec_tile_compare(&current_values, &actual_step.get_value()) {
                return Some(current_node);
            }

            already_visited.insert(current_node.clone());
            for node in current_node.get_neighbors() {
                queue.push_back(node.clone());
            }

            let mut queue_clone = queue.clone();
            if let Some(queue_entry) = &mut queue.iter_mut().enumerate().next() {
                for visited_node in already_visited.iter() {
                    if vec_tile_compare(&visited_node.get_value(), &queue_entry.1.get_value()) {
                        queue_clone.remove(queue_entry.0);
                    }
                }
            }
            queue = queue_clone;
        }
    }
    //Some(Node::new(vec![Tile::init()]))
}

fn build_state_from_model(tiles_model: Rc<VecModel<TileData>>) -> Node {
    let mut all_tiles = tiles_model.iter().enumerate();

    let mut states: Vec<Tile> = vec![];
    if let Some((tile_idx, mut tile_data)) = all_tiles.next() {
        //let title_field_id = usize::try_from(tile_data.id).unwrap();
        if tile_data.human_clicked == true {
            states.push(Tile::new(tile_data.id, Player::Human));
        } else {
            states.push(Tile::new(tile_data.id, Player::Machine));
        }
    }
    Node::new(states)
}

// fn has_human_win_combo(tiles_model: Rc<VecModel<TileData>>) -> WinGraph<Vec<Tile>> {
//     todo!()
// }

fn tile_eq(tile_a: &Tile, tile_b: &Tile) -> bool {
    (tile_a == tile_b) || 
    (tile_a.field_id == tile_b.field_id && tile_a.player == tile_b.player)
}

fn vec_tile_compare(vector_a: &[Tile], vector_b: &[Tile]) -> bool {
    (vector_a.len() == vector_b.len()) &&
     vector_a.iter()
       .zip(vector_b)
       .all(|(tile_a,tile_b)| tile_eq(tile_a, tile_b))
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

    ui.on_process(move |id: i32| {
        info!("Selected id: {}", id);

        //let ui: AppWindow = ui_weak.unwrap();
        //let human_win_combo = has_human_win_combo(tiles_model.clone());
        //let machine_win_combo = has_machine_win_combo(tiles_model.clone());

        let mut empty_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.empty);

        //Human turn
        empty_tiles.for_each(|(_i, mut tile_data)| {
            if id == tile_data.id {
                tile_data.human_clicked = true;
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
                tile_data.machine_clicked = true;
                tile_data.empty = false;
                //tile_data.win_color = COMPUTER_WIN_COLOR;
                tiles_model.set_row_data(_i, tile_data);
            }
        });
    });

    ui.run().unwrap();
}

use log::{info, trace, warn};
use petgraph::dot::Dot;
use petgraph::visit::Bfs;
use petgraph::Graph;
use petgraph_evcxr::draw_graph;
use slint::{Brush, Color, Model, VecModel, Weak};
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::option::Option;
use std::rc::Rc;

const PLAYER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(0, 140, 0));
const COMPUTER_WIN_COLOR: Brush = Brush::SolidColor(Color::from_rgb_u8(140, 0, 0));

//slint::include_modules!();

slint::slint! {
    import { AppWindow } from "ui/appwindow.slint";
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Player {
    Machine,
    Human,
    Nobody,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

fn init_steps_map() -> HashMap<&'static str, Vec<Tile>> {
    let mut steps_map = HashMap::new();
    steps_map.insert("0_0", Vec::from([Tile::init()]));
    steps_map.insert("1_0", Vec::from([Tile::new(4, Player::Machine)]));
    steps_map.insert("1_1", Vec::from([Tile::new(0, Player::Machine)]));
    steps_map.insert(
        "2_0",
        Vec::from([Tile::new(4, Player::Machine), Tile::new(1, Player::Human)]),
    );
    steps_map.insert(
        "2_1",
        Vec::from([Tile::new(4, Player::Machine), Tile::new(0, Player::Human)]),
    );
    steps_map.insert(
        "2_2",
        Vec::from([Tile::new(0, Player::Machine), Tile::new(1, Player::Human)]),
    );
    steps_map.insert(
        "2_3",
        Vec::from([Tile::new(0, Player::Machine), Tile::new(2, Player::Human)]),
    );
    steps_map.insert(
        "2_4",
        Vec::from([Tile::new(0, Player::Machine), Tile::new(4, Player::Human)]),
    );
    steps_map.insert(
        "3_0",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
        ]),
    );
    steps_map.insert(
        "3_1",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
        ]),
    );
    steps_map.insert(
        "3_2",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(6, Player::Machine),
        ]),
    );
    steps_map.insert(
        "3_3",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
        ]),
    );
    steps_map.insert(
        "4_0",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(0, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_1",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_2",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(3, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_3",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_4",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(3, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_5",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(1, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_5",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(1, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_6",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
        ]),
    );
    steps_map.insert(
        "4_7",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(6, Player::Human),
        ]),
    );
    steps_map.insert(
        "5_0",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
        ]),
    );
    steps_map.insert(
        "5_1",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(0, Player::Machine),
        ]),
    );
    steps_map.insert(
        "5_2",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(6, Player::Machine),
        ]),
    );
    steps_map.insert(
        "5_3",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
        ]),
    );
    steps_map.insert(
        "5_4",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(8, Player::Machine),
        ]),
    );
    steps_map.insert(
        "5_5",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(3, Player::Machine),
        ]),
    );
    steps_map.insert(
        "5_6",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
        ]),
    );
    steps_map.insert(
        "5_7",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(2, Player::Machine),
        ]),
    );
    steps_map.insert(
        "6_0",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(0, Player::Machine),
            Tile::new(8, Player::Human),
        ]),
    );
    steps_map.insert(
        "6_1",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(5, Player::Human),
        ]),
    );
    steps_map.insert(
        "6_2",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(7, Player::Human),
        ]),
    );
    steps_map.insert(
        "6_3",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(7, Player::Human),
        ]),
    );
    steps_map.insert(
        "6_4",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(2, Player::Human),
        ]),
    );
    steps_map.insert(
        "6_5",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(6, Player::Human),
        ]),
    );
    steps_map.insert(
        "6_6",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(1, Player::Human),
        ]),
    );
    steps_map.insert(
        "7_0",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(0, Player::Machine),
            Tile::new(8, Player::Human),
            Tile::new(3, Player::Machine),
        ]),
    );
    steps_map.insert(
        "7_1",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(5, Player::Human),
            Tile::new(1, Player::Machine),
        ]),
    );
    steps_map.insert(
        "7_2",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(7, Player::Human),
            Tile::new(5, Player::Machine),
        ]),
    );
    steps_map.insert(
        "7_3",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(7, Player::Human),
            Tile::new(4, Player::Machine),
        ]),
    );
    steps_map.insert(
        "7_4",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(6, Player::Machine),
        ]),
    );
    steps_map.insert(
        "7_5",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(2, Player::Machine),
        ]),
    );
    steps_map.insert(
        "7_6",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(5, Player::Machine),
        ]),
    );
    steps_map.insert(
        "8_0",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(5, Player::Human),
            Tile::new(1, Player::Machine),
            Tile::new(8, Player::Human),
        ]),
    );
    steps_map.insert(
        "8_1",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(5, Player::Human),
            Tile::new(1, Player::Machine),
            Tile::new(7, Player::Human),
        ]),
    );
    steps_map.insert(
        "8_2",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(1, Player::Human),
        ]),
    );
    steps_map.insert(
        "8_3",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(7, Player::Human),
        ]),
    );
    steps_map.insert(
        "9_0",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(5, Player::Human),
            Tile::new(1, Player::Machine),
            Tile::new(8, Player::Human),
            Tile::new(7, Player::Machine),
        ]),
    );
    steps_map.insert(
        "9_1",
        Vec::from([
            Tile::new(4, Player::Machine),
            Tile::new(0, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(3, Player::Machine),
            Tile::new(5, Player::Human),
            Tile::new(1, Player::Machine),
            Tile::new(7, Player::Human),
            Tile::new(8, Player::Machine),
        ]),
    );
    steps_map.insert(
        "9_2",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(6, Player::Human),
            Tile::new(2, Player::Machine),
            Tile::new(1, Player::Human),
            Tile::new(7, Player::Machine),
        ]),
    );
    steps_map.insert(
        "9_3",
        Vec::from([
            Tile::new(0, Player::Machine),
            Tile::new(4, Player::Human),
            Tile::new(8, Player::Machine),
            Tile::new(3, Player::Human),
            Tile::new(5, Player::Machine),
            Tile::new(2, Player::Human),
            Tile::new(6, Player::Machine),
            Tile::new(7, Player::Human),
            Tile::new(1, Player::Machine),
        ]),
    );

    steps_map
}

fn build_graph() -> Graph<&'static str, &'static str> {
    let mut graph = Graph::<&str, &str>::new();
    let node_0_0 = graph.add_node("0_0");
    let node_1_0 = graph.add_node("1_0");
    let node_1_1 = graph.add_node("1_1");
    let node_2_0 = graph.add_node("2_0");
    let node_2_1 = graph.add_node("2_1");
    let node_2_2 = graph.add_node("2_2");
    let node_2_3 = graph.add_node("2_3");
    let node_2_4 = graph.add_node("2_4");
    let node_3_0 = graph.add_node("3_0");
    let node_3_1 = graph.add_node("3_1");
    let node_3_2 = graph.add_node("3_2");
    let node_3_3 = graph.add_node("3_3");
    let node_4_0 = graph.add_node("4_0");
    let node_4_1 = graph.add_node("4_1");
    let node_4_2 = graph.add_node("4_2");
    let node_4_3 = graph.add_node("4_3");
    let node_4_4 = graph.add_node("4_4");
    let node_4_5 = graph.add_node("4_5");
    let node_4_6 = graph.add_node("4_6");
    let node_4_7 = graph.add_node("4_7");
    let node_5_0 = graph.add_node("5_0");
    let node_5_1 = graph.add_node("5_1");
    let node_5_2 = graph.add_node("5_2");
    let node_5_3 = graph.add_node("5_3");
    let node_5_4 = graph.add_node("5_4");
    let node_5_5 = graph.add_node("5_5");
    let node_5_6 = graph.add_node("5_6");
    let node_5_7 = graph.add_node("5_7");
    let node_6_0 = graph.add_node("6_0");
    let node_6_1 = graph.add_node("6_1");
    let node_6_2 = graph.add_node("6_2");
    let node_6_3 = graph.add_node("6_3");
    let node_6_4 = graph.add_node("6_4");
    let node_6_5 = graph.add_node("6_5");
    let node_6_6 = graph.add_node("6_6");
    let node_7_0 = graph.add_node("7_0");
    let node_7_1 = graph.add_node("7_1");
    let node_7_2 = graph.add_node("7_2");
    let node_7_3 = graph.add_node("7_3");
    let node_7_4 = graph.add_node("7_4");
    let node_7_5 = graph.add_node("7_5");
    let node_7_6 = graph.add_node("7_6");
    let node_8_0 = graph.add_node("8_0");
    let node_8_1 = graph.add_node("8_1");
    let node_8_2 = graph.add_node("8_2");
    let node_8_3 = graph.add_node("8_3");
    let node_9_0 = graph.add_node("9_0");
    let node_9_1 = graph.add_node("9_1");
    let node_9_2 = graph.add_node("9_2");
    let node_9_3 = graph.add_node("9_3");

    graph.extend_with_edges(&[
        (node_0_0, node_1_0),
        (node_0_0, node_1_1),
        (node_1_0, node_2_0),
        (node_1_0, node_2_1),
        (node_1_1, node_2_2),
        (node_1_1, node_2_3),
        (node_1_1, node_2_4),
        (node_2_0, node_3_0),
        (node_2_1, node_3_1),
        (node_2_3, node_3_2),
        (node_2_4, node_3_3),
        (node_3_0, node_4_0),
        (node_3_0, node_4_1),
        (node_3_1, node_4_2),
        (node_3_1, node_4_3),
        (node_3_2, node_4_4),
        (node_3_2, node_4_5),
        (node_3_3, node_4_6),
        (node_3_3, node_4_7),
        (node_4_0, node_5_0),
        (node_4_1, node_5_1),
        (node_4_2, node_5_2),
        (node_4_3, node_5_3),
        (node_4_4, node_5_4),
        (node_4_5, node_5_5),
        (node_4_6, node_5_6),
        (node_4_7, node_5_7),
        (node_6_0, node_7_0),
        (node_6_1, node_7_1),
        (node_6_2, node_7_2),
        (node_6_3, node_7_3),
        (node_6_4, node_7_4),
        (node_6_5, node_7_5),
        (node_6_6, node_7_6),
        (node_7_1, node_8_0),
        (node_7_1, node_8_1),
        (node_7_4, node_8_2),
        (node_7_4, node_8_3),
        (node_8_0, node_9_0),
        (node_8_1, node_9_1),
        (node_8_2, node_9_2),
        (node_8_3, node_9_3),
    ]);

    graph
}

fn search_step(tiles_model: &Rc<VecModel<TileData>>) -> Vec<Tile> {
    let actual_state: Vec<Tile> = build_steps_from_model(&tiles_model);
    let steps_map: HashMap<&str, Vec<Tile>> = init_steps_map();
    let graph = build_graph();
    let mut founded_key = None;
    let mut founded_state = None;

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
                        // let founded_state = steps_map.get(key);
                        // info!("Found state: {:?}", founded_state.unwrap());
                        founded_state = Some(key);
                        break;
                    }
                }
                None => (),
            }
        }
    }

    let mut founded_state_vec = Vec::new();
    match founded_state {
        Some(key) => {
            let hash_map = steps_map.clone();
            let result = &hash_map.get(key);
            return result.unwrap().to_owned();
        }
        None => {
            warn!("State not found");
            return founded_state_vec;
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

fn main() {
    //-> Result<(), slint::PlatformError>
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let ui = AppWindow::new().unwrap();
    let _ui_weak: Weak<AppWindow> = ui.as_weak();

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

        //Computer turn
        let founded_state_vec = search_step(&tiles_model);

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

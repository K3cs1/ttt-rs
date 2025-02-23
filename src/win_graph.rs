use crate::game_logic::{Player, Tile};
use petgraph::Graph;
use std::collections::HashMap;

pub struct WinGraph {}

impl WinGraph {
    pub fn init_steps_map() -> HashMap<&'static str, Vec<Tile>> {
        // Define a static slice of key and tile-array pairs.
        // (Note: duplicate keys have been removed.)
        let entries: &[(&'static str, &[Tile])] = &[
            ("0_0", &[Tile::init()]),
            ("1_0", &[Tile::new(4, Player::Machine)]),
            ("1_1", &[Tile::new(0, Player::Machine)]),
            (
                "2_0",
                &[Tile::new(4, Player::Machine), Tile::new(1, Player::Human)],
            ),
            (
                "2_1",
                &[Tile::new(4, Player::Machine), Tile::new(0, Player::Human)],
            ),
            (
                "2_2",
                &[Tile::new(0, Player::Machine), Tile::new(1, Player::Human)],
            ),
            (
                "2_3",
                &[Tile::new(0, Player::Machine), Tile::new(2, Player::Human)],
            ),
            (
                "2_4",
                &[Tile::new(0, Player::Machine), Tile::new(4, Player::Human)],
            ),
            (
                "3_0",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(6, Player::Machine),
                ],
            ),
            (
                "3_1",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                ],
            ),
            (
                "3_2",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(6, Player::Machine),
                ],
            ),
            (
                "3_3",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                ],
            ),
            (
                "4_0",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(0, Player::Human),
                ],
            ),
            (
                "4_1",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(2, Player::Human),
                ],
            ),
            (
                "4_2",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(3, Player::Human),
                ],
            ),
            (
                "4_3",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                ],
            ),
            (
                "4_4",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(3, Player::Human),
                ],
            ),
            (
                "4_5",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(1, Player::Human),
                ],
            ),
            (
                "4_6",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                ],
            ),
            (
                "4_7",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(6, Player::Human),
                ],
            ),
            (
                "5_0",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                ],
            ),
            (
                "5_1",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(0, Player::Machine),
                ],
            ),
            (
                "5_2",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(6, Player::Machine),
                ],
            ),
            (
                "5_3",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                ],
            ),
            (
                "5_4",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(8, Player::Machine),
                ],
            ),
            (
                "5_5",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(3, Player::Machine),
                ],
            ),
            (
                "5_6",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                ],
            ),
            (
                "5_7",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(2, Player::Machine),
                ],
            ),
            (
                "6_0",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(0, Player::Machine),
                    Tile::new(8, Player::Human),
                ],
            ),
            (
                "6_1",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(5, Player::Human),
                ],
            ),
            (
                "6_2",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(7, Player::Human),
                ],
            ),
            (
                "6_3",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(7, Player::Human),
                ],
            ),
            (
                "6_4",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(2, Player::Human),
                ],
            ),
            (
                "6_5",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(6, Player::Human),
                ],
            ),
            (
                "6_6",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(1, Player::Human),
                ],
            ),
            (
                "7_0",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(0, Player::Machine),
                    Tile::new(8, Player::Human),
                    Tile::new(3, Player::Machine),
                ],
            ),
            (
                "7_1",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(5, Player::Human),
                    Tile::new(1, Player::Machine),
                ],
            ),
            (
                "7_2",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(7, Player::Human),
                    Tile::new(5, Player::Machine),
                ],
            ),
            (
                "7_3",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(7, Player::Human),
                    Tile::new(4, Player::Machine),
                ],
            ),
            (
                "7_4",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(6, Player::Machine),
                ],
            ),
            (
                "7_5",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(2, Player::Machine),
                ],
            ),
            (
                "7_6",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(5, Player::Machine),
                ],
            ),
            (
                "8_0",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(5, Player::Human),
                    Tile::new(1, Player::Machine),
                    Tile::new(8, Player::Human),
                ],
            ),
            (
                "8_1",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(5, Player::Human),
                    Tile::new(1, Player::Machine),
                    Tile::new(7, Player::Human),
                ],
            ),
            (
                "8_2",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(1, Player::Human),
                ],
            ),
            (
                "8_3",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(7, Player::Human),
                ],
            ),
            (
                "9_0",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(5, Player::Human),
                    Tile::new(1, Player::Machine),
                    Tile::new(8, Player::Human),
                    Tile::new(7, Player::Machine),
                ],
            ),
            (
                "9_1",
                &[
                    Tile::new(4, Player::Machine),
                    Tile::new(0, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(3, Player::Machine),
                    Tile::new(5, Player::Human),
                    Tile::new(1, Player::Machine),
                    Tile::new(7, Player::Human),
                    Tile::new(8, Player::Machine),
                ],
            ),
            (
                "9_2",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(6, Player::Human),
                    Tile::new(2, Player::Machine),
                    Tile::new(1, Player::Human),
                    Tile::new(7, Player::Machine),
                ],
            ),
            (
                "9_3",
                &[
                    Tile::new(0, Player::Machine),
                    Tile::new(4, Player::Human),
                    Tile::new(8, Player::Machine),
                    Tile::new(3, Player::Human),
                    Tile::new(5, Player::Machine),
                    Tile::new(2, Player::Human),
                    Tile::new(6, Player::Machine),
                    Tile::new(7, Player::Human),
                    Tile::new(1, Player::Machine),
                ],
            ),
        ];

        let mut steps_map = HashMap::new();
        for &(key, tiles) in entries {
            steps_map.insert(key, tiles.to_vec());
        }
        steps_map
    }

    pub fn build_graph() -> Graph<&'static str, &'static str> {
        let keys = [
            "0_0", "1_0", "1_1", "2_0", "2_1", "2_2", "2_3", "2_4", "3_0", "3_1", "3_2", "3_3",
            "4_0", "4_1", "4_2", "4_3", "4_4", "4_5", "4_6", "4_7", "5_0", "5_1", "5_2", "5_3",
            "5_4", "5_5", "5_6", "5_7", "6_0", "6_1", "6_2", "6_3", "6_4", "6_5", "6_6", "7_0",
            "7_1", "7_2", "7_3", "7_4", "7_5", "7_6", "8_0", "8_1", "8_2", "8_3", "9_0", "9_1",
            "9_2", "9_3",
        ];
        let mut graph = Graph::<&'static str, &'static str>::new();
        // Build a map from key to node index.
        let nodes: HashMap<_, _> = keys.iter().map(|&k| (k, graph.add_node(k))).collect();

        let edges = [
            ("0_0", "1_0"),
            ("0_0", "1_1"),
            ("1_0", "2_0"),
            ("1_0", "2_1"),
            ("1_1", "2_2"),
            ("1_1", "2_3"),
            ("1_1", "2_4"),
            ("2_0", "3_0"),
            ("2_1", "3_1"),
            ("2_3", "3_2"),
            ("2_4", "3_3"),
            ("3_0", "4_0"),
            ("3_0", "4_1"),
            ("3_1", "4_2"),
            ("3_1", "4_3"),
            ("3_2", "4_4"),
            ("3_2", "4_5"),
            ("3_3", "4_6"),
            ("3_3", "4_7"),
            ("4_0", "5_0"),
            ("4_1", "5_1"),
            ("4_2", "5_2"),
            ("4_3", "5_3"),
            ("4_4", "5_4"),
            ("4_5", "5_5"),
            ("4_6", "5_6"),
            ("4_7", "5_7"),
            ("6_0", "7_0"),
            ("6_1", "7_1"),
            ("6_2", "7_2"),
            ("6_3", "7_3"),
            ("6_4", "7_4"),
            ("6_5", "7_5"),
            ("6_6", "7_6"),
            ("7_1", "8_0"),
            ("7_1", "8_1"),
            ("7_4", "8_2"),
            ("7_4", "8_3"),
            ("8_0", "9_0"),
            ("8_1", "9_1"),
            ("8_2", "9_2"),
            ("8_3", "9_3"),
        ];

        for &(from, to) in &edges {
            let from_idx = nodes.get(from).expect("Missing node");
            let to_idx = nodes.get(to).expect("Missing node");
            graph.add_edge(*from_idx, *to_idx, "");
        }
        graph
    }
}

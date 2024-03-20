use std::collections::HashMap;
use petgraph::Graph;

use crate::{game_logic::Player, game_logic::Tile};

pub struct WinGraph {}

impl WinGraph {
    pub fn init_steps_map() -> HashMap<&'static str, Vec<Tile>> {
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

    pub fn build_graph() -> Graph<&'static str, &'static str> {
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
}

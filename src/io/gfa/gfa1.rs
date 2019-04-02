/*
Copyright (c) 2018 Pierre Marijon <pierre.marijon@inria.fr>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

/* std use */
use std;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};

/* crate use */
use petgraph;
use petgraph::graph::NodeIndex;

/* project use */
use io;
use filter;
use filter::Filter;

//               read_a  strand read_a strand length
type LineType = (String, char, String, char, u64);
//                      read_a strand leng_a read_b strand len_b position len_containment
type ContainmentType = (String, char, u64, String, char, u64, u64, u64);

type Graph = petgraph::Graph<(String, u64), LineType>;

pub struct Gfa1 {
    keep_internal: bool,
    keep_containment: bool,
    graph: Graph,
    containments: HashMap<(String, u64), ContainmentType>,
    test_containment: filter::Containment,
    test_internalmatch: filter::InternalMatch,
    node2index: HashMap<(String, u64), petgraph::graph::NodeIndex>,
}

impl Gfa1 {
    pub fn new(keep_internal: bool, keep_containment: bool, internal_threshold: f64) -> Self {
        Gfa1 {
            keep_internal: keep_internal,
            keep_containment: keep_containment,
            graph: Graph::new(),
            containments: HashMap::new(),
            test_containment: filter::Containment::new(internal_threshold),
            test_internalmatch: filter::InternalMatch::new(internal_threshold),
            node2index: HashMap::new(),
        }
    }

    pub fn add(self: &mut Self, record: &io::MappingRecord) {
        if self.test_internalmatch.run(&*record) {
            self.add_internalmatch(record);
        } else if self.test_containment.run(&*record) {
            self.add_containment(record);
        } else {
            self.add_dovetails(record);
        }
    }

    fn add_containment(self: &mut Self, record: &io::MappingRecord) {
        if record.strand() == '+' {
            if record.begin_a() <= record.begin_b() &&
                record.len_to_end_a() < record.len_to_end_b()
            {
                // B contain A
                self.containments.insert(
                    (record.read_a(), record.length_a()),
                    (
                        record.read_b(),
                        '+',
                        record.length_b(),
                        record.read_a(),
                        '+',
                        record.length_a(),
                        record.begin_b(),
                        record.length(),
                    ),
                );
            } else if record.begin_a() >= record.begin_b() &&
                       record.len_to_end_a() > record.len_to_end_b()
            {
                // A contain B
                self.containments.insert(
                    (record.read_b(), record.length_b()),
                    (
                        record.read_a(),
                        '+',
                        record.length_a(),
                        record.read_b(),
                        '+',
                        record.length_b(),
                        record.begin_a(),
                        record.length(),
                    ),
                );
            } else {
                println!(
                    "Containment Record not managed {:?} {:?}",
                    record.read_a(),
                    record.read_b()
                );
            }
        } else {
            if record.begin_a() <= record.len_to_end_b() &&
                record.len_to_end_a() < record.begin_b()
            {
                // B contain A
                self.containments.insert(
                    (record.read_a(), record.length_a()),
                    (
                        record.read_b(),
                        '+',
                        record.length_b(),
                        record.read_a(),
                        '-',
                        record.length_a(),
                        record.begin_b(),
                        record.length(),
                    ),
                );
            } else if record.begin_a() >= record.len_to_end_b() &&
                       record.len_to_end_a() > record.begin_b()
            {
                // A contain B
                self.containments.insert(
                    (record.read_b(), record.length_b()),
                    (
                        record.read_a(),
                        '+',
                        record.length_a(),
                        record.read_b(),
                        '-',
                        record.length_b(),
                        record.begin_a(),
                        record.length(),
                    ),
                );
            } else {
                println!(
                    "Containment Record not managed {:?} {:?}",
                    record.read_a(),
                    record.read_b()
                );
            }
        }
    }

    fn add_internalmatch(self: &mut Self, record: &io::MappingRecord) {
        if self.keep_internal {
            self.add_dovetails(record);
        }
    }

    fn add_dovetails(self: &mut Self, record: &io::MappingRecord) {
        let node_a = self.add_node((record.read_a(), record.length_a()));
        let node_b = self.add_node((record.read_b(), record.length_b()));


        if record.strand() == '+' {
            if record.begin_a() > record.begin_b() {
                // A overlap B
                self.add_edge(node_a, node_b, (
                    record.read_a(),
                    '+',
                    record.read_b(),
                    '+',
                    record.length(),
                ));
            } else {
                // B overlap A
                self.add_edge(node_b, node_a, (
                    record.read_b(),
                    '+',
                    record.read_a(),
                    '+',
                    record.length(),
                ));
            }
        } else {
            if record.begin_a() > record.len_to_end_a() {
                if record.begin_a() > record.len_to_end_b() {
                    // A overlap B
                    self.add_edge(node_a, node_b, (
                        record.read_a(),
                        '+',
                        record.read_b(),
                        '-',
                        record.length(),
                    ));
                } else {
                    // B overlap Af
                    self.add_edge(node_b, node_a, (
                        record.read_b(),
                        '+',
                        record.read_a(),
                        '-',
                        record.length(),
                    ));
                }
            } else {
                if (record.length_a() - record.begin_a()) > record.end_b() {
                    // A overlap B
                    self.add_edge(node_a, node_b, (
                        record.read_a(),
                        '-',
                        record.read_b(),
                        '+',
                        record.length(),
                    ));
                } else {
                    // B overlap A
                    self.add_edge(node_b, node_a, (
                        record.read_b(),
                        '-',
                        record.read_a(),
                        '+',
                        record.length(),
                    ));
                }
            }
        }
    }

    pub fn write<W: std::io::Write>(self: &mut Self, writer: &mut W) {
        if !self.keep_containment {
            let remove_key: Vec<((String, u64), ContainmentType)> =
                self.containments.drain().collect();
            for (key, _) in remove_key {
                let index = self.add_node(key.clone());
                self.graph.remove_node(index);
            }
        }

        writer.write_all(b"H\tVN:Z:1.0\n").expect(
            "Error durring gfa1 write",
        );

        let mut writed = HashSet::new();
        for (read_a, _, len_a, read_b, _, len_b, _, _) in self.containments.values() {
            if !writed.contains(&(read_a, len_a)) {
                writer
                    .write_fmt(format_args!("S\t{}\t*\tLN:i:{}\n", read_a, len_a))
                    .expect("Error durring gfa1 write");

                writed.insert((read_a, len_a));
            }

            if !writed.contains(&(read_b, len_b)) {
                writer
                    .write_fmt(format_args!("S\t{}\t*\tLN:i:{}\n", read_b, len_b))
                    .expect("Error durring gfa1 write");
                writed.insert((read_b, len_b));
            }
        }

        for node in self.graph.node_indices() {
            if self.graph.neighbors_undirected(node).count() != 0 {
                let segment = self.graph.node_weight(node).unwrap();
                if writed.contains(&(&segment.0, &segment.1)) {
                    continue;
                }

                writer
                    .write_fmt(format_args!("S\t{}\t*\tLN:i:{}\n", segment.0, segment.1))
                    .expect("Error durring gfa1 write");
            }
        }

        for edge in self.graph.edge_references() {
            writer
                .write_fmt(format_args!(
                    "L\t{}\t{}\t{}\t{}\t{}M\n",
                    edge.weight().0,
                    edge.weight().1,
                    edge.weight().2,
                    edge.weight().3,
                    edge.weight().4
                ))
                .expect("Error durring gfa1 write");
        }

        for value in self.containments.values() {
            writer
                .write_fmt(format_args!(
                    "C\t{}\t{}\t{}\t{}\t{}\t{}M\n",
                    value.0,
                    value.1,
                    value.3,
                    value.4,
                    value.6,
                    value.7
                ))
                .expect("Error durring gfa1 write");
        }
    }

    fn add_node(self: &mut Self, node: (String, u64)) -> petgraph::graph::NodeIndex {
        return if self.node2index.contains_key(&node) {
            *self.node2index.get(&node).expect("Impossible")
        } else {
            let index = self.graph.add_node(node.clone());
            self.node2index.insert(node, index);
            index
        };
    }

    fn add_edge(self: &mut Self, node_a: NodeIndex, node_b: NodeIndex, new_edge: LineType) {
        let edge = self.graph.find_edge(node_a, node_b);

        if edge.is_some() {
            let e = edge.unwrap();
            if self.graph.edge_weight(e).unwrap().4 < new_edge.4 {
                self.graph.update_edge(node_a, node_b, new_edge);
            } else {
                return;
            }
        } else {
            self.graph.add_edge(node_a, node_b, new_edge);
        }
    }
}

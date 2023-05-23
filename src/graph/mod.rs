use crate::directus::Field;
use crate::entities::directus_permissions;
use petgraph::dot::{Config, Dot};
use petgraph::graph::Graph;
use petgraph::prelude::NodeIndex;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

pub fn build_graph(
    permissions: Vec<directus_permissions::Model>,
    fields: &[Field],
) -> Graph<Node, EdgeType> {
    let mut graph = Graph::new();
    let mut nodes: HashMap<Node, NodeIndex> = HashMap::new();
    let mut edges: HashSet<(NodeIndex, NodeIndex, EdgeType)> = HashSet::new();

    for row in permissions.into_iter() {
        if let Some((new_nodes, new_edges)) = parse_row(row, fields) {
            for node in new_nodes {
                get_or_create_node(node, &mut nodes, &mut graph);
            }
            for (source_node, target_node, edge_type) in new_edges {
                let source_index = get_or_create_node(source_node, &mut nodes, &mut graph);
                let target_index = get_or_create_node(target_node, &mut nodes, &mut graph);
                add_unique_edge(
                    (source_index, target_index, edge_type),
                    &mut edges,
                    Some(&mut graph),
                );
            }
        }
    }
    graph
}

fn parse_row(row: directus_permissions::Model, all_fields: &[Field]) -> NodesAndEdges {
    let fields = row.fields?;
    let mut nodes = HashSet::new();
    let mut edges: Edges = HashSet::new();

    // Now we create the default Nodes that each row has
    let subject_node = Node::Subject(row.role.unwrap_or("Public".to_owned()));
    nodes.insert(subject_node.clone());
    let action_node = Node::Action(ActionType::from(row.action));
    nodes.insert(action_node.clone());

    // Adding an edge between Subject and Action
    edges.insert((subject_node, action_node.clone(), EdgeType::Allow));

    // TODO: Create Caveats
    // if row.permissions.is_some() {
    //     let mut caveat_nodes: HashMap<Node, NodeIndex> = HashMap::new();
    //     let mut caveat_edges: HashSet<(NodeIndex, NodeIndex, Edge)> = HashSet::new();
    //     let caveat_last_children = caveats_from_json(
    //         &mut caveat_nodes,
    //         &mut caveat_edges,
    //         &row.permissions.unwrap(),
    //         NodeIndex::new(0),
    //     );
    // }

    // Creating all resource nodes and connecting them to actions
    // FIXME: This should AKCTUALLY! connect to the caveats, which we should have
    // created first.
    let resources = create_resource_nodes(&fields, &row.collection, all_fields);
    resources.iter().for_each(|res| {
        nodes.insert(res.clone());
        edges.insert((action_node.clone(), res.to_owned(), EdgeType::Allow));
    });

    Some((nodes, edges))
}

/// Create all Resource nodes for a directus_permissions row
fn create_resource_nodes(fields: &str, collection: &str, all_fields: &[Field]) -> Vec<Node> {
    let nodes = match fields {
        "*" => all_fields
            .iter()
            .filter(|f| f.collection == collection)
            .cloned()
            .map(|field| {
                Node::Resource(Resource {
                    collection: collection.to_owned(),
                    field: field.field,
                })
            })
            .collect(),
        _ => fields
            .split(',')
            .map(|field_name| {
                Node::Resource(Resource {
                    collection: collection.to_owned(),
                    field: field_name.to_owned(),
                })
            })
            .collect(),
    };
    nodes
}

/// Build a hierarchical graph out of a Directus Filter Json
///
/// Recursively resolve objects into a graph structure.
/// The rules are:
/// * Children of an `_and` object will be connected in a chained
/// * Children of an `_or` object will be siblings
///
/// Mutates `nodes` and `edges` in place.
/// Returns a vector of sinks or leaves
///
/// # Panics
/// If the Json structure doesn't comply with Directus Filter Syntax
/// as outlined in the [Directus Documentation about Filter Rules](https://docs.directus.io/reference/filter-rules.html#filter-rules)
///

// fn caveats_from_json(
//     nodes: &mut HashMap<Node, NodeIndex>,
//     edges: &mut HashSet<(NodeIndex, NodeIndex, Edge)>,
//     json: &JsonValue,
//     parent: NodeIndex,
// ) -> Vec<NodeIndex> {
//     match json {
//         JsonValue::Object(o) => {
//             let mut leaf_nodes = Vec::new();
//             if let Some(_and_value) = o.get("_and") {
//                 let mut last_node = parent;
//                 for sub_value in _and_value
//                     .as_array()
//                     .expect("_and value should be an array")
//                 {
//                     let sub_leaf_nodes = caveats_from_json(nodes, edges, sub_value, last_node);
//                     if let Some(last_sub_leaf_node) = sub_leaf_nodes.last() {
//                         last_node = *last_sub_leaf_node;
//                     }
//                     leaf_nodes.extend(sub_leaf_nodes);
//                 }
//             } else if let Some(_or_value) = o.get("_or") {
//                 for sub_value in _or_value.as_array().expect("_or value should be an array") {
//                     let sub_leaf_nodes = caveats_from_json(nodes, edges, sub_value, parent);
//                     leaf_nodes.extend(sub_leaf_nodes);
//                 }
//             } else {
//                 let caveat_data = Node::Caveat(JsonValue::Object(o.clone()));
//                 let caveat_node = get_or_create_node(nodes, caveat_data);
//                 add_unique_edge((parent, caveat_node, Edge::Allow), edges);
//                 leaf_nodes.push(caveat_node);
//             }
//             leaf_nodes
//         }
//         _ => {
//             panic!("Json should be an object, got: {}", json);
//         }
//     }
// }

// Helper function to add a unique edge to `edges` and optionally to `graph`
fn add_unique_edge(
    edge: (NodeIndex, NodeIndex, EdgeType),
    edges: &mut HashSet<(NodeIndex, NodeIndex, EdgeType)>,
    graph: Option<&mut Graph<Node, EdgeType>>,
) {
    if !edges.contains(&edge) {
        if let Some(graph) = graph {
            graph.add_edge(edge.0, edge.1, edge.2);
        }
        edges.insert(edge);
    }
}

fn get_or_create_node(
    node_data: Node,
    nodes: &mut HashMap<Node, NodeIndex>,
    graph: &mut Graph<Node, EdgeType>,
) -> NodeIndex {
    *nodes
        .entry(node_data.clone())
        .or_insert_with(|| graph.add_node(node_data))
}

type Nodes = HashSet<Node>;
type Edge = (Node, Node, EdgeType);
type Edges = HashSet<Edge>;
type NodesAndEdges = Option<(Nodes, Edges)>;

/// The different types a node can have in our permissions system
///
/// The possibilities are:
/// * Subject -> Role ID
/// * Action -> JsonValue of a directus_permissions.validation filter
/// * Caveat -> JsonValue of a directus_permissions.permissions rule
/// * Resource -> A precise field address represented by a collection
///     and a field.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Subject(String),
    Action(ActionType),
    Caveat(JsonValue),
    Resource(Resource),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Copy)]
pub enum EdgeType {
    Allow,
    Forbid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
    Share,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    collection: String,
    field: String,
}

impl From<String> for ActionType {
    fn from(string: String) -> Self {
        match string.as_str() {
            "create" => Self::Create,
            "read" => Self::Read,
            "update" => Self::Update,
            "delete" => Self::Delete,
            "share" => Self::Share,
            error => panic!(
                "Actions can be one of `create`, `read`, `update`, `delete`, `share`. You used {}",
                error
            ),
        }
    }
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EdgeType::Allow => write!(f, ""),
            EdgeType::Forbid => write!(f, "Forbid"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Subject(ref s) => write!(f, "Subject({})", s),
            Node::Action(ref a) => write!(f, "Action({})", a),
            Node::Caveat(ref jv) => write!(f, "Caveat({})", jv),
            Node::Resource(ref r) => write!(
                f,
                "Resource( collection:\"{}\", field: \"{}\")",
                r.collection, r.field
            ),
        }
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionType::Create => write!(f, "Create"),
            ActionType::Read => write!(f, "Read"),
            ActionType::Update => write!(f, "Update"),
            ActionType::Delete => write!(f, "Delete"),
            ActionType::Share => write!(f, "Share"),
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Node::Subject(s) => {
                "Subject".hash(state);
                s.hash(state);
            }
            Node::Action(a) => {
                "Action".hash(state);
                a.hash(state);
            }
            Node::Caveat(c) => {
                "Caveat".hash(state);
                // FIXME: still find the best way handle this. Now we naively assume
                // that the json will always be in the right order. (Although that might
                // not be such a bad assumption in the case of Directus).
                c.to_string().hash(state);
            }
            Node::Resource(r) => {
                "Resource".hash(state);
                r.hash(state);
            }
        }
    }
}

impl Hash for EdgeType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            EdgeType::Allow => {
                "allow".hash(state);
            }
            EdgeType::Forbid => {
                "forbid".hash(state);
            }
        }
    }
}

// Display the Nodes nicely in a graphviz Dot graph
// Consider if there might be a better place for this
// in a different module
struct NodeWrapper<'a>(&'a Node);
struct EdgeWrapper<'a>(&'a EdgeType);

impl<'a> fmt::Display for NodeWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Node::Resource(r) => write!(f, "{}.{}", r.collection, r.field),
            Node::Action(a) => write!(f, "{}", a.to_string().to_uppercase()),
            Node::Subject(s) => write!(f, "{}", s),
            Node::Caveat(c) => write!(f, "{}", c),
        }
    }
}

impl<'a> fmt::Display for EdgeWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            EdgeType::Allow => write!(f, ""),
            EdgeType::Forbid => write!(f, "Forbid"),
        }
    }
}

pub trait GraphToString {
    fn draw(&self);
}

impl GraphToString for Graph<Node, EdgeType> {
    fn draw(&self) {
        let binding = self.map(|_, node| NodeWrapper(node), |_, edge| EdgeWrapper(edge));
        let dot = Dot::with_config(&binding, &[Config::EdgeNoLabel]);
        println!("{}", dot)
    }
}

#[cfg(test)]
mod test;

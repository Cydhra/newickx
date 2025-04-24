mod tokenizer;

pub mod tree;

pub mod parser;

/// A trait for building tree structures.
/// The trait is used by the [`Parser`] to create trees from newick data.
/// Implementations of the trait allow the parser to create different tree data structures.
pub trait TreeBuilder {
    /// The tree structure that will be built by the builder.
    type Tree;

    /// The node ID type used to identify nodes in the tree.
    type NodeId: Clone;
    
    /// Build an empty tree structure and reset the builder to its initial state.
    fn build(&mut self) -> Self::Tree;

    /// Add a node to the tree. It will not be connected to the tree yet.
    /// The node ID is returned, which can be used to uniquely identify the node in the tree.
    /// The node ID of a node must not change once the node has been added to the tree.
    fn add_node(&mut self, label: Option<String>) -> Self::NodeId;

    /// Add an edge between two existing nodes in the tree.
    /// The assignment of parent and child is arbitrary if the tree is unrooted.
    /// If the tree is rooted, the parent must be closer to the root than the child.
    /// An edge can only be added between two nodes that are already part of the tree.
    fn add_edge(&mut self, parent: Self::NodeId, child: Self::NodeId, support: Option<f64>, branch_length: Option<f64>);
}

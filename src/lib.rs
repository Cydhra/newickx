mod tokenizer;

pub mod tree;
mod parser;

pub trait TreeBuilder {
    type Tree;

    type NodeId: Clone;
    
    /// Build an empty tree structure and reset the builder to its initial state.
    fn build(&mut self) -> Self::Tree;

    /// Add a node to the tree. It will not be connected to the tree yet.
    fn add_node(&mut self, label: Option<String>) -> Self::NodeId;

    /// Add an edge between two existing nodes in the tree.
    /// The assignment of parent and child is arbitrary if the tree is unrooted.
    /// If the tree is rooted, the parent must be closer to the root than the child.
    fn add_edge(&mut self, parent: Self::NodeId, child: Self::NodeId, support: Option<f64>, branch_length: Option<f64>);
}

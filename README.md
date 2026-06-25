# Flexible Newick Parser
A minimal newick parser designed for maximum flexibility.
By default, it constructs a simple adjacency-list-based tree structure.
This built-in tree structure is lightweight (no overhead that is not required for Newick), implements `Send` and `Sync`, and fully supports modification.
But the parser can also be used to construct arbitrary types using a simple builder trait.
The parser is a fast LL(1) single-pass parser without recursion for speed and lower memory footprint.

A serializer is provided to convert trees back into Newick format. 
The same philosophy of flexibility is applied here, allowing downstream crates to control certain ambiguities in the Newick format.

# Usage
In the simplest case, construct a `Parser` with a `SimpleTreeBuilder` and call `parse()` until `Ok(None)` is returned,
to parse all trees in the input to `NTree` instances.

```rs
let newick = "(A, B, (D, E):0.2)The_Root;";
let mut parser = Parser::new(newick.as_bytes(), SimpleTreeBuilder::new());
let tree: Result<Option<NTree>> = parser.parse();
```

Multiple defaults can be changed with a `Settings` instance.
For example, the translation of underscores in labels to spaces can be disabled:
```rs
let mut parser = Parser::with_settings(
    newick.as_bytes(),
    SimpleTreeBuilder::new(),
    Settings::default().translate_underscores(false)
);
```

The `SimpleTreeBuilder` constructs `NTree` instances which is a simple tree structure based on doubly-linked Nodes.
It does not calculate or store extraneous information, and is designed to be a lightweight and flexible tree structure.

An optional optimization can be enabled with the crate feature `smallvec`, which optimizes the tree structure for binary trees using the smallvec crate.

If you want to parse Newick into your own tree structure, simply implement the `TreeBuilder` trait and give the parser an instance of your implementation instead of a `SimpleTreeBuilder`. 
The analogue `TreeSerialize` trait enables serialization using the built-in `Serializer`.
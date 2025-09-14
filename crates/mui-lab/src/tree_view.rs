//! Simple hierarchical tree structure.
//!
//! The `TreeView` type models expansion state and is intended as a foundation
//! for richer UI representations. Rendering is not handled here so that
//! alternative front ends (e.g. Yew, Leptos) can build on top without pulling in
//! extra dependencies. The API is intentionally tiny to keep tests fast and the
//! mental model small.

/// Node within a tree.
#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    /// Value stored at this node.
    pub value: T,
    /// Child nodes in insertion order.
    pub children: Vec<TreeNode<T>>,
    /// Whether this node's children are visible. Defaults to `false`.
    pub expanded: bool,
}

impl<T> TreeNode<T> {
    /// Creates a new leaf node with the given value.
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::new(),
            expanded: false,
        }
    }

    /// Toggles the node's expansion state.
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }
}

/// Convenience wrapper representing a full tree.
#[derive(Debug, Clone)]
pub struct TreeView<T> {
    /// Root node of the tree. Applications can traverse and mutate this
    /// directly for more complex scenarios.
    pub root: TreeNode<T>,
}

impl<T> TreeView<T> {
    /// Creates a new tree from a root node.
    pub fn new(root: TreeNode<T>) -> Self {
        Self { root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_flips_expanded_state() {
        let mut node = TreeNode::new(1);
        assert!(!node.expanded);
        node.toggle();
        assert!(node.expanded);
    }
}

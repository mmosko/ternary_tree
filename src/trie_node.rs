use std::rc::Rc;

/// This type is only used in Trie.  We box it for use in
///  the TrieNode left, middle, right pointers.
pub (super) type TrieNodeType = Box<TrieNode>;

/// This type is used outside of the module
pub type TrieValueType = Rc<Vec<u8>>;

#[derive(Clone, Debug, PartialEq)]
pub (super) struct TrieNode {
    pub left: Option<TrieNodeType>,
    pub middle: Option<TrieNodeType>,
    pub right: Option<TrieNodeType>,
    pub value: Option<TrieValueType>,
    pub key: u8,

    /// Number of ties fetched
    pub uses: usize,
}

impl std::fmt::Display for TrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tst[{:x}, {:?}, ({:?}, {:?}, {:?})]",
            self.key, self.value, self.left, self.middle, self.right
        )
    }
}

impl TrieNode {
    pub fn new(key: &u8, value: Option<TrieValueType>) -> TrieNodeType {
        Box::new(TrieNode {
            left: None,
            middle: None,
            right: None,
            value: value.clone(),
            key: *key,
            uses: 0,
        })
    }
}

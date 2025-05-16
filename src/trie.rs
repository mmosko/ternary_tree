use super::trie_node::{TrieNode, TrieNodeType, TrieValueType};

/// A ternary search trie
/// Based on Sedgewick.
/// See "Ternary Search Trees" by Jon Bentley and Robert Sedgewick
/// in the April, 1998, Dr. Dobb's Journal.
/// 
/// Each TST node has a 1 byte key.  This is matched byte-by-byte with
/// some input string.
/// 
/// Each TST node has a dictionary `value` field that is used in the compressed
/// output version of the string.  A TST node may have a None value if it
/// is not associated with a dictionary key.
pub struct Trie {
    pub root: Option<TrieNodeType>,
    size: usize
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: None, size: 0 }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    /// Inserts a value into the trie for the token string.
    /// 
    /// If there is already a value for the token string, panics.
    pub fn insert(&mut self, tokens: &[u8], value: &TrieValueType) {
        self.root = self.recursive_insert(&mut self.root.clone(), tokens, 0, value);
    }

    fn recursive_insert(&mut self, node: &mut Option<TrieNodeType>, tokens: &[u8], offset: usize, value: &TrieValueType) -> Option<TrieNodeType> {
        let key = tokens[offset];

        let node = match node {
            Some(n) => n,
            None => &mut TrieNode::new(&key, None)
        };
        
        if key < node.key {
            node.left = self.recursive_insert(&mut node.left, tokens, offset, value);
        } else if key > node.key {
            node.right = self.recursive_insert(&mut node.right, tokens, offset, value);
        } else {
            // middle path
            if (offset + 1) == tokens.len() {
                // last token
                if node.value.is_some() {
                    panic!("There is already a value at node {}", node);
                }
                node.value = Some(value.clone());
                self.size += 1;
            } else {
                node.middle = self.recursive_insert(&mut node.middle, tokens, offset+1, value);
            }
        }

        Some(node.clone())
    }

    /// Searches the trie for the token string and returns the value
    /// of the exact match node.  Will return None if not found.
    pub fn search(&mut self, tokens: &[u8]) -> Option<TrieValueType> {
        let mut node = &mut self.root;
        
        let mut offset: usize = 0;
        while offset < tokens.len() {
            let Some(box_node) = node else { return None };

            let key = tokens[offset];

            if key < box_node.key {
                node = &mut box_node.left;
            } else if key > box_node.key {
                node = &mut box_node.right;
            } else {
                // middle key
                offset += 1;
                if offset == tokens.len() {
                    box_node.uses += 1;
                    return box_node.value.clone();
                }
                node = &mut box_node.middle;
            }
        }
        return None;
    }

    /// Finds the longest matching string for tokens.
    /// 
    /// On success, returns the TrieValueType and the number of bytes consumed.
    /// 
    pub fn longest_match(&self, tokens: &[u8]) -> Option<(TrieValueType, usize)> {
        let mut longest_match: Option<(TrieValueType, usize)> = None;
        let mut longest_node: Option<TrieNodeType> = None;
        let mut node = &self.root;

        let mut offset: usize = 0;
        while offset < tokens.len() {
            let Some(box_node) = node else { 
                if let Some(mut longest_node) = longest_node {
                    longest_node.uses += 1;
                }
                return longest_match 
            };

            let key = tokens[offset];

            if key < box_node.key {
                node = &box_node.left;
            } else if key > box_node.key {
                node = &box_node.right;
            } else {
                // middle key
                offset += 1;
                if box_node.value.is_some() {
                    // get a clone of the Box out of the Option
                    let value = box_node.value.as_ref().unwrap().clone();
                    longest_match = Some((value, offset));
                    longest_node = node.clone();
                }
                node = &box_node.middle;
            }
        }

        if let Some(mut longest_node) = longest_node {
            longest_node.uses += 1;
        }
        return longest_match;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_empty_trie() {
        let mut t = Trie::new();
        let result = t.search(&[0, 1, 2]);
        assert!(result.is_none());
    }

    #[test]
    fn insert_empty_trie() {
        let mut t = Trie::new();
        let value= TrieValueType::new(vec![2, 3]);
        let key = [5u8];

        t.insert(&key, &value);

        let actual = t.search(&key);
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), value);

        let actual = t.search(&[5, 4]);
        assert!(actual.is_none());
    }

        #[test]
    fn insert_multi_byte() {
        let mut t = Trie::new();
        let value= TrieValueType::new(vec![2, 3]);
        let key = [5u8, 8u8, 9u8];

        t.insert(&key, &value);

        let actual = t.search(&key);
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), value);

        let actual = t.search(&[5, 4]);
        assert!(actual.is_none());
    }

    #[test]
    fn insert_many() {
        let vectors = vec![
            ("grapefruit", TrieValueType::new(vec![1u8])),
            ("grapes", TrieValueType::new(vec![2u8])),
            ("apple", TrieValueType::new(vec![3u8])),
            ("applesauce", TrieValueType::new(vec![4u8])),
            ("jelly", TrieValueType::new(vec![5u8])),
            ("yams", TrieValueType::new(vec![6u8]))
        ];

        let mut t = Trie::new();

        for (k, v) in &vectors {
            let key = k.as_bytes();
            t.insert(key, v);
        }

        assert_eq!(vectors.len(), t.len());
        

        for (k, v) in &vectors {
            let key = k.as_bytes();
            let actual = t.search(key);
            assert!(actual.is_some(), "Failed to find key {}", k);
            assert_eq!(actual.unwrap(), *v);
        }
    }

    #[test]
    fn test_longest_match() {
        let data = vec![
            ("abcdefgh", TrieValueType::new(vec![1u8])),
            ("abcd", TrieValueType::new(vec![2u8])),
        ];
        let vectors: Vec<(&str, Option<(TrieValueType, usize)>)> = vec![
            ("abcdefgh", Some((TrieValueType::new(vec![1u8]), 8))),
            ("abcd", Some((TrieValueType::new(vec![2u8]), 4))),
            ("ab", None),
            ("abcde", Some((TrieValueType::new(vec![2u8]), 4))),
            ("abcdefghi", Some((TrieValueType::new(vec![1u8]), 8)))
        ];

        let mut t = Trie::new();

        for (k, v) in &data {
            let key = k.as_bytes();
            t.insert(key, v);
        }

        for (k, v) in &vectors {
            let key = k.as_bytes();
            let actual = t.longest_match(key);
            assert_eq!(actual, *v, "Failed for key {}", k);
        }        
    }

}


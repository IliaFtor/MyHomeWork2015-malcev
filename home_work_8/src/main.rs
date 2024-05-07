use std::rc::Rc;

struct SuffixTreeNode {
    children: Vec<Option<Rc<SuffixTreeNode>>>,
    start: usize,
    end: usize,
}

impl SuffixTreeNode {
    fn new(start: usize, end: usize) -> Self {
        let children = vec![None; 256];
        SuffixTreeNode { children, start, end }
    }

    fn fill_leaves(&mut self, text: &str) {
        let mut start = self.start;
        let mut end = self.end;
        let mut current_node = self;

        while start <= end {
            let mut leaf_start = start;
            let mut i = start;

            while i <= end {
                let c = text.as_bytes()[i];
                if current_node.children[c as usize].is_none() {
                    let new_node = Rc::new(SuffixTreeNode::new(leaf_start, i));
                    current_node.children[c as usize] = Some(new_node.clone());
                }

                if let Some(child) = current_node.children[c as usize].as_ref() {
                    if let Ok(child_inner) = Rc::try_unwrap(child.clone()) {
                        *current_node = child_inner;
                        break;
                    }
                }

                leaf_start += 1;
                i += 1;
            }

            start += 1;
        }
    }
}

fn build_suffix_tree(text: &str) -> SuffixTreeNode {
    let mut root = SuffixTreeNode::new(0, text.len() - 1);
    root.fill_leaves(text);
    root
}

fn print_suffix_tree(node: &SuffixTreeNode, level: usize) {
    for (i, child) in node.children.iter().enumerate() {
        if let Some(child_node) = child {
            println!("{:width$}{}", "", i, width = level);
            print_suffix_tree(child_node, level + 1);
        }
    }
}

fn main() {
    let text = "banana";
    let suffix_tree = build_suffix_tree(text);
    print_suffix_tree(&suffix_tree, 0);
}

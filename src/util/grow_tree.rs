use std::fmt;

#[derive(Debug)]
pub struct GrowTree<T> {
    all_items: Vec<T>,
    parent_indices: Vec<Option<usize>>,
    child_indices: Vec<Vec<usize>>
}

impl<T> GrowTree<T> {
    pub fn new(root: T) -> GrowTree<T> {
        GrowTree {
            all_items: vec![root],
            child_indices: vec![vec![]],
            parent_indices: vec![None],
        }
    }

    fn add_item(&mut self, item: T) -> usize {
        let index = self.all_items.len();
        self.all_items.push(item);
        self.child_indices.push(vec![]);
        index
    }

    pub fn add_child(&mut self, item: T, parent_index: usize) -> usize {
        if parent_index >= self.all_items.len() {
            panic!("Invalid parent_index: {}", parent_index);
        }
        let index = self.add_item(item);
        self.child_indices[parent_index].push(index);
        self.parent_indices.push(Some(parent_index));
        index
    }

    pub fn item(&self, index: usize) -> Option<&T> {
        self.all_items.get(index)
    }

    pub fn item_mut(&mut self, index: usize) -> Option<&mut T> {
        self.all_items.get_mut(index)
    }

    pub fn root(&self) -> &T {
        &self.all_items[0]
    }

    pub fn children(&self, parent_index: usize) -> Vec<(usize, &T)> {
        self.child_indices.get(parent_index).unwrap_or(&vec![]).iter().map(|index| (*index, &self.all_items[*index])).collect()
    }

    pub fn parent(&self, index: usize) -> Option<(usize, &T)> {
        self.parent_indices.get(index).unwrap_or(&None).map(|index| (index, &self.all_items[index]))
    }

    pub fn subtree<'a>(&'a self, index: usize) -> Option<GrowTree<&'a T>> {
        self.as_refs().into_subtree(index)
    }

    pub fn map<F, O>(&self, f: F) -> GrowTree<O>
        where F: FnMut(&T) -> O {
        GrowTree {
            all_items: self.all_items.iter().map(f).collect(),
            child_indices: self.child_indices.clone(),
            parent_indices: self.parent_indices.clone(),
        }
    }

    pub fn as_refs(&self) -> GrowTree<&T> {
        GrowTree {
            all_items: self.all_items.iter().collect(),
            child_indices: self.child_indices.clone(),
            parent_indices: self.parent_indices.clone(),
        }
    }

    pub fn map_into<F, O>(self, f: F) -> GrowTree<O>
        where F: FnMut(T) -> O {
            GrowTree {
                all_items: self.all_items.into_iter().map(f).collect(),
                child_indices: self.child_indices.clone(),
                parent_indices: self.parent_indices.clone(),
            }
    }

    pub fn into_subtree(self, index: usize) -> Option<GrowTree<T>> {
        if self.all_items.len() <= index {
            return None;
        }

        let mut source_tree = self.map_into(|item| Some(item));

        let mut subtree = GrowTree::new(std::mem::replace(&mut source_tree.all_items[index], None).unwrap());
        for child_index in source_tree.child_indices[index].clone() {
            subtree.import_tree(&mut source_tree, child_index, 0);
        }
        Some(subtree)
    }

    pub fn add_tree_into(&mut self, parent_index: usize, tree: GrowTree<T>) {
        let mut source_tree = tree.map_into(|item| Some(item));

        self.import_tree(&mut source_tree, 0, parent_index);
    }

    fn import_tree(&mut self, source: &mut GrowTree<Option<T>>, source_index: usize, target_parent_index: usize) {
        let item = std::mem::replace(&mut source.all_items[source_index], None).unwrap();
        let target_index = self.add_child(item, target_parent_index);
        let source_child_indices = source.child_indices[source_index].clone();
        for source_child_index in source_child_indices {
            self.import_tree(source, source_child_index, target_index);
        }
    }
}

impl<T> fmt::Display for GrowTree<T>
  where T: fmt::Display {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn write_items<T>(f: &mut fmt::Formatter, tree: &GrowTree<T>, items: Vec<(usize, &T)>, prefix: &str) -> fmt::Result
            where T: fmt::Display {
            let len = items.len();
            for (index, item) in items {
                writeln!(f, "{}-- {}", prefix, item)?;
                let children = tree.children(index);
                write_items(f, tree, children, &format!("{} |", prefix))?
            }
            Ok(())
        }

        writeln!(f, "GrowTree {{")?;
        write_items(f, self, vec![(0, &self.all_items[0])], "  ")?;
        writeln!(f, "}}")
    }
}

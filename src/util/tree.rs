use std::fmt;
use std::collections::{ HashMap };
use std::hash::{ Hash };

pub trait Tree {
    type TreeItem;
    type Index: Copy + Eq + Hash;

    fn children<'a>(&'a self, parent: Option<Self::Index>) -> Vec<(Self::Index, &'a Self::TreeItem)>;

    fn parent<'a>(&'a self, index: Self::Index) -> Option<(Self::Index, &'a Self::TreeItem)>;

    fn into_hierarchy(self) -> Vec<(Self::Index, Option<Self::Index>, Self::TreeItem)>;
}

pub trait TreeInsert: Tree {
    fn add(&mut self, item: Self::TreeItem, parent: Option<Self::Index>) -> Self::Index;

    fn add_tree<U: Tree<TreeItem=Self::TreeItem>>(&mut self, tree: U, parent: Option<Self::Index>) -> Vec<Self::Index> {
        let mut indices = vec![];
        let mut index_mapping = HashMap::new();

        for (index, opt_parent_index, item) in tree.into_hierarchy() {
            if let Some(parent_index) = opt_parent_index {
                if let Some(&target_parent_index) = index_mapping.get(&parent_index) {
                    let target_index = self.add(item, Some(target_parent_index));
                    index_mapping.insert(index, target_index);
                    indices.push(target_index);
                }
            } else {
                // A root item.
                let target_index = self.add(item, parent);
                index_mapping.insert(index, target_index);
                indices.push(target_index);
            }
        }

        indices
    }
}

pub struct VecTree<T> {
    items: Vec<T>,
    parent_indices: Vec<Option<usize>>,
    children_indices: Vec<Vec<usize>>,
    root_indices: Vec<usize>
}

impl<T> VecTree<T> {
    pub fn new() -> VecTree<T> {
        VecTree {
            items: vec![],
            parent_indices: vec![],
            children_indices: vec![],
            root_indices: vec![]
        }
    }
}

impl<T> Tree for VecTree<T> {
    type TreeItem = T;
    type Index = usize;

    fn children<'a>(&'a self, parent: Option<usize>) -> Vec<(Self::Index, &'a Self::TreeItem)> {
        let indices = match parent {
            Some(parent_index) => &self.children_indices[parent_index],
            None => &self.root_indices
        };

        indices.iter().map(|&child_index| (child_index, &self.items[child_index])).collect()
    }

    fn parent<'a>(&'a self, index: Self::Index) -> Option<(Self::Index, &'a Self::TreeItem)> {
        match self.parent_indices.get(index) {
            Some(Some(index)) => Some((*index, &self.items[*index])),
            _ => None
        }
    }

    fn into_hierarchy(self) -> Vec<(Self::Index, Option<Self::Index>, Self::TreeItem)> {
        let parent_indices = self.parent_indices.clone();
        self.items.into_iter().enumerate().map(|(index, item)| (index, parent_indices[index], item)).collect()
    }
}

impl<T> TreeInsert for VecTree<T> {
    fn add(&mut self, item: T, parent: Option<Self::Index>) -> Self::Index {
        let index = self.items.len();
        self.items.push(item);
        self.parent_indices.push(parent);
        self.children_indices.push(vec![]);
        if let Some(parent_index) = parent {
            self.children_indices[parent_index].push(index);
        } else {
            self.root_indices.push(index);
        }
        index
    }
}

impl<T> fmt::Display for VecTree<T>
  where T: fmt::Display {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn write_items<T>(f: &mut fmt::Formatter, tree: &VecTree<T>, items: Vec<(usize, &T)>, prefix: &str) -> fmt::Result
            where T: fmt::Display {
            for (index, item) in items {
                writeln!(f, "{}-- {}", prefix, item)?;
                let children = tree.children(Some(index));
                write_items(f, tree, children, &format!("{} |", prefix))?
            }
            Ok(())
        }

        writeln!(f, "VecTree {{")?;
        write_items(f, self, self.children(None), "  ")?;
        writeln!(f, "}}")
    }
}

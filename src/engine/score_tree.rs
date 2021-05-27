use std::fmt;
use std::collections::{ HashMap };

#[derive(Debug)]
pub struct ScoreTree {
    score_items: Vec<ScoreItem>,
    child_indices: Vec<Vec<usize>>
}

#[derive(Debug,Copy,PartialEq,Eq,Clone,Hash)]
pub enum ScoreTarget {
    Highest,
    Lowest
}

impl fmt::Display for ScoreTarget {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[derive(Debug,Copy,PartialEq,Eq,Clone,Hash)]
struct ScoreItem {
    local_score: i32,
    score_target: ScoreTarget,
    deep_score: Option<DeepScore>,
    parent_index: Option<usize>,
}

impl fmt::Display for ScoreItem {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "local: {}", self.local_score)?;
        if let Some(deep_score) = self.deep_score {
            write!(f, ", deep: {}", deep_score)?;
        }
        write!(f, ", target: {}", self.score_target)
    }
}

#[derive(Debug,Copy,PartialEq,Eq,Clone,Hash)]
struct DeepScore {
    score: i32,
    depth: i8,
}

impl fmt::Display for DeepScore {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.score, self.depth)
    }
}

impl ScoreTree {
    pub fn new() -> ScoreTree {
        ScoreTree {
            score_items: vec![],
            child_indices: vec![],
        }
    }

    pub fn add(&mut self, local_score: i32, score_target: ScoreTarget, parent_index: Option<usize>) -> Result<usize, &str> {
        let len = self.score_items.len();

        if let Some(index) = parent_index {
            if index >= len {
                return Err("parent_index is too large");
            }
        }

        self.score_items.push(ScoreItem {
            local_score,
            score_target,
            deep_score: None,
            parent_index,
        });
        self.child_indices.push(vec![]);

        // Add the child index.
        if let Some(parent_index) = parent_index {
            self.child_indices[parent_index].push(len);
        }

        self.remove_ancestor_deep_scores(len);

        Ok(len)
    }

    fn remove_ancestor_deep_scores(&mut self, index: usize) {
        let score_item = self.score_items[index];
        if let Some(parent_index) = score_item.parent_index {
            self.score_items[parent_index].deep_score = None;
            self.remove_ancestor_deep_scores(parent_index);
        }
    }

    pub fn score_at(&self, index: usize) -> Option<i32> {
        return Some(0)
    }

    pub fn subtree(&self, root_index: usize) -> ScoreTree {
        let mut sub_items = vec![];
        let mut sub_child_indices = vec![];

        fn add_sub_items(tree: &ScoreTree, parent_index: usize, &mut sub_items: Vec<ScoreItem>, &mut sub_child_indices: Vec<Vec<usize>>) {

        }

        let mut parent_mapping = HashMap::new();

        for (index, item) in self.score_items.iter().enumerate() {
            if index == root_index {
                parent_mapping.insert(index, None);
            } else {
                if let Some(parent_index) = item.parent_index {
                    if parent_mapping.contains_key(&parent_index) {
                        let mut new_item = item.clone();
                        new_item.parent_index = *parent_mapping.get(&parent_index).unwrap();
                        parent_mapping.insert(index, Some(sub_items.len()));
                        sub_items.push(new_item);
                    }
                }
            }
        }

        ScoreTree {
            score_items: sub_items
        }
    }
}

fn choose_score(local_score: i32, score_target: ScoreTarget, score1: DeepScore, score2: DeepScore) -> DeepScore {
    match score_target {
        ScoreTarget::Highest => {
            if score1.score > score2.score {
                score1
            } else if score2.score > score1.score {
                score2
            } else {
                // Scores are the same.
                if score1.score > local_score {
                    // The scores are higher than the local score. Choose the one that is closest (strike sooner).
                    if score1.depth > score2.depth { score2 } else { score1 }
                } else {
                    // The scores are lower than the local score. Choose the one that is farthest (delay losses).
                    if score2.depth > score1.depth { score2 } else { score1 }
                }
            }
        },
        ScoreTarget::Lowest => {
            if score2.score > score1.score {
                score1
            } else if score1.score > score2.score {
                score2
            } else {
                // Scores are the same.
                if score1.score > local_score {
                    // The scores are higher than the local score. Choose the one that is farthest (delay losses).
                    if score2.depth > score1.depth { score2 } else { score1 }
                } else {
                    // The scores are lower than the local score. Choose the one that is farthest (strike sooner).
                    if score1.depth > score2.depth { score2 } else { score1 }
                }
            }
        }
    }
}

impl fmt::Display for ScoreTree {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: make this efficient for large trees. This is O(n^2) but it's just for simple debugging at this stage.
        fn fmt_with_prefix(f: &mut fmt::Formatter, tree: &ScoreTree, prefix: &str) -> fmt::Result {
            for (index, item) in tree.score_items.iter().enumerate().filter(|(_, item)| item.parent_index.is_none()) {
                writeln!(f, "{}- {}", prefix, item)?;
                fmt_with_prefix(f, &tree.subtree(index), &format!("{}  |", prefix))?;
            }
            Ok(())
        }

        writeln!(f, "ScoreTree {{")?;
        fmt_with_prefix(f, self, "")?;
        writeln!(f, "}}")
    }
}

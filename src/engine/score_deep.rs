use std::cmp::{ Ordering };

use crate::board::{ Board, Color };
use crate::engine::{ is_checked, next_boards, score_board };

const CHECKMATE_SCORE: i32 = 1000;
const STALEMATE_SCORE: i32 = 0;

#[derive(Debug)]
struct Worker {
    work_items: Vec<WorkItem>,
    work_index: usize
}

#[derive(Debug,Copy,PartialEq,Eq,Clone,Hash)]
struct DeepScore {
    score: i32,
    depth: u8
}

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
struct WorkItem {
    board: Board,
    active_player: Color,
    score: DeepScore,
    parent_index: Option<usize>,
    depth: u8,
}

impl Worker {
    pub fn new(board: &Board, active_player: Color) -> Worker {
        Worker {
            work_items: vec![
                WorkItem {
                    board: *board,
                    active_player,
                    score: DeepScore { score: score_board(&board), depth: 0 },
                    parent_index: None,
                    depth: 0
                }
            ],
            work_index: 0
        }
    }

    pub fn step(&mut self) {
        if self.work_items.len() <= self.work_index {
            return;
        }

        let work_item = self.work_items[self.work_index];

        // Calculate all next moves.
        let boards = next_boards(&work_item.board, work_item.active_player);
        if boards.len() == 0 {
            // Checkmate or stalemate!
            // Mutate our work item.
            self.work_items[self.work_index].score = DeepScore {
                score: match (work_item.active_player, is_checked(&work_item.board, work_item.active_player)) {
                    (Color::White, true) => -1 * CHECKMATE_SCORE,
                    (Color::Black, true) => CHECKMATE_SCORE,
                    _ => STALEMATE_SCORE
                },
                depth: work_item.depth
            };
        }
        for board in boards {
            let depth = work_item.depth + 1;
            let new_work_item = WorkItem {
                board,
                active_player: work_item.active_player.opposite(),
                score: DeepScore { score: score_board(&board), depth },
                parent_index: Some(self.work_index),
                depth
            };
            self.work_items.push(new_work_item);
        }

        // Propagate our score upward.
        let mut opt_parent_index = work_item.parent_index;
        let mut score = self.work_items[self.work_index].score;
        while let Some(parent_index) = opt_parent_index {
            let parent = self.work_items[parent_index];
            let old_parent_score = parent.score;

            // TODO: I still think this part is the reason for the bug in test_score_deep().
            let new_parent_score = if parent.score.depth == parent.depth {
                // No sub boards have been analyzed for the parent board. Its score is just a basic board scoring.
                // But not moving is not an option! So just take our current score.
                score
            } else if self.work_items[parent_index].active_player == Color::Black {
                // Keep the score that is better for black.
                deep_score_min(score, old_parent_score)
            } else {
                // Keep the score that is better for white.
                deep_score_max(score, old_parent_score)
            };
            if old_parent_score == new_parent_score {
                // Nothing is going to change from here on out.
                break
            }
            self.work_items[parent_index].score = new_parent_score;
            score = new_parent_score;
            opt_parent_index = self.work_items[parent_index].parent_index;
        }

        // Move the pointer forward.
        self.work_index = self.work_index + 1;

        if opt_parent_index.is_none() {
            // We have changed the top-level score.
            // If this board definitely leads to a checkmate, no further analysis is needed.
            if self.work_items[0].score.score.abs() == CHECKMATE_SCORE {
                println!("Stop at checkmate");
                self.work_index = self.work_items.len();
            }
        }
    }

    pub fn score(&self) -> DeepScore {
        self.work_items[0].score
    }
}

pub fn scored_next_boards(board: &Board, active_player: Color, allow_continue: &mut impl FnMut() -> bool) -> Vec<(i32, Board)> {
    let boards = next_boards(board, active_player);

    // TODO: distribute workers over threads
    let mut workers: Vec<Worker> = boards.iter().map(|board| Worker::new(board, active_player.opposite())).collect();

    while allow_continue() {
        for worker in &mut workers {
            worker.step();
        }
    }

    let mut scores: Vec<(DeepScore, Board)> = workers.into_iter().map(|worker| worker.score()).zip(boards.into_iter()).collect();

    if active_player == Color::White {
        scores.sort_by(|(score_a, _), (score_b, _)| {
            if score_a == score_b {
                return Ordering::Equal;
            }
            return if deep_score_max(*score_a, *score_b) == *score_a {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    } else {
        scores.sort_by(|(score_a, _), (score_b, _)| {
            if score_a == score_b {
                return Ordering::Equal;
            }
            return if deep_score_min(*score_a, *score_b) == *score_a {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    };

    scores.into_iter().map(|(deep_score, board)| (deep_score.score, board)).collect()
}

fn deep_score_min(score1: DeepScore, score2: DeepScore) -> DeepScore {
    if score1.score < score2.score {
        score1
    } else if score1.score > score2.score {
        score2
    } else {
        closest_score(score1, score2)
    }
}

fn deep_score_max(score1: DeepScore, score2: DeepScore) -> DeepScore {
    if score1.score > score2.score {
        score1
    } else if score1.score < score2.score {
        score2
    } else {
        closest_score(score1, score2)
    }
}

fn closest_score(score1: DeepScore, score2: DeepScore) -> DeepScore {
    if score1.depth < score2.depth {
        score1
    } else {
        score2
    }
}

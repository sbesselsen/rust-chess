use std::cmp::{ max, min };

use crate::board::{ Board, Color };
use crate::engine::{ is_checked, next_boards, score_board };

const CHECKMATE_SCORE: i32 = 1000;
const STALEMATE_SCORE: i32 = 0;

#[derive(Debug)]
struct Worker {
    work_items: Vec<WorkItem>,
    work_index: usize
}

impl Worker {
    pub fn new(board: &Board, active_player: Color) -> Worker {
        Worker {
            work_items: vec![
                WorkItem {
                    board: *board,
                    active_player,
                    score: score_board(&board),
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
            // Checkmate!
            // Mutate our work item.
            if is_checked(&work_item.board, work_item.active_player) {
                self.work_items[self.work_index].score = if work_item.active_player == Color::White { -1 * CHECKMATE_SCORE } else { CHECKMATE_SCORE };
            } else {
                self.work_items[self.work_index].score = STALEMATE_SCORE;
            }
        }
        for board in boards {
            let new_work_item = WorkItem {
                board,
                active_player: work_item.active_player.opposite(),
                score: score_board(&board),
                parent_index: Some(self.work_index),
                depth: work_item.depth + 1
            };
            self.work_items.push(new_work_item);
        }

        // Propagate our score upward.
        let mut opt_parent_index = work_item.parent_index;
        let mut score = self.work_items[self.work_index].score;
        while let Some(parent_index) = opt_parent_index {
            let old_parent_score = self.work_items[parent_index].score;

            // TODO: I still think this part is the reason for the bug in test_score_deep().
            let new_parent_score = if self.work_items[parent_index].active_player == Color::Black {
                // The parent board has black as the active player. We want to see how bad it can get, so take the maximum score.
                min(score, old_parent_score)
            } else {
                max(score, old_parent_score)
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
            if self.work_items[0].score.abs() == CHECKMATE_SCORE {
                self.work_index = self.work_items.len();
            }
        }
    }

    pub fn score(&self) -> i32 {
        self.work_items[0].score
    }
}

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
struct WorkItem {
    board: Board,
    active_player: Color,
    score: i32,
    parent_index: Option<usize>,
    depth: u8,
}

pub fn scored_next_boards(board: &Board, active_player: Color, allow_continue: &mut impl FnMut() -> bool) -> Vec<(i32, Board)> {
    let boards = next_boards(board, active_player);

    // TODO: distribute workers over threads
    let mut workers: Vec<Worker> = boards.iter().map(|board| Worker::new(board, active_player)).collect();

    while allow_continue() {
        for worker in &mut workers {
            worker.step();
        }
    }

    let mut scores: Vec<(i32, Board)> = workers.into_iter().map(|worker| worker.score()).zip(boards.into_iter()).collect();

    scores.sort_by(|(score_a, _), (score_b, _)| score_a.cmp(score_b));
    if active_player == Color::White {
        scores.reverse();
    }
    scores
}

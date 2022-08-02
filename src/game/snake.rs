use super::{Coord, BOARD_SZ};
use crate::util::{Direction, Point};
use std::collections::VecDeque;

pub struct Snake {
    pub(super) direction: Direction,
    pub(super) segments: VecDeque<Point<Coord>>,
}
impl Snake {
    pub fn new(direction: Direction, length: usize) -> Self {
        let mut segments = VecDeque::new();
        let mut head = Point {
            x: BOARD_SZ.x / 2,
            y: BOARD_SZ.y / 2,
        };
        for _ in 0..length {
            segments.push_front(head);
            head.move_towards(direction);
            head.wrap(BOARD_SZ);
        }
        Self {
            direction,
            segments,
        }
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn random_unoccupied_point(&self) -> Point<Coord> {
        loop {
            let food = Point::random(BOARD_SZ);
            if self.segments.iter().all(|segment| *segment != food) {
                return food;
            }
        }
    }
}

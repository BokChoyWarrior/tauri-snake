use std::ops::Add;

use rand::{self, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub food: Position,
    pub snake: Snake,
    pub score: usize,
    pub lost: bool,
}

#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Snake {
    body: Vec<Position>,
    pub direction: Direction,
    queued_direction: Direction,
    did_eat: bool,
}

impl Snake {
    pub fn new(length: usize, head_pos: Position) -> Self {
        let mut body = vec![];
        let mut i = 1;
        while i <= length {
            body.push(Position {
                x: head_pos.x - i,
                y: head_pos.y,
            });
            i += 1;
        }
        Self {
            body,
            direction: Direction::Right,
            queued_direction: Direction::Right,
            did_eat: false,
        }
    }
    fn change_direction(&mut self) {
        self.direction = self.queued_direction;
    }

    pub fn queue_change_direction(&mut self, dir: Direction) {
        match (&self.direction, dir) {
            (Direction::Up, Direction::Left)
            | (Direction::Up, Direction::Right)
            | (Direction::Left, Direction::Up)
            | (Direction::Left, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Down, Direction::Right)
            | (Direction::Right, Direction::Up)
            | (Direction::Right, Direction::Down) => self.queued_direction = dir,
            (_, _) => {
                self.queued_direction = self.direction;
            }
        }
    }

    pub fn get_head(&self) -> Position {
        self.body[0]
    }

    pub fn tick(&mut self, food_pos: Position) {
        self.did_eat = false;
        let (x, y) = (self.body[0].x, self.body[0].y);
        self.change_direction();
        let new_head = match &self.direction {
            Direction::Up => Position { x, y: y - 1 },
            Direction::Left => Position { x: x - 1, y },
            Direction::Down => Position { x, y: y + 1 },
            Direction::Right => Position { x: x + 1, y },
        };
        self.body.insert(0, new_head);

        if new_head.eq(&food_pos) {
            self.did_eat = true;
        } else {
            self.body.pop();
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub enum Direction {
    Up,
    Left,
    Down,
    #[default]
    Right,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            food: Game::get_new_food_pos(width, height),
            snake: Snake::new(
                4,
                Position {
                    x: width / 2,
                    y: height / 2,
                },
            ),
            score: 0,
            lost: false,
        }
    }

    pub fn queue_change_direction(&mut self, dir: Direction) {
        self.snake.queue_change_direction(dir);
    }

    pub fn place_new_food(&mut self) {
        self.food = Game::get_new_food_pos(self.width, self.height);
    }

    pub fn get_new_food_pos(width: usize, height: usize) -> Position {
        let mut rng = rand::thread_rng();

        Position {
            x: rng.gen_range(1..=width - 2),
            y: rng.gen_range(1..=height - 2),
        }
    }

    pub fn tick(&mut self) {
        self.snake.tick(self.food);
        let new_head = self.snake.get_head();

        if self.snake.did_eat {
            self.score += 1;
            while self.snake.body.contains(&self.food) {
                self.place_new_food();
            }
        }

        if self.snake.body[1..].contains(&new_head)
            || new_head.x < 1
            || new_head.x > self.width - 2
            || new_head.y < 1
            || new_head.y > self.height - 2
        {
            self.lost = true;
        }
    }
}

use std::ops::Add;

use rand::{self, Rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub food: Position,
    pub snake: Snake,
    pub score: usize,
    pub lost: bool,
}

#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Snake {
    body: Vec<Position>,
    direction: Direction,
}

impl Snake {
    pub fn new(length: usize, head_pos: Position) -> Self {

        let mut body = vec![];
        let mut i = 1;
        while i <= length {
            body.push(Position{x: head_pos.x - i, y: head_pos.y});
            i += 1;
        };
        Self {
            body,
            direction: Direction::Right,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            food: Game::get_new_food_pos(width, height),
            snake: Snake::new(4, Position{x: width/2, y: height/2 }),
            score: 0,
            lost: false,
        }
    }

    pub fn place_new_food(&mut self) {
        self.food = Game::get_new_food_pos(self.width, self.height);
    }

    pub fn get_new_food_pos(width: usize, height: usize) -> Position {
        let mut rng = rand::thread_rng();

        Position { 
            x: rng.gen_range(1..width-1),
            y: rng.gen_range(1..height-1),
        }
    }

    pub fn change_direction(&mut self, dir: Direction) {
        match (&self.snake.direction, dir) {
            (Direction::Up, Direction::Left)  
            | (Direction::Up, Direction::Right) 
            | (Direction::Left, Direction::Up)
            | (Direction::Left, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Down, Direction::Right)
            | (Direction::Right, Direction::Up)
            | (Direction::Right, Direction::Down) => {self.snake.direction = dir}
            (_, _) => {self.snake.direction = self.snake.direction}
        }
    }

    pub fn tick(&mut self) {

        let (x, y) = (self.snake.body[0].x, self.snake.body[0].y);
        let new_head = match &self.snake.direction {
            Direction::Up => Position { x, y: y+1 },
            Direction::Left => Position { x: x-1, y },
            Direction::Down => Position { x, y: y-1 },
            Direction::Right => Position { x: x+1, y },
        };
        self.snake.body.insert(0, new_head);

        if new_head.eq(&self.food) {
            self.score += 1;
            while self.snake.body.contains(&self.food) {
                self.place_new_food();
            }
        } else {
            self.snake.body.pop();
        }

        if self.snake.body[1..].contains(&new_head) 
        || new_head.x < 0 || new_head.x > self.width-1
        || new_head.y < 0 || new_head.y > self.height-1 {
            self.lost = true;
            return;
        }
    }

}

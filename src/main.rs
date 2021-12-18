use macroquad::prelude::*;
use std::collections::LinkedList;

const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 720.0;
const FIELD_CELLS: i8 = 16;
const CELL_DIMENSION: f32 = 20.0;
const GAME_FIELD_BORDER_WIDTH: f32 = 2.0;

fn window_conf() -> Conf {
    Conf {
        window_height: WINDOW_HEIGHT as i32,
        window_width: WINDOW_WIDTH as i32,
        window_title: "Snake".to_owned(),
        ..Default::default()
    }
}

struct Game {
    score: i32,
    fps: i32,
    snake: Snake,
    apple: Point,
    current_time: f64,
    is_over: bool,
}

impl Game {
    fn new() -> Game {
        let snake = Snake::new();
        let apple = Game::spawn_apple(&snake.body);

        Game {
            score: 0,
            fps: 5,
            snake,
            apple,
            current_time: 0.0,
            is_over: false,
        }
    }

    fn draw_score(&self) {
        draw_text(
            &*format!("Score: {}", self.score),
            25.0,
            25.0,
            24.0,
            DARKGRAY,
        )
    }

    fn draw_game_field() {
        // Left up corner of game field
        let game_field_size: f32 = FIELD_CELLS as f32 * CELL_DIMENSION;
        let left_up_corner_x: f32 = WINDOW_WIDTH / 2.0 - game_field_size / 2.0;
        let left_up_corner_y: f32 = WINDOW_HEIGHT / 2.0 - game_field_size / 2.0;
        draw_rectangle(
            left_up_corner_x - GAME_FIELD_BORDER_WIDTH,
            left_up_corner_y - GAME_FIELD_BORDER_WIDTH,
            game_field_size + 2.0 * GAME_FIELD_BORDER_WIDTH,
            game_field_size + 2.0 * GAME_FIELD_BORDER_WIDTH,
            DARKGRAY,
        );
        draw_rectangle(
            left_up_corner_x,
            left_up_corner_y,
            game_field_size,
            game_field_size,
            RED,
        );
    }

    fn draw_end_game() {
        clear_background(RED);
        draw_text("GAME OVER", 100.0, 100.0, 36.0, DARKGRAY);
    }

    fn draw_snake(&self) {
        self.snake.draw()
    }

    fn draw_apple(&self) {
        self.apple.draw(GREEN);
    }

    fn tick(&mut self) {
        let current_time = get_time();
        self.snake.turn_snake();
        if current_time - self.current_time > (1.0 / self.fps as f64) {
            if *self.snake.head() == self.apple {
                self.snake.is_hungry = false;
                self.apple = Game::spawn_apple(&self.snake.body);
                self.score += 1;
            }
            self.snake.move_body();
            if self.snake.check_collision() {
                self.is_over = true;
                return;
            }
            self.snake.is_hungry = true;
            self.current_time = current_time;
        }
    }

    fn spawn_apple(occupied: &LinkedList<Point>) -> Point {
        let apple_x = rand::gen_range(0, FIELD_CELLS as i32);
        let apple_y = rand::gen_range(0, FIELD_CELLS as i32);
        let apple = Point::new(apple_x, apple_y);
        loop {
            for point in occupied.iter() {
                if *point != apple {
                    return apple;
                }
            }
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn draw(&self, color: Color) {
        let game_field_size: f32 = FIELD_CELLS as f32 * CELL_DIMENSION;
        let x = (WINDOW_WIDTH / 2.0 - game_field_size / 2.0) + self.x as f32 * CELL_DIMENSION;
        let y =
            (WINDOW_HEIGHT / 2.0 + game_field_size / 2.0) - (self.y as f32 + 1.0) * CELL_DIMENSION;
        draw_rectangle(x, y, CELL_DIMENSION, CELL_DIMENSION, color)
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Snake {
    body: LinkedList<Point>,
    direction: Direction,
    is_hungry: bool,
}

impl Snake {
    fn draw(&self) {
        for point in self.body.iter() {
            point.draw(DARKGRAY);
        }
    }

    fn move_body(&mut self) {
        let current_head = self.body.front().unwrap();
        let new_head: Point;
        match self.direction {
            Direction::UP => {
                new_head = Point::new(current_head.x, current_head.y + 1);
            }
            Direction::LEFT => {
                new_head = Point::new(current_head.x - 1, current_head.y);
            }
            Direction::DOWN => {
                new_head = Point::new(current_head.x, current_head.y - 1);
            }
            Direction::RIGHT => {
                new_head = Point::new(current_head.x + 1, current_head.y);
            }
        }
        self.body.push_front(new_head);
        if self.is_hungry {
            self.body.pop_back();
        }
    }

    fn turn_snake(&mut self) {
        if is_key_down(KeyCode::W) && self.direction != Direction::DOWN {
            self.direction = Direction::UP;
            return;
        }
        if is_key_down(KeyCode::D) && self.direction != Direction::LEFT {
            self.direction = Direction::RIGHT;
            return;
        }
        if is_key_down(KeyCode::A) && self.direction != Direction::RIGHT {
            self.direction = Direction::LEFT;
            return;
        }
        if is_key_down(KeyCode::S) && self.direction != Direction::UP {
            self.direction = Direction::DOWN;
            return;
        }
    }

    fn new() -> Snake {
        let initial_point = Point::new(5, 5);
        let second_point = Point::new(5, 6);
        let body = LinkedList::from([initial_point, second_point]);
        Snake {
            body,
            direction: Direction::RIGHT,
            is_hungry: true,
        }
    }

    fn head(&self) -> &Point {
        self.body.front().unwrap()
    }

    fn check_collision(&self) -> bool {
        // Возвращает true если змейка столкнулась с собой
        // или с границей игрового поля
        let head = self.head();
        for (pos, point) in self.body.iter().enumerate() {
            if pos == 0 {
                continue;
            }
            if point == head {
                return true;
            }
        }
        match self.direction {
            Direction::UP => {
                if head.y == FIELD_CELLS as i32 {
                    return true;
                }
            }
            Direction::RIGHT => {
                if head.x == FIELD_CELLS as i32 {
                    return true;
                }
            }
            Direction::DOWN => {
                if head.y < 0 {
                    return true;
                }
            }
            Direction::LEFT => {
                if head.x < 0 {
                    return true;
                }
            }
        }
        false
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        if game.is_over {
            Game::draw_end_game();
        } else {
            clear_background(RED);
            Game::draw_game_field();
            game.draw_score();
            game.draw_apple();
            game.draw_snake();
            game.tick();
        }
        next_frame().await
    }
}

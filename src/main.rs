// #![windows_subsystem = "windows"]
use macroquad::audio::{load_sound, play_sound_once, Sound};
use macroquad::prelude::*;
use std::collections::{LinkedList, VecDeque};

const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 720.0;
const FIELD_CELLS: i8 = 16;
const CELL_DIMENSION: f32 = 20.0;
const GAME_FIELD_BORDER_WIDTH: f32 = 2.0;

const BACKGROUND_COLOR: &str = "FCF0C8";
const SNAKE_COLOR: &str = "911F27";
const APPLE_COLOR: &str = "630A10";
const UI_COLOR: &str = "630A10";

const FPS: i32 = 5;

fn window_conf() -> Conf {
    Conf {
        window_height: WINDOW_HEIGHT as i32,
        window_width: WINDOW_WIDTH as i32,
        window_title: "Snake".to_owned(),
        ..Default::default()
    }
}

fn hex_to_color(hexcolor: &str) -> Color {
    let formatted_string: &str;
    match hexcolor.strip_suffix("#") {
        Some(color) => {
            if color.len() != 6 {
                panic!("Invalid hex string");
            }
            formatted_string = color;
        }
        None => {
            if hexcolor.len() != 6 {
                panic!("invalid hex string");
            }
            formatted_string = hexcolor
        }
    }
    let red = i64::from_str_radix(&formatted_string[0..2], 16).unwrap();
    let green = i64::from_str_radix(&formatted_string[2..4], 16).unwrap();
    let blue = i64::from_str_radix(&formatted_string[4..6], 16).unwrap();
    let color = Color::new(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        1.0,
    );
    color
}

struct Game {
    score: i32,
    fps: i32,
    snake: Snake,
    apple: Point,
    current_time: f64,
    is_over: bool,
    ui_color: Color,
    bg_color: Color,
    apple_color: Color,
    snake_color: Color,
    apple_sound: Sound,
}

impl Game {
    fn new(apple_sound: Sound) -> Game {
        let snake = Snake::new();
        let apple = Game::spawn_apple(&snake.body);
        let ui_color = hex_to_color(UI_COLOR);
        let bg_color = hex_to_color(BACKGROUND_COLOR);
        let apple_color = hex_to_color(APPLE_COLOR);
        let snake_color = hex_to_color(SNAKE_COLOR);

        Game {
            score: 0,
            fps: FPS,
            snake,
            apple,
            current_time: 0.0,
            is_over: false,
            ui_color,
            bg_color,
            apple_color,
            snake_color,
            apple_sound,
        }
    }

    fn draw_score(&self) {
        draw_text(
            &*format!("Score: {}", self.score),
            25.0,
            25.0,
            24.0,
            self.ui_color,
        )
    }

    fn draw_game_field(&self) {
        // Left up corner of game field
        let game_field_size: f32 = FIELD_CELLS as f32 * CELL_DIMENSION;
        let left_up_corner_x: f32 = WINDOW_WIDTH / 2.0 - game_field_size / 2.0;
        let left_up_corner_y: f32 = WINDOW_HEIGHT / 2.0 - game_field_size / 2.0;
        draw_rectangle(
            left_up_corner_x - GAME_FIELD_BORDER_WIDTH,
            left_up_corner_y - GAME_FIELD_BORDER_WIDTH,
            game_field_size + 2.0 * GAME_FIELD_BORDER_WIDTH,
            game_field_size + 2.0 * GAME_FIELD_BORDER_WIDTH,
            self.ui_color,
        );
        draw_rectangle(
            left_up_corner_x,
            left_up_corner_y,
            game_field_size,
            game_field_size,
            self.bg_color,
        );
    }

    fn draw_end_game(&self) {
        clear_background(self.bg_color);
        draw_text(
            "GAME OVER. \n Press Enter to rty again!",
            100.0,
            100.0,
            36.0,
            self.ui_color,
        );
    }

    fn draw_snake(&self) {
        self.snake.draw(self.snake_color)
    }

    fn draw_apple(&self) {
        self.apple.draw(self.apple_color);
    }

    fn tick(&mut self) {
        let current_time = get_time();
        self.snake.process_commands();
        if current_time - self.current_time > (1.0 / self.fps as f64) {
            if *self.snake.head() == self.apple {
                play_sound_once(self.apple_sound);
                self.snake.is_hungry = false;
                self.apple = Game::spawn_apple(&self.snake.body);
                self.score += 1;
            }
            self.snake.turn_snake();
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

#[derive(PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug)]
struct CommandsQueue {
    queue: VecDeque<Direction>,
}

impl CommandsQueue {
    fn new() -> CommandsQueue {
        let mut queue = VecDeque::with_capacity(2);
        queue.push_back(Direction::RIGHT);
        CommandsQueue { queue }
    }
    fn push_direction(&mut self, direction: Direction) {
        if self.get_last() != direction {
            self.queue.push_back(direction)
        }
    }
    fn get_direction(&mut self) -> Direction {
        if self.queue.len() == 1 {
            let current_direction = self.queue.get(0).unwrap().clone();
            return current_direction;
        }
        self.queue.pop_front().unwrap()
    }
    fn get_last(&self) -> Direction {
        return self.queue.back().unwrap().clone();
    }
}

struct Snake {
    body: LinkedList<Point>,
    is_hungry: bool,
    commands_queue: CommandsQueue,
    direction: Direction,
}

impl Snake {
    fn draw(&self, color: Color) {
        for point in self.body.iter() {
            point.draw(color);
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

    fn process_commands(&mut self) {
        let last_command = self.commands_queue.get_last();
        if is_key_down(KeyCode::W) && last_command != Direction::DOWN {
            // self.direction = Direction::UP;
            self.commands_queue.push_direction(Direction::UP);
            return;
        }
        if is_key_down(KeyCode::D) && last_command != Direction::LEFT {
            // self.direction = Direction::RIGHT;
            self.commands_queue.push_direction(Direction::RIGHT);
            return;
        }
        if is_key_down(KeyCode::A) && last_command != Direction::RIGHT {
            // self.direction = Direction::LEFT;
            self.commands_queue.push_direction(Direction::LEFT);
            return;
        }
        if is_key_down(KeyCode::S) && last_command != Direction::UP {
            // self.direction = Direction::DOWN;
            self.commands_queue.push_direction(Direction::DOWN);
            return;
        }
    }

    fn turn_snake(&mut self) {
        self.direction = self.commands_queue.get_direction();
    }

    fn new() -> Snake {
        let initial_point = Point::new(5, 5);
        let second_point = Point::new(5, 6);
        let body = LinkedList::from([initial_point, second_point]);
        let commands = CommandsQueue::new();
        Snake {
            body,
            is_hungry: true,
            commands_queue: commands,
            direction: Direction::RIGHT,
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
    let apple_sound = load_sound("assets/mixkit-small-hit-in-a-game-2072.wav")
        .await
        .unwrap();
    let mut game = Game::new(apple_sound);
    let bg_color = hex_to_color("FEECE9");
    loop {
        if game.is_over {
            game.draw_end_game();
            if is_key_down(KeyCode::Enter) {
                game = Game::new(apple_sound);
            }
        } else {
            clear_background(bg_color);
            game.draw_game_field();
            game.draw_score();
            game.draw_apple();
            game.draw_snake();
            game.tick();
        }
        next_frame().await
    }
}

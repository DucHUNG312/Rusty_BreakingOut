use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([120f32, 20f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = const_vec2!([100f32, 30f32]);
const BALL_SIZE: f32 = 20f32;
const BALL_SPEED: f32 = 400f32;

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath,
}

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            )
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _=> 0f32,
        };
        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self .rect.w {
            self.rect.x = screen_width() - self .rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
    }
}

struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 7,
            block_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                    7 => RED,
                    6 => ORANGE,
                    5 => YELLOW,
                    4 => GREEN,
                    3 => BLUE,
                    2 => PURPLE,
                    _ => VIOLET,
                },
            BlockType::SpawnBallOnDeath => match self.lives {
                    7 => BLACK,
                    6 => BLACK,
                    5 => DARKGRAY,
                    4 => DARKGRAY,
                    3 => DARKGRAY,
                    2 => GRAY,
                    _ => GRAY
            },
        }; 
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

pub struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self { 
            rect: Rect::new(
                pos.x, 
                pos.y,
                BALL_SIZE,
                BALL_SIZE,
            ), 
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.25f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() }
    );
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {

    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum(); 
    match intersection.w > intersection.h {
        true => {
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            a.x -= to_signum.x * intersection.h;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}

fn pushing_ball(balls: &mut Vec<Ball>) {
    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32 - BALL_SIZE * 0.5f32,
        screen_height() * 0.5f32,
    )));
}

fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
) {
    *player = Player::new();
    *score = 0;
    *player_lives = 3;
    balls.clear();
    blocks.clear();
    pushing_ball(balls);
    init_block(blocks);
}

fn init_block(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 4);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), BlockType::Regular));
    }
    for _ in 0..5 {
        let rand_index = rand::gen_range(0, blocks.len());
        blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
    }
}

#[macroquad::main("rusty_breakingout")]
async fn main() {
    let font = load_ttf_font("res/Heebo-VariableFont_wght.ttf").await.unwrap();
    let mut game_state = GameState::Menu;

    let mut score = 0;
    let mut player_lives = 3;
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    
    pushing_ball(&mut balls);
    init_block(&mut blocks);

    loop {
        match game_state {
            GameState::Menu => {
               if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
               }
            },
            GameState::Game => {
                player.update(get_frame_time());

                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }
        
                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -=1;
                            if block.lives <= 0 {
                                score += 10;
                                if block.block_type == BlockType::SpawnBallOnDeath {
                                    spawn_later.push(Ball::new(ball.rect.point()));
                                }
                            }
                        }
                    }
                }

                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                }
        
                let balls_len = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());

                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && balls.is_empty() {
                    player_lives -= 1;
                    balls.push(Ball::new(
                        player.rect.point() + vec2(player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32,
                         -50f32)
                    ));
                    if player_lives <= 0 {
                        game_state = GameState::Dead;
                    }
                }
        
                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
            },
            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
        }

        clear_background(WHITE);
        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }

        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to start", font);
            },
            GameState::Game => {
                let score_text = &format!("SCORE: {}", score);
                let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() }
                );

                draw_text_ex(
                    &format!("LIVES: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() }
                );
            },
            GameState::LevelCompleted => {
                draw_title_text(&format!("You win! {} score", score), font);
            },
            GameState::Dead => {
                draw_title_text(&format!("Try again! {} score", score), font);
            },
        }

        next_frame().await
    }
}
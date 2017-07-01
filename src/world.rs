extern crate rand;

use std::collections::HashMap;
use color::Color;
use size::Size;
use entity::*;
use max_min::*;
use point::*;
use config::*;
use score::*;
use rand::*;
use game_state::*;
use grid_distribution::*;
use default;
use std::time::{Instant};
use rand::distributions::{IndependentSample, Range};

pub struct World {
    pub hero: (Point, Entity),
    pub enemies: HashMap<Point, Entity>,
    pub start_time: Instant,
    pub rng: ThreadRng,
    pub config: Config,
    pub score: Score,
    pub game_state: GameState,
}

// Initialization
impl World {
    pub fn new(config: Config) -> World {
        let hero = Entity {
            color: config.hero_color,
            size: config.hero_size,
        };

        let world = World {
            hero: (config.hero_starting_position, hero),
            enemies: default::default(),
            start_time: Instant::now(),
            rng: rand::thread_rng(),
            config: config,
            score: default::default(),
            game_state: default::default(),
        };

        world
    }

    pub fn start(&mut self) {
        self.hero.0 = self.config.hero_starting_position;
        self.enemies = default::default();
        self.score = default::default();

        self.populate_with_enemies();

        self.game_state = GameState::Playing;
    }

    pub fn add_enemy_at(&mut self, position: Point, enemy: Entity) {
        self.enemies.insert(position, enemy);
    }

    pub fn populate_with_enemies(&mut self) {
        let grid_size = self.config.world_size - Size { height: 200, width: 40 };
        let remaining_space = self.config.world_size.width - grid_size.width;
        let padding_on_sides_of_grid = (remaining_space as f64 / 2.0) as u32;

        let distribution = GridDistribution {
            available_space: grid_size,
            entity_size: self.config.enemy_size,
            horizontal_padding: 10,
            vertical_padding: 10,
        }.distribute();

        for point in distribution {
            let enemy = Entity {
                size: Size {
                    height: self.config.enemy_size.height,
                    width: self.config.enemy_size.width
                },
                color: self.config.enemy_color,
            };

            self.add_enemy_at(
                point + Point { x: padding_on_sides_of_grid, y: 20 },
                enemy
                );
        }
    }
}

// Moving the hero
impl World {
    pub fn move_up(&mut self) {
        if !self.config.vertical_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.y <= 0 { return }
        let new_pos = Point { y: current_pos.y - self.config.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_down(&mut self) {
        if !self.config.vertical_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.y + self.hero.1.size.height >= self.config.world_size.height { return }
        let new_pos = Point { y: current_pos.y + self.config.hero_speed, .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_left(&mut self) {
        if !self.config.horizontal_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.x <= 0 { return }
        let new_pos = Point { x: current_pos.x - self.config.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn move_right(&mut self) {
        if !self.config.horizontal_movement_allowed { return }
        let (current_pos, _) = self.hero;
        if current_pos.x + self.hero.1.size.width >= self.config.world_size.width { return }
        let new_pos = Point { x: current_pos.x + self.config.hero_speed , .. current_pos };
        self.hero.0 = new_pos;
        self.check_if_still_alive();
    }

    pub fn check_if_still_alive(&mut self) {
        if self.dead() { return };

        let h = PositionAndSize {
            position: self.hero.0,
            size: self.hero.1.size,
        };

        for (position, enemy) in &self.enemies {
            let p = PositionAndSize {
                position: position.clone(),
                size: enemy.size,
            };

            if h.collides_with(&p) || p.collides_with(&h) {
                self.game_state = GameState::Dead;
                return;
            }
        }
    }

    pub fn dead(&self) -> bool {
        match self.game_state {
            GameState::Dead => true,
            _ => false,
        }
    }
    pub fn alive(&self) -> bool { !self.dead() }

    pub fn move_enemies(&mut self) {
        self.check_if_still_alive();
    }
}

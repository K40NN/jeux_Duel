use rand::Rng;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};

#[derive(Debug)]
struct Player {
    name: String,
    vitality: i32,
    speed: u64,
    strength: i32,
    score: i32,
}

impl Player {
    fn new(name: String, vitality: i32, speed: u64, strength: i32) -> Self {
        Player {
            name,
            vitality,
            speed,
            strength,
            score: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    player1: Player,
    player2: Player,
    objectives: Vec<i32>,
}

impl Game {
    fn new(player1: Player, player2: Player) -> Self {
        Game {
            player1,
            player2,
            objectives: vec![],
        }
    }

    fn generate_objectives(&mut self, count: usize) {
        self.objectives.clear();
        let mut rng = rand::rng();
        for _ in 0..count {
            self.objectives.push(rng.random_range(0..101));
        }
    }
}


fn main() {
}
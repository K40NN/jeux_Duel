use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use rand::Rng;

struct Player {
    name: String,
    vitality: i32,
    speed: u64,
    strength: i32,
    score: i32,
}

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

fn calculate_score(diff: u32, miss: u32, strength: i32) -> i32 {
    match diff {
        0 => (100 + strength) / (miss + 1) as i32,
        1..=5 => (80 + strength) / (miss + 1) as i32,
        6..=10 => (60 + strength) / (miss + 1) as i32,
        11..=20 => (40 + strength) / (miss + 1) as i32,
        21..=40 => (20 + strength) / (miss + 1) as i32,
        _ => (0 + strength) / (miss + 1) as i32,
    }
}

fn play_turn(player: &mut Player, objectives: &[i32]) -> i32 {
    let mut scores = vec![];

    println!("→ Objectifs : {:?}", objectives);
    println!("→ Appuyer sur ENTREE pour démarrer le tour...");

    for &objectif in objectives {
        let (tx, rx) = mpsc::channel();
        let speed = player.speed;

        let handle = thread::spawn(move || {
            let mut counter = 0;
            let mut miss = 0;
            loop {
                if rx.try_recv().is_ok() {
                    return (counter, miss);
                }
                print!("\r{:width$}\r→ Objectif {} : Miss = {} | Compteur = {}", "", objectif, miss, counter, width = 50);
                io::stdout().flush().unwrap();

                counter = (counter + 1) % 100;
                if counter == 0 {
                    miss += 1;
                }
                thread::sleep(Duration::from_millis(speed));
            }
        });

        let _ = io::stdin().read_line(&mut String::new());
        tx.send(()).unwrap();

        let (final_counter, miss): (i32, i32) = handle.join().unwrap();
        let diff = (final_counter - objectif).abs() as u32;
        let score = calculate_score(diff, miss as u32, player.strength);
        println!(" | Score obtenu : {}", score);
        scores.push(score);
    }

    let total_score: i32 = scores.iter().sum();
    let average_score = (total_score as f32 / scores.len() as f32).ceil() as i32;
    println!("# Fin du tour #");
    println!("→ Score moyen {}", average_score);

    player.score = average_score;
    average_score
}

fn main() {
    let player1 = Player {
        name: String::from("Player1"),
        vitality: 10,
        speed: 100,
        strength: 8,
        score: 0,
    };
    let player2 = Player {
        name: String::from("Player2"),
        vitality: 12,
        speed: 90,
        strength: 9,
        score: 0,
    };

    let mut game = Game::new(player1, player2);
    game.generate_objectives(3);

    println!("Tour de {}", game.player1.name);
    play_turn(&mut game.player1, &game.objectives);

    println!("Tour de {}", game.player2.name);
    play_turn(&mut game.player2, &game.objectives);
}
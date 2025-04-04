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
    let score = match diff {
        0 => (100 + strength) / (miss + 1) as i32,
        1..=5 => (80 + strength) / (miss + 1) as i32,
        6..=10 => (60 + strength) / (miss + 1) as i32,
        11..=20 => (40 + strength) / (miss + 1) as i32,
        21..=40 => (20 + strength) / (miss + 1) as i32,
        _ => (0 + strength) / (miss + 1) as i32,
    };
    println!("Détails du score : (diff {} + strength {}) / (miss{} + 1) = score : {}", score*(miss as i32 + 1)-strength, strength, miss, score);
    score
}

fn play_turn(player: &mut Player, objectives: &[i32]) -> i32 {
    let mut scores = vec![];
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
                print!("\r{:width$}\r→ Objectif {} : Miss = {} | Compteur = {}", "", objectif, miss, counter + 1, width = 50);
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
        scores.push(score);
        let _ = io::stdin().read_line(&mut String::new());
    }

    let total_score: i32 = scores.iter().sum();
    let average_score = (total_score as f32 / scores.len() as f32).ceil() as i32;
    println!("# Fin du tour #");
    println!("→ Score moyen {}", average_score);

    player.score = average_score;
    average_score
}

fn apply_poison(winner: &mut Player, loser: &mut Player) {
    println!("{} vous devez choisir quel poison appliquer à {} :", winner.name, loser.name);
    println!("→ 1: -15 speed -le compteur passe + vite-");
    println!("→ 2: -5 strength -penalité sur le calcul du score-");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<i32>().unwrap_or(0);

    match choice {
        1 => loser.speed = loser.speed.saturating_sub(15),
        2 => loser.strength = loser.strength.saturating_sub(5),
        _ => println!("Choix invalide, aucun poison appliqué."),
    }
}

fn main() {
    let player1 = Player {
        name: String::from("Michel"),
        vitality: 50,
        speed: 50,
        strength: 50,
        score: 0,
    };
    let player2 = Player {
        name: String::from("Jacque"),
        vitality: 50,
        speed: 50,
        strength: 50,
        score: 0,
    };

    let mut game = Game::new(player1, player2);

    while game.player1.vitality > 0 && game.player2.vitality > 0 {
        game.generate_objectives(5);
        let objectives_player1 = game.objectives.clone();
        game.generate_objectives(5);
        let objectives_player2 = game.objectives.clone();
        
        println!("→ Objectifs : {:?}", objectives_player1);
        println!("→ Appuyer sur ENTREE pour démarrer le tour...");
        let _ = io::stdin().read_line(&mut String::new());
        let score1 = play_turn(&mut game.player1, &objectives_player1);
        
        println!("→ Objectifs : {:?}", objectives_player2);
        println!("→ Appuyer sur ENTREE pour démarrer le tour...");
        let _ = io::stdin().read_line(&mut String::new());
        let score2 = play_turn(&mut game.player2, &objectives_player2);

        if score1 > score2 {
            println!("{} gagne la manche.", game.player1.name);
            let damage = score1 - score2;
            println!("Dégâts infligés à {} : {} points de vitalité perdus.", game.player2.name, damage);
            game.player2.vitality -= score1 - score2;
            apply_poison(&mut game.player1, &mut game.player2);
        } else {
            println!("{} gagne la manche.", game.player2.name);
            let damage = score1 - score2;
            println!("Dégâts infligés à {} : {} points de vitalité perdus.", game.player1.name, damage);
            game.player1.vitality -= score2 - score1;
            apply_poison(&mut game.player2, &mut game.player1);
        }

        println!("Vitalité de {}: {}", game.player1.name, game.player1.vitality);
        println!("Vitalité de {}: {}", game.player2.name, game.player2.vitality);
    }

    if game.player1.vitality <= 0 {
        println!("{} a perdu la partie.", game.player1.name);
    } else {
        println!("{} a perdu la partie.", game.player2.name);
    }
}
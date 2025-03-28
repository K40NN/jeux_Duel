use rand::Rng;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{execute, terminal::{Clear, ClearType}};

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

fn start_counter(player: &Player, miss: Arc<Mutex<i32>>) -> i32 {
    let mut counter = 0;
    let miss_clone = Arc::clone(&miss);
    let speed = player.speed;
    loop {
        print!("\rCompteur = {}", counter);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(speed));
        counter = (counter + 1) % 101;
       if counter % 101 == 0 {
        let mut miss = miss_clone.lock().unwrap();
        *miss += 1;
        // Efface la ligne actuelle
        execute!(io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
       }
        if event::poll(Duration::from_millis(30)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Enter {
                    break;
                }
            }
        }
    }
    counter
}

fn play_turn(player: &mut Player, objectives: &Vec<i32>) -> i32 {
    let mut total_score = 0;
    let mut obejctif_index = 0;

    println!("Au tour de {} (Vitality={}, Speed={}, Strength={})", player.name, player.vitality, player.speed, player.strength);
    println!("→ Objectifs : {:?}", objectives);
    println!("→ Appuyer sur ENTREE pour démarrer le tour..");
    loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.code == KeyCode::Enter {
                break;
            }
        }
    }
    
    for &objective in objectives.iter() {
        let miss = Arc::new(Mutex::new(0));
        let counter = start_counter(player, Arc::clone(&miss));
        let diff = (counter - objective).abs();
        let miss_value = *miss.lock().unwrap();
        let score = match diff {
            0 => (100 + player.strength) / (miss_value + 1),
            1..=5 => (80 + player.strength) / (miss_value + 1),
            6..=10 => (60 + player.strength) / (miss_value + 1),
            11..=20 => (40 + player.strength) / (miss_value + 1),
            21..=40 => (20 + player.strength) / (miss_value + 1),
            _ => (0 + player.strength) / (miss_value + 1),
        };
        total_score += score;
        obejctif_index +=1 ;
        println!("→ Objectif {} : Miss = {} | Compteur = {}   // Score = {}", objective, miss_value, counter, score);
        // Affiche les objectifs restants après chaque tour
        let remaining_objectives: Vec<_> = objectives.iter().rev().take(objectives.len() - obejctif_index ).rev().collect();
        println!("→ Objectifs restants : {:?}", remaining_objectives);
            
        // Ajoute un délai après avoir appuyé sur Enter
        thread::sleep(Duration::from_secs(1));

    }

    let average_score = (total_score as f32 / objectives.len() as f32).ceil() as i32;
    println!("# Fin du tour #");
    println!("→ Score moyen {}", average_score);

    player.score = average_score;
    average_score
}



fn apply_poison(winner: &mut Player, loser: &mut Player) {
    println!("{} vous devez choisir quel poison appliquer à {} :", winner.name, loser.name);
    println!("→ 1: -5 speed");
    println!("→ 2: -5 strength");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<i32>().unwrap_or(0);

    match choice {
        1 => loser.speed = loser.speed.saturating_sub(5),
        2 => loser.strength = loser.strength.saturating_sub(5),
        _ => println!("Choix invalide, aucun poison appliqué."),
    }
}

fn main() {
    let player1 = Player::new("Michel".to_string(), 50, 50, 50);
    let player2 = Player::new("Jacque".to_string(), 50, 50, 50);

    let mut game = Game::new(player1, player2);

    while game.player1.vitality > 0 && game.player2.vitality > 0 {
        game.generate_objectives(5);
        let objectives_player1 = game.objectives.clone();
        game.generate_objectives(5);
        let objectives_player2 = game.objectives.clone();

        let score1 = {
            play_turn(&mut game.player1, &objectives_player1)
        };
        
        let score2 = {
            play_turn(&mut game.player2, &objectives_player2)
        };

        if score1 > score2 {
            println!("{} gagne la manche.", game.player1.name);
            game.player2.vitality -= score1 - score2;
            apply_poison(&mut game.player1, &mut game.player2);
        } else {
            println!("{} gagne la manche.", game.player2.name);
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
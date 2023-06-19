use rand::prelude::*;
use std::io::Write;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    hand: Card,
    is_ai: bool,
    money: u32,
}

#[derive(Debug, Clone, Copy)]
enum Card {
    Jack,
    Queen,
    King,
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Jack => 0,
            Card::Queen => 1,
            Card::King => 2,
        }
    }

    fn name(&self) -> &str {
        match self {
            Card::Jack => "Jack",
            Card::Queen => "Queen",
            Card::King => "King",
        }
    }

    fn draw() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..3) {
            0 => Card::Jack,
            1 => Card::Queen,
            2 => Card::King,
            _ => panic!("Invalid card"),
        }
    }
}

impl Player {
    fn new(name: String, is_ai: bool, money: u32) -> Self {
        Player {
            name,
            hand: Card::draw(),
            is_ai,
            money,
        }
    }

    fn hand(&self) -> Card {
        self.hand
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_broke(&self) -> bool {
        self.money == 0
    }

    fn bet(&mut self, amount: u32) -> bool {
        if self.money < amount {
            false
        } else {
            self.money -= amount;
            true
        }
    }

    // Raise if true, check if false
    fn raise(&self) -> bool {
        if self.is_ai {
            let mut rng = thread_rng();
            match self.hand {
                Card::Jack => rng.gen_bool(0.4),
                Card::Queen => rng.gen_bool(0.7),
                Card::King => true,
            }
        } else {
            println!("Your card is {}", self.hand().name());
            println!("Do you want to raise? (y/n)");
            print!("> ");

            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input = input.trim().to_string();
            if input != "y" && input != "n" {
                println!("Invalid input");
                self.raise()
            } else {
                input == "y"
            }
        }
    }

    // Call if true, fold if false
    fn call(&self) -> bool {
        if self.is_ai {
            let mut rng = thread_rng();
            match self.hand {
                Card::Jack => rng.gen_bool(0.1),
                Card::Queen => rng.gen_bool(0.5),
                Card::King => true,
            }
        } else {
            println!("Do you want to call? (y/n)");
            print!("> ");

            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input = input.trim().to_string();
            if input != "y" && input != "n" {
                println!("Invalid input");
                self.call()
            } else {
                input == "y"
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    players: (Player, Player),
}

impl Game {
    fn new(players: (Player, Player)) -> Self {
        Game { players }
    }

    fn play(&mut self) {
        // Check money of players
        if self.players.0.is_broke() {
            println!("{} is broke", self.players.0.name());
            println!("{} wins", self.players.1.name());
            return;
        } else if self.players.1.is_broke() {
            println!("{} is broke", self.players.1.name());
            println!("{} wins", self.players.0.name());
            return;
        }

        // Draw cards
        self.players.0.hand = Card::draw();
        self.players.1.hand = Card::draw();

        // Bet (default)
        let mut pot = 0;
        let mut bet = 1;
        self.players.0.bet(bet);
        self.players.1.bet(bet);
        pot += bet * 2;

        loop{
            // Show own card and raise or check
            let p1_raise = if self.players.0.money < bet || self.players.1.money < bet { 
                false
            } else {
                self.players.0.raise()
            };

            if p1_raise {
                println!("{} raises", self.players.0.name());
                self.players.0.bet(bet);
                pot += bet;
                let p2_call = self.players.1.call();
                if p2_call {
                    println!("{} calls", self.players.1.name());
                    self.players.1.bet(bet);
                    pot += bet;
                    bet *= 2;
                    println!("Now the bet is {}", bet);
                    continue;
                } else {
                    println!("{} folds", self.players.1.name());
                    println!("{} wins", self.players.0.name());
                    self.players.0.money += pot;
                    return;
                }
            } else {
                println!("{} checks", self.players.0.name());
                let p2_raise = if self.players.0.money < bet || self.players.1.money < bet {
                    false 
                } else {
                    self.players.1.raise()
                };
                if p2_raise {
                    println!("{} raises", self.players.1.name());
                    self.players.1.bet(bet);
                    pot += bet;
                    let p1_call = self.players.0.call();
                    if p1_call {
                        println!("{} calls", self.players.0.name());
                        self.players.0.bet(bet);
                        pot += bet;
                        bet *= 2;
                        println!("Now the bet is {}", bet);
                        continue;
                    } else {
                        println!("{} folds", self.players.0.name());
                        println!("{} wins", self.players.1.name());
                        self.players.1.money += pot;
                        return;
                    }
                } else {
                    println!("{} checks", self.players.1.name());
                    println!("------------------");
                    println!("{}'s card is {}", self.players.0.name(), self.players.0.hand().name());
                    println!("{}'s card is {}", self.players.1.name(), self.players.1.hand().name());
                    if self.players.0.hand().value() > self.players.1.hand().value() {
                        println!("{} wins", self.players.0.name());
                        self.players.0.money += pot;
                    } else if self.players.0.hand().value() < self.players.1.hand().value() {
                        println!("{} wins", self.players.1.name());
                        self.players.1.money += pot;
                    } else {
                        println!("Draw");
                        self.players.0.money += pot / 2;
                        self.players.1.money += pot / 2;
                    }
                    return;
                }
            }
        }
    }
}

fn main() {
    println!("Welcome to the game!");

    println!("Please set the amount of money");
    print!("> ");

    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    let money = input.parse::<u32>().unwrap();

    println!("Please enter your name");
    print!("> ");

    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    let player = Player::new(input, false, money);

    println!("Please enter your opponent's name");
    print!("> ");

    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    let opponent = Player::new(input, true, money);

    let mut game = Game::new(
        (player, opponent),
    );
    loop {
        game.play();
        println!("------------------");
        println!("{}'s money is {}", game.players.0.name(), game.players.0.money);
        println!("{}'s money is {}", game.players.1.name(), game.players.1.money);
        println!("------------------");
        if game.players.0.is_broke() {
            println!("{} is broke", game.players.0.name());
            println!("Finally {} wins", game.players.1.name());
            break;
        } else if game.players.1.is_broke() {
            println!("{} is broke", game.players.1.name());
            println!("Finally {} wins", game.players.0.name());
            break;
        }
        println!("Do you want to continue? (y/n)");
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input != "y" && input != "n" {
            println!("Invalid input");
            continue;
        } else if input == "n" {
            break;
        }
    }
}

use rand::prelude::*;
use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
struct Player {
    name: String,
    hand: Card,
    is_ai: bool,
    money: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn draw(&mut self) {
        self.hand = Card::draw();
    }

    fn bet(&mut self, amount: u32) -> bool {
        if self.money < amount {
            false
        } else {
            self.money -= amount;
            true
        }
    }

    fn get_input(&self, message: &str) -> bool {
        println!("{}", message);
        print!("> ");

        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input != "y" && input != "n" {
            println!("Invalid input");
            self.get_input(message)
        } else {
            input == "y"
        }
    }

    // Raise if true, check if false
    fn raise(&self, rng: &mut ThreadRng) -> bool {
        if self.is_ai {
            match self.hand {
                Card::Jack => rng.gen_bool(0.3),
                Card::Queen => rng.gen_bool(0.8),
                Card::King => true,
            }
        } else {
            self.get_input(&format!("Your card is {}. Do you want to raise? (y/n)", self.hand.name()))
        }
    }

    // Call if true, fold if false
    fn call(&self, rng: &mut ThreadRng) -> bool {
        if self.is_ai {
            match self.hand {
                Card::Jack => rng.gen_bool(0.1),
                Card::Queen => rng.gen_bool(0.6),
                Card::King => true,
            }
        } else {
            self.get_input("Do you want to call? (y/n)")
        }
    }
}

#[derive(Debug)]
struct Game {
    players: Vec<Rc<RefCell<Player>>>,
}

impl Game {
    fn new(players: Vec<Rc<RefCell<Player>>>) -> Self {
        Game { players }
    }

    fn play(&mut self, rng: &mut ThreadRng) {
        let mut bet = 1;
        let mut pot = bet * 2;

        let mut p0 = self.players[0].borrow_mut();
        let mut p1 = self.players[1].borrow_mut();

        p0.draw();
        p1.draw();

        if !(p0.bet(bet) && p1.bet(bet)) {
            return;
        }

        loop {
            let (first, second) = (&mut p0, &mut p1);
            let (first_name, second_name) = (first.name().to_string(), second.name().to_string());

            if first.money < bet || second.money < bet {
                break;
            }

            if first.raise(rng) {
                println!("{} raises", first_name);
                if first.bet(bet) {
                    pot += bet;
                } else {
                    return;
                }
                if second.call(rng) {
                    println!("{} calls", second_name);
                    if second.bet(bet) {
                        pot += bet;
                    } else {
                        return;
                    }
                    bet *= 2;
                    println!("Now the bet is {}", bet);
                    continue;
                } else {
                    println!("{} folds", second_name);
                    println!("{} wins", first_name);
                    first.money += pot;
                    return;
                }
            } else {
                println!("{} checks", first_name);
                if second.raise(rng) {
                    println!("{} raises", second_name);
                    if second.bet(bet) {
                        pot += bet;
                    } else {
                        return;
                    }
                    if first.call(rng) {
                        println!("{} calls", first_name);
                        if first.bet(bet) {
                            pot += bet;
                        } else {
                            return;
                        }
                        bet *= 2;
                        println!("Now the bet is {}", bet);
                        continue;
                    } else {
                        println!("{} folds", first_name);
                        println!("{} wins", second_name);
                        second.money += pot;
                        return;
                    }
                } else {
                    break;
                }
            }
        }

        println!("{} checks", p1.name());
        println!("------------------");
        println!("{}'s card is {}", p0.name(), p0.hand().name());
        println!("{}'s card is {}", p1.name(), p1.hand().name());

        match p0.hand().value().cmp(&p1.hand().value()) {
            std::cmp::Ordering::Greater => {
                println!("{} wins", p0.name());
                p0.money += pot;
            }
            std::cmp::Ordering::Less => {
                println!("{} wins", p1.name());
                p1.money += pot;
            }
            std::cmp::Ordering::Equal => {
                println!("Draw");
                p0.money += pot / 2;
                p1.money += pot / 2;
            }
        }
    }
}

fn prompt(msg: &str) -> String {
    println!("{}", msg);
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned()
}

fn get_money() -> u32 {
    loop {
        match prompt("Please set the amount of money").parse() {
            Ok(money) => return money,
            Err(_) => println!("Invalid input, please enter a number"),
        }
    }
}

fn get_name(prompt_msg: &str) -> String {
    prompt(prompt_msg)
}

fn main() {
    println!("Welcome to the game!");

    let money = get_money();

    let player = Player::new(get_name("Please enter your name"), false, money);
    let opponent = Player::new(get_name("Please enter your opponent's name"), true, money);

    let players = vec![Rc::new(RefCell::new(player)), Rc::new(RefCell::new(opponent))];

    let mut rng = thread_rng();

    let mut game = Game::new(players);

    loop {
        game.play(&mut rng);

        println!("------------------");
        let p0 = game.players[0].borrow();
        let p1 = game.players[1].borrow();
        println!("{}'s money is {}", p0.name(), p0.money);
        println!("{}'s money is {}", p1.name(), p1.money);
        println!("------------------");

        if p0.is_broke() {
            println!("{} is broke", p0.name());
            println!("Finally {} wins", p1.name());
            break;
        } else if p1.is_broke() {
            println!("{} is broke", p1.name());
            println!("Finally {} wins", p0.name());
            break;
        }

        match prompt("Do you want to continue? (y/n)").as_str() {
            "y" => continue,
            "n" => break,
            _ => println!("Invalid input"),
        }
    }
}

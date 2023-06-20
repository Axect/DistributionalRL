use peroxide::fuga::*;

const N: usize = 100_000;

fn main() {
    let mut rng_1 = thread_rng();
    let mut rng_2 = thread_rng();
    let safe_policy = SafePolicy;
    let quick_policy = QuickPolicy;
    let mut discount_1 = 0.99;
    let mut discount_2 = 0.99;

    let mut reward_1_vec = vec![0f64; N];
    let mut reward_2_vec = vec![0f64; N];

    for n in 0 .. N {
        let mut state_1 = State::new(0, 0);
        let mut state_2 = State::new(0, 0);
        let mut reward_1 = 0f64;
        let mut reward_2 = 0f64;
        let mut count = 0;
        loop {
            let safe_action = if discount_1 > 0f64 {
                safe_policy.get_action(&mut rng_1, state_1)
            } else {
                Action::Null
            };
            let quick_action = if discount_2 > 0f64 {
                quick_policy.get_action(&mut rng_2, state_2)
            } else {
                Action::Null
            };
            state_1.update(safe_action);
            state_2.update(quick_action);
            reward_1 += discount_1.powi(count) * state_1.get_reward();
            reward_2 += discount_2.powi(count) * state_2.get_reward();
            if state_1.is_cliff() || state_1.is_goal() {
                discount_1 = 0f64;
            }
            if state_2.is_cliff() || state_2.is_goal() {
                discount_2 = 0f64;
            }
            if discount_1 == 0f64 && discount_2 == 0f64 {
                break;
            }
            count += 1;
        }
        reward_1_vec[n] = reward_1;
        reward_2_vec[n] = reward_2;
        discount_1 = 0.99;
        discount_2 = 0.99;
    }

    let mut df = DataFrame::new(vec![]);
    df.push("safe", Series::new(reward_1_vec));
    df.push("quick", Series::new(reward_2_vec));
    df.print();
    
    df.write_parquet("cliff.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet");
}

#[derive(Debug, Clone, Copy)]
struct State {
    x: usize,
    y: usize
}

impl State {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Up => {
                if self.y < 3 {
                    self.y += 1;
                }
            }
            Action::Down => {
                if self.y > 0 {
                    self.y -= 1;
                }
            }
            Action::Left => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            Action::Right => {
                if self.x < 11 {
                    self.x += 1;
                }
            }
            Action::Null => {}
        }
    }

    fn is_cliff(&self) -> bool {
        self.x > 0 && self.x < 11 && self.y == 0
    }

    fn is_goal(&self) -> bool {
        self.x == 11 && self.y == 0
    }

    fn get_reward(&self) -> f64 {
        if self.is_cliff() {
            -1f64
        } else if self.is_goal() {
            1f64
        } else {
            0f64
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Null,
}

trait RandomPolicy {
    fn get_action(&self, rng: &mut ThreadRng, state: State) -> Action;
}

struct SafePolicy;

impl RandomPolicy for SafePolicy {
    fn get_action(&self, rng: &mut ThreadRng, state: State) -> Action {
        let x = state.x;
        let y = state.y;
        let p = rng.gen::<f64>();
        let actions = vec![Action::Up, Action::Down, Action::Left, Action::Right];

        if p < 0.25 {
            let u = rand_num(rng, 0, 3);
            actions[u]
        } else {
            match (x, y) {
                (0, y) if y < 3             => Action::Up,
                (x, 3) if x < 11            => Action::Right,
                (11, y) if y > 0            => Action::Down,
                (x, y) if x < 11 && y < 3   => Action::Up,
                _ => unreachable!(),
            }
        }
    }
}

struct QuickPolicy;

impl RandomPolicy for QuickPolicy {
    fn get_action(&self, rng: &mut ThreadRng, state: State) -> Action {
        let x = state.x;
        let y = state.y;
        let p = rng.gen::<f64>();
        let actions = vec![Action::Up, Action::Down, Action::Left, Action::Right];

        if p < 0.25 {
            let u = rand_num(rng, 0, 3);
            actions[u]
        } else {
            match (x, y) {
                (0, 0)                      => Action::Up,
                (x, 1) if x < 11            => Action::Right,
                (11, y) if y > 0            => Action::Down,
                (x, y) if x < 11 && y > 1   => Action::Down,
                _ => unreachable!(),
            }
        }
    }
}

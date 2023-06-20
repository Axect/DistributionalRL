use peroxide::fuga::*;
use rayon::prelude::*;
use indicatif::{ProgressBar, ParallelProgressIterator};

const N: usize = 1_000_000;

fn main() {
    let n_vec = linspace(1, 20, 20);

    // Fair coin
    let b = Bernoulli(0.5);

    let ((mean_vec, std_vec), max_vec): ((Vec<f64>, Vec<f64>), Vec<f64>) = n_vec.par_iter()
        .progress_with(ProgressBar::new(n_vec.len() as u64))
        .map(|n| {
        let n = n.round() as usize;
        let data = (0 .. N).map(|_| {
            let mut state = State::new();
            let mut action = Action::new();
            let coin_results = b.sample(n);
            coin_results.iter().for_each(|&x| {
                if x == 1f64 {
                    state.loss = -1f64;
                } else {
                    state.update(action);
                }
                action.update(state);
            });
            state.loss
        }).collect::<Vec<_>>();
        let mean = data.mean();
        let std  = data.sd();
        let max  = data
            .into_iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        ((mean, std), max)
    }).unzip();

    let mut df = DataFrame::new(vec![]);
    df.push("n", Series::new(n_vec));
    df.push("mean", Series::new(mean_vec));
    df.push("std", Series::new(std_vec));
    df.push("max", Series::new(max_vec));
    df.print();

    df.write_parquet("martingale.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet file");
}

#[derive(Debug, Clone, Copy)]
struct State {
    loss: f64,
}

#[derive(Debug, Clone, Copy)]
struct Action {
    bet: f64,
}

impl State {
    fn new() -> Self {
        State { loss: 0.0 }
    }

    fn update(&mut self, action: Action) {
        self.loss += action.bet;
    }
}

impl Action {
    fn new() -> Self {
        Action { bet: 1.0 }
    }

    /// Martingale policy
    fn update(&mut self, state: State) {
        if state.loss > 0f64 {
            self.bet = state.loss + 1f64;
        } else {
            self.bet = 0f64;
        }
    }
}

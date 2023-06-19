use peroxide::fuga::*;
use std::env::args;

const N: usize = 100000;

fn main() {
    let rounds = args().nth(1).unwrap().parse::<usize>().unwrap();

    // Create distributions for draw cards
    let u = Uniform(1, 3);
    let raise_j = Bernoulli(1f64/3f64); // For raise when J is drawn
    let raise_q = Bernoulli(0f64);      // For raise when Q is drawn
    let raise_k = Bernoulli(1f64);      // For raise when K is drawn
    let call_j  = Bernoulli(0f64);      // For call when J is drawn
    let call_q  = Bernoulli(2f64/3f64); // For call when Q is drawn
    let call_k  = Bernoulli(1f64);      // For call when K is drawn

    // Create Bernoulli distribution for call & raise (for opponent)
    let b = Bernoulli(0.5);

    let mut history = vec![0f64; N];

    for i in 0 .. N {
        let mut win = 0f64;

        for _ in 0 .. rounds {
            if win.abs() >= 10f64 {
                break;
            }
            let mut pot = 0f64;
            let bet = 1f64;
            win -= bet;
            pot += 2f64 * bet;
            // Draw cards
            let c = u.sample(2);
            let c1 = c[0] as usize;
            let c2 = c[1] as usize;

            // Player's action
            // 1. Raise
            let p = {
                let draw = match c1 {
                    1 => raise_j.sample(1)[0],
                    2 => raise_q.sample(1)[0],
                    3 => raise_k.sample(1)[0],
                    _ => unreachable!(),
                };
                draw == 1f64
            };
            if p && win.abs() < 10f64 - 2f64 * bet {               // Raise
                // Opponent's action
                let q = b.sample(1)[0];
                if q == 1f64 {                              // Raise -> Call
                    win -= bet;
                    pot += 2f64 * bet;

                    win += check(c1, c2, pot);
                } else {                                    // Raise -> Fold
                    win += pot;
                }
            } else {                                        // Check
                let q = b.sample(1)[0];
                if q == 1f64 && win.abs() < 10f64 - bet {   // Check -> Raise
                    let call_draw = match c1 {
                        1 => call_j.sample(1)[0],
                        2 => call_q.sample(1)[0],
                        3 => call_k.sample(1)[0],
                        _ => unreachable!(),
                    };
                    if call_draw == 1f64 {                  // Check -> Raise -> Call
                        win -= bet;
                        pot += 2f64 * bet;

                        win += check(c1, c2, pot);
                    }
                } else {                                    // Check -> Check
                    win += check(c1, c2, pot);
                }
            }
        }
        history[i] = if win > 10f64 {
            10f64
        } else if win < -10f64 {
            -10f64
        } else {
            win
        };
    }

    let mut df = DataFrame::new(vec![]);
    df.push("win", Series::new(history));
    df.print();

    df.write_parquet("result.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet file");
}

fn check(c1: usize, c2: usize, pot: f64) -> f64 {
    if c1 < c2 {
        return 0f64;
    } else if c1 == c2 {
        return pot / 2f64;
    } else {
        return pot;
    }
} 

// use candle_core;
use super::tokenizer;

pub fn run(input: &str) -> Result<(), std::io::Error> {
    let batch_size: usize = 32;
    let block_size: usize = 8;
    let max_iters: usize = 3000;
    let eval_interval: usize = 300;
    let learning_rate = 1e-2;
    let eval_iters: usize = 200;


    Ok(())
}
// use candle_core;
use super::tokenizer;

use candle_core::{Result, Device, DType, Tensor};
use candle_nn::{embedding, Embedding, Module, VarBuilder, VarMap};

struct BigramLanguageModel {
    token_embedding_table: Embedding,
    var_map: VarMap
}

impl BigramLanguageModel {
    fn new(
        vocab_size: usize,
        num_embdeddings: usize,
    ) -> BigramLanguageModel {
        let var_map = VarMap::new();
        let var_builder = VarBuilder::from_varmap(&var_map, DType::F32, &Device::Cpu);
        let token_embedding_table = embedding(
            vocab_size,
            num_embdeddings,
            var_builder.push_prefix("token_embedding"),
        ).unwrap();
        BigramLanguageModel { token_embedding_table, var_map }
    }

    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let (_, time_size) = xs.shape().dims2()?;

        let token_embedding = self.token_embedding_table.forward(xs)?; // (B, T, C)

        // let position_embedding = self.position_embedding_table.forward(&Tensor::arange(
        //     0,
        //     time_size as u32,
        //     xs.device(),
        // )?)?; // (T,C)
        todo!()
    }
}

pub fn run(
    input: &str, 
    // device: &Device
) -> Result<(), std::io::Error> {
    let batch_size: usize = 32;
    let block_size: usize = 8;
    let max_iters: usize = 3000;
    let eval_interval: usize = 300;
    let learning_rate = 1e-2;
    let eval_iters: usize = 200;
    let num_embdeddings: usize = 64;

    let tokenizer = tokenizer::Tokenizer::new(input);
    let data = tokenizer.encode(&input);

    let train_size = data.len()*0.9 as usize;
    let train_data = &data[0..train_size];
    let val_data = &data[train_size..];



    todo!()
}
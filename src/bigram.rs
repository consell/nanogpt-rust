// use candle_core;
use super::tokenizer;

use candle_core::{Result, Device, DType, Tensor};
use candle_nn::{embedding, Embedding, Module, VarBuilder, VarMap};

const BATCH_SIZE: usize = 32;
const NUM_EPOCHS: usize = 3000;
const NUM_EMBEDDINGS: usize = 64;

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
        let logits = self.token_embedding_table.forward(xs); // (B, T, C)
        logits
    }

    fn train(&self, dataset: &[usize]) {
    }
}

pub fn run(
    input: &str, 
    // device: &Device
) -> Result<(), std::io::Error> {
    let block_size: usize = 8;
    let eval_interval: usize = 300;
    let learning_rate = 1e-2;
    let eval_iters: usize = 200;
    let num_embdeddings: usize = 64;

    let tokenizer = tokenizer::Tokenizer::new(input);
    let data = tokenizer.encode(&input);

    let train_size = data.len()*0.9 as usize;
    let train_data = &data[0..train_size];
    let val_data = &data[train_size..];

    // let mut model = BigramLanguageModel::new();



    todo!()
}
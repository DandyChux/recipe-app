use candle_core::*;
use hf_hub::api::tokio::Api;
use candle_nn::{Linear, Module};

pub async fn generate_embeddings(input: Vec<String>) -> Vec<f32> {
    let api = Api::new().unwrap();
    let repo = api.model("bert-base-uncased".to_string());

    let weights_filename = repo.get("model.safetensors").await.unwrap();

    let weights = safetensors::load(weights_filename, &Device::Cpu).unwrap();

    let weight = weights.get("bert.encoder.layer.0.attention.self.query.weight").unwrap();
    let bias = weights.get("bert.encoder.layer.0.attention.self.query.bias").unwrap();

    let linear = Linear::new(weight.clone(), Some(bias.clone()));

    let input_ids = Tensor::zeros((3, 768), DType::F32, &Device::Cpu).unwrap();
    let output = linear.forward(&input_ids).unwrap();

    println!("{:?}", output);

    vec![]
}
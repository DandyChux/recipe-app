use candle_core::*;
use candle_nn::VarBuilder;
use hf_hub::{api::tokio::Api, Repo, RepoType};
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use tokenizers::Tokenizer;
use common::schema::feedback::ErrorResponse;
use linfa::prelude::*;
use linfa_clustering::{KMeans, KMeansParams};
use ndarray::*;

pub struct BertEmbeddingsModel {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
    embeddings: Tensor,
}

impl BertEmbeddingsModel {
    pub async fn new(
        model_name: &str,
        revision: Option<&str>,
        embeddings_filename: &str,
        embeddings_key: &str,
    ) ->  Result<Self> {
        let device = Device::Cpu;
        // let embeddings = Tensor::zeros((1, 768), DType::F32, &device).unwrap();
        let embeddings = match embeddings_filename.is_empty() {
            true => {
                println!("no file name provided. embeddings return an empty tensor");
                Tensor::new(&[0i64, 0i64], &device)?
            }
            false => {
                let tensor_file = safetensors::load(embeddings_filename, &device)?;
                tensor_file
                    .get(embeddings_key)
                    .expect("error getting key: embedding")
                    .clone()
            }
        };
        println!("loaded embedding shapes: {:?}", embeddings.shape());

        // Start loading the model from the hub
        let repo = Repo::with_revision(model_name.parse().unwrap(), RepoType::Model, revision.unwrap().parse().unwrap());
        let api = Api::new().unwrap();
        let api = api.repo(repo);
        let config_filename = api.get("config.json").await.unwrap();
        let tokenizer_filename = api.get("tokenizer.json").await.unwrap();
        let weights_filename = api.get("model.safetensors").await.unwrap();
        // load the model config
        let config = std::fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config).unwrap();
        // load the tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_filename)
        .map_err(|e| ErrorResponse::new(e.to_string().as_str())).unwrap();
        // load the model
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? };
        let model = BertModel::load(vb, &config)?;

        Ok(Self {
            model,
            tokenizer,
            device,
            embeddings,
        })
    }

    pub fn generate_embeddings(&mut self, input: Vec<String>) -> Result<Tensor> {
        println!("generate_embeddings: sentences.len(): {:?}", input.len());
        let tokens = self
            .tokenizer
            .encode(input, true)
            .map_err(|e| ErrorResponse::new(e.to_string().as_str())).unwrap();
        let token_ids = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;
        let start = std::time::Instant::now();
        let embeddings = self.model.forward(&token_ids, &token_type_ids)?;
        println!("time taken for forward: {:?}", start.elapsed());
        println!("embeddings: {:?}", embeddings);
        let embeddings = Self::apply_max_pooling(&embeddings)?;
        let embeddings = Self::l2_normalize(&embeddings)?;
        println!(
            "generate_embeddings completed - shape: {:?}",
            embeddings.shape()
        );
        Ok(embeddings)
    }

    pub fn apply_max_pooling(embeddings: &Tensor) -> Result<Tensor> {
        Ok(embeddings.max(1)?)
    }

    pub fn l2_normalize(embeddings: &Tensor) -> Result<Tensor> {
        Ok(embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?)
    }
}
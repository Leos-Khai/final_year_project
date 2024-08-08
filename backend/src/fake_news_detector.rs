use crate::models::post_model::Post;
use onnxruntime::environment::Environment;
use onnxruntime::ndarray::Array2;
use onnxruntime::session::Session;
use onnxruntime::tensor::OrtOwnedTensor;
use onnxruntime::GraphOptimizationLevel;
use std::error::Error;
use std::sync::Arc;
use tokenizers::Tokenizer;

pub struct FakeNewsDetector {
    _environment: Arc<Environment>, // Store the environment to keep it alive
    session: Box<Session<'static>>, // Use Box to handle lifetimes flexibly
    tokenizer: Tokenizer,
}

impl FakeNewsDetector {
    pub fn new(
        model_path: &str,
        tokenizer_path: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Initialize the ONNX Runtime environment
        let environment = Arc::new(
            Environment::builder()
                .with_name("fake_news_detector")
                .with_log_level(onnxruntime::LoggingLevel::Warning)
                .build()?,
        );

        // Clone the environment to ensure it lives long enough
        let environment_clone = environment.clone();

        // Load the ONNX model
        let session = environment_clone
            .new_session_builder()?
            .with_optimization_level(GraphOptimizationLevel::Basic)?
            .with_model_from_file(model_path)?;

        // Convert session to Box<Session<'static>>
        let session =
            Box::new(unsafe { std::mem::transmute::<Session<'_>, Session<'static>>(session) });

        // Load the tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path)?;

        Ok(FakeNewsDetector {
            _environment: environment,
            session,
            tokenizer,
        })
    }

    pub fn validate_post(
        &mut self,
        post: &Post,
    ) -> Result<(String, f32, f32), Box<dyn Error + Send + Sync>> {
        let encoding = self.tokenizer.encode(post.post_content.as_str(), true)?;

        // Convert input IDs and attention mask to vectors
        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let attention_mask: Vec<i64> = encoding
            .get_attention_mask()
            .iter()
            .map(|&mask| mask as i64)
            .collect();

        // Ensure the input length is 128 (pad if necessary)
        let mut input_ids_padded = input_ids.clone();
        input_ids_padded.resize(128, 0);
        let mut attention_mask_padded = attention_mask.clone();
        attention_mask_padded.resize(128, 0);

        // Convert the input data to ndarray arrays
        let input_ids_array = Array2::from_shape_vec((1, 128), input_ids_padded)?;
        let attention_mask_array = Array2::from_shape_vec((1, 128), attention_mask_padded)?;

        // Run the model
        let outputs: Vec<OrtOwnedTensor<f32, _>> = self
            .session
            .run(vec![input_ids_array.into(), attention_mask_array.into()])?;

        // Process the outputs
        let output_tensor = &outputs[0];
        let output_array = output_tensor
            .as_slice()
            .ok_or("Failed to convert tensor to slice")?;
        println!("Logits: {:?}", output_array);

        // Convert logits to probabilities
        let probabilities = softmax(output_array);
        println!("Probabilities: {:?}", probabilities);

        // Determine the predicted class and probabilities
        let (predicted_class, fake_prob, real_prob) = if probabilities[0] > probabilities[1] {
            ("Real News".to_string(), probabilities[1], probabilities[0])
        } else {
            ("Fake News".to_string(), probabilities[0], probabilities[1])
        };

        Ok((predicted_class, fake_prob, real_prob))
    }
}

// Softmax function to convert logits to probabilities
fn softmax(logits: &[f32]) -> Vec<f32> {
    let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|&x| (x - max_logit).exp()).collect();
    let sum_exps: f32 = exps.iter().sum();
    exps.iter().map(|&x| x / sum_exps).collect()
}

use onnxruntime::environment::Environment;
use onnxruntime::ndarray::Array2;
use onnxruntime::tensor::OrtOwnedTensor;
use onnxruntime::GraphOptimizationLevel;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokenizers::Tokenizer;

#[derive(Serialize, Deserialize)]
struct Post {
    text: String,
}

// Softmax function to convert logits to probabilities
fn softmax(logits: &[f32]) -> Vec<f32> {
    let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|&x| (x - max_logit).exp()).collect();
    let sum_exps: f32 = exps.iter().sum();
    exps.iter().map(|&x| x / sum_exps).collect()
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the ONNX Runtime environment
    let environment = Environment::builder()
        .with_name("fake_news_detector")
        .with_log_level(onnxruntime::LoggingLevel::Warning)
        .build()?;

    // Load the ONNX model
    let mut session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_model_from_file("bert_fake_news_detector.onnx")?;

    // Load the tokenizer
    let tokenizer = Tokenizer::from_file("tokenizer.json")?;

    // Example social media posts
    let social_media_posts = vec![
        Post {
            text: String::from("This is a sample social media post."),
        },
        Post {
            text: String::from("Another example post that could be real or fake news."),
        },
        Post {
            text: String::from("Breaking news: Scientists have discovered a cure for cancer that the government doesn't want you to know about!"),
        },
        Post {
            text: String::from("Shocking: New study shows that vaccines cause more harm than good."),
        },
        Post {
            text: String::from("Amazing! You won't believe what this celebrity did to stay young forever!"),
        },
        Post {
            text: String::from("Did you know? Drinking bleach can cure COVID-19 according to a recent claim."),
        },
    ];

    // Tokenize and prepare inputs
    for post in social_media_posts {
        let encoding = tokenizer.encode(post.text, true).unwrap();

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
        let outputs: Vec<OrtOwnedTensor<f32, _>> =
            session.run(vec![input_ids_array.into(), attention_mask_array.into()])?;

        // Process the outputs
        let output_tensor = &outputs[0];
        let output_array = output_tensor
            .as_slice()
            .ok_or("Failed to convert tensor to slice")?;
        println!("Logits: {:?}", output_array);

        // Convert logits to probabilities
        let probabilities = softmax(output_array);
        println!("Probabilities: {:?}", probabilities);

        // Determine the predicted class
        let predicted_class = if probabilities[0] > probabilities[1] {
            "Real News"
        } else {
            "Fake News"
        };
        println!("Predicted Class: {}", predicted_class);
    }

    Ok(())
}

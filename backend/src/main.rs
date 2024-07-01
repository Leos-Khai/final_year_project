mod fake_news_detector;
mod post;

use crate::fake_news_detector::FakeNewsDetector;
use crate::post::Post;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Create an instance of FakeNewsDetector
    let mut detector = FakeNewsDetector::new("bert_fake_news_detector.onnx", "tokenizer.json")?;

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

    // Validate each post
    for post in social_media_posts {
        let result = detector.validate_post(&post)?;
        println!("Post: {}", post.text);
        println!("Predicted Class: {}", result);
    }

    Ok(())
}

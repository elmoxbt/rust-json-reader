use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct Paragraph {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}


fn main() {
    let data = r#"
    {
        "article": "Rust Programming",
        "author": "elmoxbt",
        "paragraph": [
            { "text": "Rust is a systems programming language." },
            { "text": "It is designed for performance and safety." }
        ]
    }"#;

    let parsed: Article = read_json_typed(data);
    println!("The first paragraph from the article '{}' is {}", parsed.article, parsed.paragraph[0].text);
}

fn read_json_typed(raw_json: &str) -> Article {
    match serde_json::from_str(raw_json) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            std::process::exit(1);
        }
    }
}
use mdbook_content_collections::build_content_index;
use serde_json::Value;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("--version")
        || args.get(1).map(|s| s.as_str()) == Some("-V")
    {
        println!("mdbook-content-collections {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        println!("true");
        return;
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let input_array: Vec<Value> = serde_json::from_str(&input).expect("Invalid JSON from mdBook");
    let context = &input_array[0];
    let book = &input_array[1];

    let root = context
        .pointer("/root")
        .and_then(|v| v.as_str())
        .unwrap_or(".");

    let src_dir = PathBuf::from(root).join("src");

    // Write directly into src/ → gets copied to book/ automatically
    let index_path = src_dir.join("content-collections.json");

    eprintln!("Writing content index to: {}", index_path.display());

    if let Err(e) = build_content_index(&src_dir, &index_path) {
        eprintln!("ERROR: Failed to build content index: {e}");
        std::process::exit(1);
    }

    // Success confirmation (optional, nice for debugging)
    if index_path.exists() {
        let size = index_path.metadata().map(|m| m.len()).unwrap_or(0);
        eprintln!("Content index written successfully ({} bytes)", size);
    }

    // Echo back the unchanged book
    println!("{}", serde_json::to_string(book).unwrap());
}

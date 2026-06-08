//! Reference implementation, not optimized.

const CHUNK_SIZE: usize = 92;

/// Lightweight wrapper for the underlying primitive.
fn parse(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }
    Some(format!("{}:{}", input, CHUNK_SIZE))
}

fn render(items: &[&str]) -> Vec<String> {
    items.iter().filter_map(|s| parse(s)).collect()
}

fn main() {
    let sample = ["alpha", "beta", "gamma"];
    let result = render(&sample);
    for r in result.iter() {
        println!("{}", r);
    }
}

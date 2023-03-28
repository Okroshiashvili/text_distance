![Cargo Build & Test](https://github.com/Okroshiashvili/text_distance/actions/workflows/ci.yml/badge.svg)


# Text Distance

---

> **⚠ WARNING:**
> **This is a work in progress. The API is not optimized and stable yet.**

> **Side Note:**
> I've started this project to learn Rust. If you see something that is very odd or is in dire need of improvement please let me know!

---

**Text Distance** - A collection of algorithms for measuring the distance between two strings.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
text_distance = "0.1.0"
```

## Example

```rust
use text_distance::Levenshtein;


fn main() {
    let lev = Levenshtein {s: "test".to_owned(),  t: "book".to_owned()};

    let plain_distance = lev.distance();
    let normalized_distance = lev.normalized_distance();
    let similarity = lev.similarity();
    let normalized_similarity = lev.normalized_similarity();

    println!("plain_distance: {}", plain_distance);
    println!("normalized_distance: {}", normalized_distance);
    println!("similarity: {}", similarity);
    println!("normalized_similarity: {}", normalized_similarity);
}
```

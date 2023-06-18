# Paperless
Paperless api client in rust.

## Example

```rust
use paperless::Paperless;

pub fn main() {
    let paperless = Paperless::new("https://example.com/paperless/api/", "thisIsAToken");
    
    for tag in paperless.tags(Default::default()) {
        println!("tag: {:?}", tag);
    }
}
```
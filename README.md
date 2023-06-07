# Bionic Reading API

Unofficial Rust library to get a Bionic Reading converted string from their Rapid API.

Allows setting fixation and saccade to values supported by the API.
For more information on Bionic Reading see the [official page](https://bionic-reading.com/).

The returned string can be used either as raw HTML straight from the response or a Markdown converted version of the HTML.

## Example

```rust
use bionic_reading_api::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = Client::new("api_key")
        .convert("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.")
        .send()
        .await?;

    let html = res.html().unwrap();
    let markdown = res.markdown().unwrap();

    println!("{html}");
    println!("{markdown}");

    Ok(())
}
```

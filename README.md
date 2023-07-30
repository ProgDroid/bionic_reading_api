[![cio-img]][cio-link] [![ci-img]][ci-link] [![doc-img]][doc-link]

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

[ci-link]: https://github.com/ProgDroid/bionic_reading_api/actions/workflows/test.yml
[ci-img]: https://github.com/ProgDroid/bionic_reading_api/actions/workflows/test.yml/badge.svg
[cio-link]: https://crates.io/crates/bionic_reading_api
[cio-img]: https://img.shields.io/crates/v/bionic_reading_api
[doc-link]: https://docs.rs/bionic_reading_api
[doc-img]: https://img.shields.io/docsrs/bionic_reading_api/latest

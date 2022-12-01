# Bowser

A fun little toy browser written in Rust. This is a learning exercise guided by [Browser Engineering](https://browser.engineering/). 

To run the progrom, make sure you have `rust` and `cargo` installed. Next, run `cd bowser` from the root and then `cargo run`. You should see a small screen with an address bar. Type your url into the address bar and hit the go button to see results. Many pages won't render due to the limitated nature of this project, here are a few example queries that we know will work:

- `http://example.org/index.html`
- `data://<b>bold</b><i>italic</i>regular`

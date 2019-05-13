# :book: tread 

tread - (**T**erminal **Read**er) is a tool to help you read documents and e-books quickly from the safety of your terminal.

## Features

- Supports multiple file-formats:
  - Plaintext (txt, markdown, etc)
  - **Coming Soon!** E-books (epub, pdf, mobi)
  - Web Formats (html)
- Toggleable Status Bar
- Pause, Rewind, Fastforward Controls
- TTR (Time to Read) Estimation
- Adjustable WPM (Words Per Minute)
- Bookmarks (Save position on exit)

## :package: Installation & :hammer: Usage

`tread` is easy to install if you have `cargo`.

```bash
cargo install tread
```

Usage is just as simple.

```bash
tread ./that-book-i-really-need-to-finish.txt
```

## Controls:

- `h`: toggle the status bar
- `r`: Restart the document
- `SPACE`: pause the reader
- `LEFT` & `RIGHT` Arrows: Move go back or forward a word.
- `UP` & `DOWN` Arrows: Adjust WPM by 50 WPM.
- `ESC` & `q`: Quits `tread`

# Defyne Data

[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://www.rust-lang.org/)

This project is not officially endorsed by the Wikitionary Project or Wikimedia Foundation. The dataset(s) produced by this project are subject to the licenses of their sources. See LICENSE.md.

## Overview

Converts a [postprocessed Kaikki Wikitionary dataset](https://kaikki.org/dictionary/) into a leaner version suited for usage in dictionary applications. This dataset is made use of by the Defyne iOS app.

To use this tool, supply a `.jsonl` Kaikki dataset as the argument.

## Compile

This project is built in Rust. Build with `cargo b`.

## Run-time Requirements

- `jq` v1.8.2 or a more recent version.

## Credits

- [Wikitionary](https://www.wiktionary.org/) data postprocessed and obtained via [Kaikki](https://kaikki.org/dictionary/English/).
- `kaggle_word_list.txt` obtained from Kaggle's redistribution of [this list by dwyl under The Unlicense](https://github.com/dwyl/english-words). 

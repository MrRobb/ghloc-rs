# 🐙 ghloc-rs

[![Crates.io](https://img.shields.io/crates/v/ghloc)](https://crates.io/crates/ghloc)
[![Docs.rs](https://docs.rs/ghloc/badge.svg)](https://docs.rs/ghloc/latest/ghloc)
[![codecov](https://codecov.io/gh/MrRobb/ghloc-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/MrRobb/ghloc-rs)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MrRobb/ghloc-rs/blob/master/LICENSE)

We have [tokei](https://crates.io/crates/tokei) and [ghloc.vercel.app](https://ghloc.vercel.app/) for counting lines of code in a GitHub repository. But what if you want to count lines of code in a GitHub repository from the command line? Well, that's what this crate is for.

> Actively maintained! If you have any problem just [create an issue](https://github.com/MrRobb/ghloc-rs/issues/new).

### Install

```bash
cargo install ghloc
```

### Usage
    
```bash
ghloc "https://github.com/MrRobb/ghloc-rs"
```

### TODO

- [ ] CI / CD:
  - [ ] Add tests.
  - [ ] Get rid of `unwrap()`.
  - [ ] Add [no_panic](https://docs.rs/no-panic/).
- [ ] Code:
  - [ ] File selection.
  - [ ] Local repository support.
  - [ ] Git authentication.
  - [ ] Add comments and blank space to the output.
- [ ] UI:
  - [ ] Inline statistics.
  - [ ] Percentages.
  - [ ] Colors for each bar (using [github-colors](https://github.com/ozh/github-colors)).

<!-- LICENSE -->
## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

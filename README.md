

# Rust Demo

Study rust project, for my dream, reference by:

- [rust-tutorial](https://www.runoob.com/rust/cargo-tutorial.html)

- [doc.rust-lang.org](https://doc.rust-lang.org/book/title-page.html)

## Definitioins

Expain mutilple noun for understand.

### Rust

safe language

### rustup

[rustup](https://github.com/rust-lang/rustup) is a command  util to instead of [multirust](https://github.com/brson/multirust) , to manage mutilple installations of the Rust toolchain.

### cargo

 compile util and package manager.

```
# like eslint
cargo colippy

# like go fmt
cargo fmt

# dependencies view
cargo tree

# benchmark
cargo bench

# check unused third party dependencies
cargo udeps

# performance compilation higher than default debug compilation.
cargo build/run --release
```



## Environment

You must install

### Rust

To check Rust env

```
rustc -V
cargo -V
```

Install

```
TODO
```

### Rustup

 You can follow [rustup.rs](https://rustup.rs/) or [tools/install](https://www.rust-lang.org/zh-CN/tools/install).

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Init

you can run this command in you blank project

```
cargo init
```

## build

you can run:

```
cargo build
```

## run

You can run:

```
cargo run
```



## Development





## Help

can help you to start faster

### IDE

- No Rust toolchain configured

  Peferences > Rust > Toolchain location

- No Cargo projects found

  config to direct the project folder, then you can handle this warn

## Common

- error[E0554]: `#![feature]` may not be used on the stable release channel

  need stable Rust

  ```
  rustup install nightly
  # or 1
  rustup default nightly
  # or 2
  rustup override set nightly
  
  ```

  
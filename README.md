# [fdiff](https://github.com/NoSpawnn/fdiff)

A simple GUI text and file diff utility

## Usage

### Build from source

1. Ensure [Rust](https://www.rust-lang.org/tools/install) is installed on your system

```shell
# On linux
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repo

```shell
$ git clone https://github.com/NoSpawnn/fdiff.git
```

3. Install dependencies

```shell
$ cd fdiff
$ pnpm i
```

4. Install [tauri-cli](https://crates.io/crates/tauri-cli)

```shell
$ cargo install tauri-cli
```

5. Build the project

```shell
$ cd fdiff
$ cargo tauri build
```

## Development

1. Ensure [Rust](https://www.rust-lang.org/tools/install) is installed on your system

```shell
# On linux
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repo

```shell
$ git clone https://github.com/NoSpawnn/fdiff.git
```

3. Install dependencies

```shell
$ cd fdiff
$ pnpm i
```

4. Run in development mode

```shell
$ pnpm tauri dev
```

## TODO

- [x] Text diff
- [ ] File diff
- [ ] User themes/Theme customisation

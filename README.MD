# | **📖 Yui**

### Looking to create your own collaborative wiki ? Then you should try this.

<br>

# **Yui the modern collaborative Wiki**

<br>

# Install dependencies

<br>

> ## Windows

- ### <u><a href="https://win.rustup.rs/">Click to install cargo</a></u>
- ### <u><a href="https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16">Click to install VS2019 build tools</a></u>
    &emsp; + `Select "Desktop development with C++`<br>
    &emsp; + `Select "Universal Windows Platform build tools`
- ### <u>`Install cargo-binutils`</u>
    ```bash
    cargo install -f cargo-binutils
    ```
- ### <u>`Install llvm tools`</u>
    ```bash
    rustup component add llvm-tools-preview
    ```

<br>

> ## Linux

- ### <u>`Install cargo`</u>
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
- ### <u>`Install clang and lld`</u>
    > **Ubuntu:** ``sudo apt-get install clang lld``

    > **Arch:** ``sudo pacman -S clang lld``

<br>

# How to build ? 

```bash
cargo build --release
```

<br>

# How to run ?

```bash
cargo run --release
```

<br>

### Notes

#### This wiki is using the wonderful *`rocket.rs`* framework

<br>

# <u>_**Made with ❤️ by RqndomHax**_</u>

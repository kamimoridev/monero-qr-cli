# Monero QR CLI

A simple command-line tool to generate a QR code from a Monero wallet address.  
Supports saving as a PNG image or printing the QR code directly to the terminal.

## Features
- Generate PNG with transparent or solid white background
- Display QR code in ASCII directly in the terminal
- Automatically adds `monero:` prefix to the address if missing
- Minimal dependencies, fast execution

## Installation
Requires [Rust](https://www.rust-lang.org/) installed.


git clone https://github.com/kamimoridev/monero-qr-cli.git
cd monero-qr-cli
cargo build --release


The compiled binary will be located at:

target/release/monero-qr-cli


You may want to add it to your `PATH` for global use.

## Usage


# Show QR code in the terminal
monero-qr-cli 4Ad...Your_Monero_Address...

# Save QR code as PNG
monero-qr-cli 4Ad...Your_Monero_Address... -o qr.png


If the provided address does not include the `monero:` prefix, it is added automatically.

## Examples

**QR code in terminal:**
```
▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
█ ▄▄▄▄▄ █▀▄█▀▀▀▀▄▀▄▀▀▀ ▄ ▀ █ ▀▄▄█ ▀▀█ ▀▀▀ █ ▄▄▄▄▄ █
█ █   █ █▄▀█▄▀▄ ▄█▀███▄█ █▄▄████ ▄██▀▀ █ ▄█ █   █ █
█ █▄▄▄█ ███ ▄ ▀▀▄█▄▄▄█  ▄▄▄ █▄██▀▀ ▀▀██▀███ █▄▄▄█ █
█▄▄▄▄▄▄▄█▄▀▄█▄▀ ▀▄█ ▀ █ █▄█ █ █▄▀ ▀ █ ▀ ▀ █▄▄▄▄▄▄▄█
█▀▄ ▄█▄▄███  ▀ ▄ ▄ ▀▄▀▄▄▄▄  █  ▀ █▄▄███▄▄▄▀▀▀▀█▄ ▀█
██▄▀█▄▄▄█▀ █▄▀▀  ▄▀▄▀▄ █▀ ██▄▄█▀▀▄▄▄▀▀▄██ ▄▀█▄▀ ▀██
█▀██▀▄ ▄▀ ▄▀▀▀▄ ▀     ▀▄  ▀█▀▄▄█▄█ ██ ▄▀█▄██ ▀▀▀▄▀█
█▄▀█ █▀▄█  ▀█▀▄▀▀█▀ ▀▄█  ▀█▄█▄▀█▀▀▀██ ▀▄▄▀█ ██▀▄▄▀█
█▄▀▀█ █▄ ▄▄ ▄▀█▀▄██▀  █ ▄▀█▄▄   █▄  █▀▄▄ ▀▀█ ▄ ▄▀██
██▀▄ ██▄▄ ▀█ █▄ ▀▀█▀█▄▀▀▄▀  ▀▄▀ ▀ ▄▀█▀█▄▄▀█▄█▄▀█▄▄█
█▄ █ ▄ ▄▄▄ ▀▀▀███▄ ▀▀▀▀█▄ ▀▀▄▄ ▀██▄███▄▄ █ ▀█▀▀██▄█
██▄█▀ ▄▄▄  ▀▄███▄█  █▄  ▄▄▄ ▀ █ ▀ █▀██ █▀ ▄▄▄ ▀▀█▀█
█ ▀▄█ █▄█ ▀▄█ ▄▄██   █▀ █▄█  ███ ▀ ▄▄▄▀   █▄█ ▀▄ ▄█
█▀██▄▄ ▄  ▀ ██▄ █ ▄▀█▀ ▄ ▄ ▄▀▄▀█▄███  ▄▄▀  ▄  █▄ ▀█
█▀█▀███▄▀▄▄█▀ ███ ██ ▀█▀▄▀▀▄▄█ █ ▄▀ █▀▀█▀ ▀▀ █▄██▄█
█▀█ ▄▄▀▄▄ ▀█▀ ▄█▄█▀▀█▄▄▄██▀ █▄▀█▄▀ ▄▄    ▀     ▄▄ █
█▀█ ▄█▀▄   ▄▀██▀▄ ███▄█▄█  ▄ ▀ ▀█ ▄█▄▀█▄▀▄█▄ █ ██▄█
█▄  ▀▀█▄▀ ▀▀▀▄██  █▄█▀▄▄▄███▄▄▄▀▄▀▀ ▄▄ █ █ ▀  ▀▀▀██
█▀▄████▄▄▄▀▀▀▄▀▀ ▄▀▀▄ ▄▄█▄█▄█▀█▄▀█ ▄▄    ▄█▄▄ ▄▄ ██
██ ▀▀█▄▄█▀ ▄▀▄██ ▄██▄███ █▀█ ▄█  █▄█▀  █▄ ▄▀█▀▀█▄▀█
█▄▄▄███▄█ █   ▀▄▀▀  ▄▀▄ ▄▄▄ ▄███▄ █▀▀▀▀▄█ ▄▄▄  ▄▄ █
█ ▄▄▄▄▄ █▀▀█▀  ██▄ ██▄▄ █▄█ █ ▄▄▀█ ██▄▄▄█ █▄█ ▀▄▄▄█
█ █   █ █▀▀ ▄█▀▀█  ▄ ▀▄▄      ▀ ▀▀██ ▀▄█▀▄    ▀█▄██
█ █▄▄▄█ █ ▀█▄█▀  █▄ ██▀ ▄ ▀▀ █ █ ▀▄ ██▄█ ▀▀▄▀   ███
█▄▄▄▄▄▄▄███▄▄▄█▄▄███▄▄█▄█▄▄▄▄██▄▄███▄▄▄██▄██▄█▄▄▄▄█
```


**PNG output (transparent background):**  
![PNG Example](screenshots/example.png)

## Options

-o, --output <FILE>     Save QR code as a PNG file to the specified location
<ADDRESS>               Monero wallet address


## License
MIT

## Donate if you want (yes, its the same address)
86UKFtJivEiPshGkbReMLraabZD8DtV5Z8mR5V9VopRLfmmZy3ACG73ZMwcY5QNKFPiJgj53KK8fs6KHfnML271MHUd8v3D
monero:86UKFtJivEiPshGkbReMLraabZD8DtV5Z8mR5V9VopRLfmmZy3ACG73ZMwcY5QNKFPiJgj53KK8fs6KHfnML271MHUd8v3D
![donation monero address](screenshots/example.png)

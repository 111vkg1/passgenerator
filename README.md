# passgenerator
Rust-written password generator
### Download
```
git clone https://github.com/111vkg1/passgenerator.git
cd passgenerator
```
### Build & run
```
cargo build --release

./target/release/passgenrust
```
#### or
```
cargo run
```

### Usage
passgenrust :
First prompt (4-32) - Password len
Second prompt (y/n) - Include special syms
Example session:

    Password generator on rust
    Input len [4-32]: 16
    Include special(!@#$%^&*) [y/n]: y
    Password: aB3$xY9kLmNpQ7Zt
passgen :
passgen -l <LEN> -s (specials)

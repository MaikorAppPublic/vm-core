# Maikor

>⚠️ Work in progress
>
> Links may be broken, features missing, etc

*Cross platform 16 bit game system*

See more at [maikor.app](https://maikor.app) and the [project homepage](https://github.com/MaikorAppPublic)

### Play

[Android](https://github.com/MaikorAppPublic/android-app)

[iOS](https://github.com/MaikorAppPublic/ios-app)

[Windows, macOS and Linux](https://github.com/MaikorAppPublic/desktop-app)

### Make

[iOS IDE](https://github.com/MaikorAppPublic/ios-app)

[Desktop IDE](https://github.com/MaikorAppPublic/desktop-ide)

[Build tools](https://github.com/MaikorAppPublic/build-tools)

[REPL](https://play.vm.maikor.app)

## vm-core

This is a library that executes Maikor game files in a VM. It won't run by itself though, instead it requires an external program to manage timing and frame rate to keep the code execution speed similar between platforms.

### Usage

Ideally use one of the `vm-interface` libraries

However, this is all that's need to run a game:
```rust
//read file
let maikor_game = read_file();
//create an instance
let mut vm = VM::new();
//load the game
vm.init(maikor_game); 
//then
loop {
    vm.step();
}
```
*(but most games need user input to work)*
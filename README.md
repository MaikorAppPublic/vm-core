# Maikor

*Cross platform 16 bit game system*

See more at [maikor.app](https://maikor.app)

### Play

[Android](https://github.com/MaikorAppPublic/android-app)

[iOS](https://github.com/MaikorAppPublic/ios-app)

[Windows, macOS and Linux](https://github.com/MaikorAppPublic/desktop-app)

### Make

iOS IDE

Desktop IDE

Build tools

### Project breakdown

#### Major
* [vm-core](https://github.com/MaikorAppPublic/vm-core)
  * Executes Maikor games
* [vm-interface](https://github.com/MaikorAppPublic/vm-interface)
  * Acts as hardware emulation layer for Maikor, it converts VM memory into graphics and inputs into VM memory 
* [desktop-app](https://github.com/MaikorAppPublic/desktop-app)
  * Host program for Windows, macOS and Linux
* [android-app](https://github.com/MaikorAppPublic/android-app)
  * Host program for Android
* [ios-app](https://github.com/MaikorAppPublic/ios-app)
  * Host program for iOS

#### Minor
* [vm-interface-android](https://github.com/MaikorAppPublic/vm-interface-android)
  * Android compatible wrapper for `vm-interface`
* [vm-interface-ios](https://github.com/MaikorAppPublic/vm-interface-ios)
  * iOS compatible wrapper for `vm-interface`
* [vm-desktop-simple](https://github.com/MaikorAppPublic/vm-desktop-simple)
  * Simple desktop program for testing Maikor games (it can't save, etc)
* [vm-file](https://github.com/MaikorAppPublic/vm-file) 
  * For reading and writing Maikor game files 
 

## maikor-vm-core

This is a library that executes Maikor game files in a VM. It won't run by itself though, instead it requires an external program to manage timing and vsync to keep the code execution speed similar between platforms.

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
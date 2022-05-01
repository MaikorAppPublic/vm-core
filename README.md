# Maikor

*Cross platform 16 bit game console*

See more at [maikor.app](https://maikor.app)

### Play

[Android](https://github.com/raybritton/maikor-vm-android)

[iOS](https://github.com/raybritton/maikor-vm-ios)

[Windows, macOS and Linux](https://github.com/raybritton/maikor-vm-desktop)

### Make

iOS IDE

Desktop IDE

Build tools

### Project breakdown

#### Major
* [maikor-vm-core](https://github.com/raybritton/maikor-vm-core)
  * Executes Maikor games
* [maikor-vm-interface](https://github.com/raybritton/maikor-vm-interface)
  * Acts as hardware emulation layer for Maikor, it converts VM memory into graphics and inputs into VM memory 
* [maikor-desktop](https://github.com/raybritton/maikor-desktop)
  * Host program for Windows, macOS and Linux
* [maikor-android](https://github.com/raybritton/maikor-android)
  * Host program for Android
* [maikor-ios](https://github.com/raybritton/maikor-ios)
  * Host program for iOS

#### Minor
* [maikor-vm-interface-android](https://github.com/raybritton/maikor-vm-interface-android)
  * Android compatible wrapper for `maikor-vm-interface`
* [maikor-vm-interface-ios](https://github.com/raybritton/maikor-vm-interface-ios)
  * iOS compatible wrapper for `maikor-vm-interface`
* [maikor-vm-desktop-simple](https://github.com/raybritton/maikor-vm-desktop-simple)
  * Simple desktop program for testing Maikor games (it can't save, etc)
* [maikor-file](https://github.com/raybritton/maikor-file) 
  * For reading and writing Maikor game files 
 

## maikor-vm-core

This is a library that executes Maikor game files in a VM. It won't run by itself though, instead it requires an external program to manage timing and vsync to keep the code execution speed similar between platforms.

### Usage

Ideally use one of the `maikor-vm-interface` libraries

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
## Pre-alpha

### 0.1.15
- Don't copy changes into code and atlas banks
- Fail if attempting to write in

### 0.1.14
- Fix bug with loading game

### 0.1.13
- Update dependencies
  - File to 0.1.9

### 0.1.12
- Update dependencies
  - Language to 0.1.25
- Controller banking now works

### 0.1.11
- Add support for loading game files

### 0.1.10
- Update dependencies
  - Language to 0.1.22
- Fix bugs:
  - Incorrect PPID change amounts
  - Save bank not loaded on init
  - MSWP only swapping first byte
  - Atlas banks 3 and 4 not loading
- Add bitwise methods (OR, AND, etc)
- Add stack methods (PUSH, CALL, etc)

### 0.1.9
- Add support for sound
- Add support for interrupts
- Update dependencies
  - Language to 0.1.22

### 0.1.8
- *BREAKING CHANGE*
- Fix bug with atlas bank swapping
- Update dependencies
  - Language to 0.1.21

### 0.1.7
- Fix `debug_set_mem_range`
- Change sprite table to start as all 0s

### 0.1.6

- *BREAKING CHANGE*
- Change how registers and memory are accessed internally
- Add `halted` and `error` to VM
- Add `executed_ops` and `executed_cycles` to VM
- Add `init()` to VM
- Improve invalid register error handing
- Fix `NOP` bug
- Add more tests
- Add `EHALT`, `JMP`
- Update dependencies
  - Language to 0.1.19
- Fix jump bugs  
- Saves dirty flags are affected by `SAVE_CONTROL`

**Register/Memory/Type changes**

Originally the three main value types the VM used (bytes, words and addresses) were types and most methods have generic implementations for those types, however this was causing a massive performance hit.

On an M1 Mac with 0.1.5 code, a benchmark program (~750k ops / 1.5m cycles) took ~77ms to run, now with the changes below it takes ~8ms. (Debug times went from 700ms to 33ms).

Main changes:
 - Remove all uses of VecDeque
 - Remove all generic methods

Unfortunately adding support for index addressing increases the average time for the benchmark to ~9ms.

### 0.1.5

- *BREAKING CHANGE*
- Update dependencies
  - Language to 0.1.11
- Fix `MUL`
- Fix bugs with flags
  - Signed flag is now correctly set
- Fix `CMPS.B (R,R)`

### 0.1.4

- *BREAKING CHANGE*
- Fix `INC/DEC` to only change by 1 
- Fix clear_flag bug
- Change `INC/DEC` to use wrapping_math methods (only affects debug builds)

### 0.1.3

- Update language dep
- Move register offsets to VM
  - As language registers should be independent of VM implementation 

### 0.1.2

- Update language dep

### 0.1.1

- *BREAKING CHANGE*
- Added commands
  - MUL
  - MULS
  - DIV
  - DIVS
  - JMP
  - JE
  - JNE
  - JL
  - JG
  - JGE
  - JLE
  - JBS
  - JBC
  - XOR
  - OR
  - AND
  - NOT
  - SWAP
  - CMP
  - CMPS
  - ADDC
  - SUBC
  - MCPY
  - NOP
  - HALT
- Moves constants to `maikor-language` lib
- All methods are untested, this is being published to remove blocks on other projects

### 0.1.0

- Initial release
- Added commands
  - INC
  - DEC
  - CPY
  - ADD
  - SUB
- Setup register and memory format
- Setup graphics limits and format
- Added basic execution step
- Setup internal types (Byte, Word, Address)
- Added some unit and integration tests
- Add README and CHANGELOG
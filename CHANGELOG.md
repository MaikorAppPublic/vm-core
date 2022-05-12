## Pre-alpha

### 0.1.5

- *BREAKING CHANGE*
- Update dependencies
  - Language to 0.1.11
- Fix MUL
- Fix bugs with flags
  - Signed flag is now correctly set
  - Flags are no longer set when results are written to memory
    - This is controlled by `mem_change_affects_flags` in the `VM`
- Fix CMPS.B (R,R)

### 0.1.4

- *BREAKING CHANGE*
- Fix INC/DEC to only change by 1 
- Fix clear_flag bug
- Change INC/DEC to use wrapping_math methods (only affects debug builds)

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
## Pre-alpha

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
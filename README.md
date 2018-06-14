# tiger-rs

Following along with Andrew Appel's [*Modern Compiler Implementation in ML*][1].

### Progress

- [x] Lexing

Handwritten lexer.

- [x] Parsing

Using [LALRPOP][2] as an LR(1) parser generator.

- [x] Type checking

- [ ] IR generation

### TODO

- Lexing
  - [x] Add pretty-printing for tokens
  - [x] Decouple lexer from parser
  - [x] Integrate lexing phase into CLI
  - [ ] Implement string unescaping
  - [ ] Handle MIN\_INT - might have to store literal ints as strings?
  - [x] Write test cases for lexing

- Parsing
  - [ ] Implement or find global symbol table library
  - [ ] Convert all allocated `String` fields into `usize`
  - [ ] Implement `to_span` functions for AST nodes for better errors

- Type checking
  - [ ] Write test cases
  - [ ] Check for variable mutability
  - [ ] Check for uniqueness of type and function names within a mutually recursive group
  - [ ] Upgrade `TypeError` variants with more information
  - [ ] Use `codespan::Label` to display better errors
  - [x] Possibly use macros to clean up repeated code, or reduce the number of `clone` calls

[1]: https://www.cs.princeton.edu/~appel/modern/ml/
[2]: https://github.com/lalrpop/lalrpop

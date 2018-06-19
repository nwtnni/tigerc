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
  - [x] Implement or find global symbol table library (EDIT: see [sym][3])
  - [x] Convert all allocated `String` fields into cached symbols
  - [x] Implement `to_span` functions for AST nodes for better errors
  - [x] Add more `span` fields to AST where necessary (e.g. saving function name in `Call` node)

- Type checking
  - [x] Write test cases
  - [x] Check for variable mutability
  - [x] Check for uniqueness of type and function names within a mutually recursive group
  - [ ] Check for invalid type cycles
  - [ ] Upgrade `TypeError` variants with more information
  - [ ] Use `codespan::Label` to display better errors
  - [x] Possibly use macros to clean up repeated code, or reduce the number of `clone` calls

### Deviations

- Allow comparison operators to associate (e.g. (a = b = c) evaluates as ((a = b) = c))
- Allow assignment to for loop index variable (e.g. for i := 0 to 10 do i := i + 1)

[1]: https://www.cs.princeton.edu/~appel/modern/ml/
[2]: https://github.com/lalrpop/lalrpop
[3]: https://github.com/nwtnni/sym

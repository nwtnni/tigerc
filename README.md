# tiger-rs

Following along with Andrew Appel's [*Modern Compiler Implementation in ML*][1].

### Progress

- [x] Lexing

Handwritten lexer.

- [x] Parsing

Using [LALRPOP][2] as an LR(1) parser generator.

- [x] Type checking

- [x] IR translation

- [ ] Abstract assembly generation

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

- IR translation
  - [x] Implement Appel's Tree enum
  - [x] Implement AST translation functions
  - [x] Attach AST translation to type checking phase
  - [x] Implement canonization
  - [ ] Implement interpreter for testing purposes
  - [ ] Write test cases for interpreted IR
  - [ ] Make sure commuting logic is sound (i.e. pure vs. impure expressions)
  - [x] Implement constant folding
  - [x] Implement finding escaping variables
  - [x] Implement static link traversal
  - [x] Construct control flow graph from canonized IR
  - [x] Reorder IR using control flow graph to remove unnecessary jumps

- Abstract assembly generation
  - [x] Design instruction types for assembly
  - [ ] Implement AT&T and Intel syntax in separate traits for easy swapping
  - [x] Implement tiling using maximal munch
  - [ ] Implement trivial register allocation
  - [ ] Figure out how to write a C runtime for Tiger
  - [ ] Clean up command-line interface
  - [ ] Organize compiler passes into distinct phases (maybe use a Phase trait?)

### Deviations

- Allow comparison operators to associate (e.g. (a = b = c) evaluates as ((a = b) = c))
- Allow assignment to for loop index variable (e.g. for i := 0 to 10 do i := i + 1)

[1]: https://www.cs.princeton.edu/~appel/modern/ml/
[2]: https://github.com/lalrpop/lalrpop
[3]: https://github.com/nwtnni/sym

# derivative calulator
    made this project to understand parsing

# Grammar Used
```text
expr    = term (('+' | '-') term)*
term    = factor (('*' | '/') factor)*
factor  = base ('^' factor)?
base    = NUMBER | VARIABLE | '(' expr ')' | '-' base

- expr handles addition and subtraction
- term handles multiplication and division
- factor handles exponentiation
- base handles numbers, variables, parentheses, and unary minus
```

# making this
I initially thought of using libraries to evaluate the mathematical expressions using libraries but
since i'll be dealing with symbols of some sort, maybe it's feasible if i wrote a parser/tokenizer from scratch
since I had some idea on how to write a tokenizer or parser from previous small projects.
So...
- I'll be writing a tokenizer and parser from scratch
lets see how that goes (it did not go well)

## update 1 (2025-06-21)
- currently working on the tokenizer (still) i focused more on writing better code instead of just programming it out
 I'm taking references from the docs and existing tokenizers lets see how this goes
## update 2 (2025-07-08)
- I've taken the tokenizer present on mkdb as reference for this and modified it
- I've followed the [Recursive Descent Parsing](https://en.wikipedia.org/wiki/Recursive_descent_parser)  parsing method for this project
## update 3 (2025-07-7)
- was kinda stuck on parser when it failed to include the variable 'x' in parse tree(AST), well it was a simple mistake, I forgot the BinaryOperator in between
- currently working on the parser(still)

## References
- (Structure and Interpretation of Computer Programs)[https://web.mit.edu/6.001/6.037/sicp.pdf]
- (An implementation of a database from scratch in rust)[https://github.com/antoniosarosi/mkdb]

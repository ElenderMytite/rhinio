# Quick Start Guide for Tests

## Overview
10 comprehensive test files have been created to test all features of the TinyWrite language.

## Quick Run

### Run a single test:
```bash
cargo run test_arithmetic.txt
```

### Run all tests with the test runner:
```bash
chmod +x run_tests.sh
./run_tests.sh
```

## Test Files at a Glance

| Test | Purpose | Focus |
|------|---------|-------|
| `test_lexer_basic.txt` | Basic tokenization | Numbers, operators, comments |
| `test_arithmetic.txt` | Math operations | +, -, *, /, % |
| `test_comparisons.txt` | Comparisons | >, <, ==, !=, >=, <= |
| `test_logic.txt` | Logical operations | &, \|, ^, !, !&, !\| |
| `test_variables.txt` | Variable storage | Assignment and retrieval |
| `test_expressions.txt` | Complex expressions | Nesting and evaluation |
| `test_vectors.txt` | List operations | Create, push, get, pop, len |
| `test_edge_cases.txt` | Boundary testing | Zero, negatives, large numbers |
| `test_advanced.txt` | Complex scenarios | Multi-step computation |
| `test_integration.txt` | Full integration | All features combined |

## File Locations

```
tinywrite/
├── examples/
│   ├── test_lexer_basic.txt
│   ├── test_arithmetic.txt
│   ├── test_comparisons.txt
│   ├── test_logic.txt
│   ├── test_variables.txt
│   ├── test_expressions.txt
│   ├── test_vectors.txt
│   ├── test_edge_cases.txt
│   ├── test_advanced.txt
│   ├── test_integration.txt
│   └── README.md (detailed documentation)
├── TESTS.md (this file - full test summary)
└── run_tests.sh (test runner script)
```

## What Each Test Covers

### test_lexer_basic.txt
Tests tokenization of all language elements:
- Positive integers: `42`, `-100`, `999`
- Variables: `x`, `my_var`, `result`
- All operators: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, etc.
- Comments: `# this is ignored`

### test_arithmetic.txt
All arithmetic operations with various operand counts:
- `(+ 5 3)` - Addition
- `(- 10 3)` - Subtraction
- `(* 4 5)` - Multiplication
- `(/ 20 4)` - Division
- `(% 17 5)` - Modulo

### test_comparisons.txt
All six comparison operators:
- `(> 10 5)` - Greater than
- `(< 5 10)` - Less than
- `(== 5 5)` - Equal
- `(!= 5 6)` - Not equal
- `(>= 10 5)` - Greater or equal
- `(<= 5 10)` - Less or equal

### test_logic.txt
All logical operators:
- `(& a b)` - AND
- `(| a b)` - OR
- `(^ a b)` - XOR
- `(! a)` - NOT
- `(!& a b)` - NAND
- `(!| a b)` - NOR

### test_variables.txt
Variable management:
- `x = 42;` - Assignment
- `x;` - Retrieval
- `my_var = 10;` - Underscores allowed

### test_expressions.txt
Complex expressions:
- `result = (+ 5 3);` - Assign result
- `(+ a b)` - Use variables in expressions
- `(+ (- 10 5) (* 2 3))` - Nested expressions

### test_vectors.txt
Vector/list operations:
- `v = (1 2 3,,);` - Create vector
- `v :len;` - Get length
- `v :push 4;` - Push element
- `v :get 0;` - Get element at index
- `v :pop;` - Pop element

### test_edge_cases.txt
Boundary conditions:
- Negative numbers: `-42`, `-999`
- Zero handling: `0`
- Large numbers: `999999`
- Operations that might overflow

### test_advanced.txt
Complex real-world scenarios:
- Multiple variables working together
- Variable swapping: `x y = y x`
- Computation chains: `result1 = (+ a b); result2 = (- c result1)`
- Nested operations

### test_integration.txt
Full language feature integration:
- Combines all test categories
- Realistic code examples
- Multi-step computations
- Variable dependencies

## Output

When you run tests, the program will output the results of evaluating each statement.

Example output from `test_arithmetic.txt`:
```
8      # (+ 5 3)
80     # (+ 10 20 30) - but first two are added
20     # (- 10 3)
... etc
```

## Common Commands

```bash
# Run single test
cargo run test_variables.txt

# Run all tests manually
for f in examples/test_*.txt; do cargo run $(basename $f); done

# Run with test script
./run_tests.sh

# See what's in a test
cat examples/test_arithmetic.txt

# Count lines in all tests
wc -l examples/test_*.txt
```

## Tips for Writing New Tests

1. Use clear comments to describe what's being tested
2. Each test should be self-contained
3. Group related tests together
4. Include edge cases in appropriate files
5. Follow the naming convention: `test_<topic>.txt`

## Troubleshooting

**Error: "cannot find file"**
- Make sure you're in the `tinywrite` directory
- File should be in `examples/` subdirectory

**Error: "unexpected token"**
- Check syntax in the test file
- Ensure all parentheses are balanced
- Verify operator names are correct

**No output**
- Some test files might not produce visible output
- The program evaluates expressions but may not print them
- Check the source code to see what should be printed

## Next Steps

- Review `examples/README.md` for language syntax reference
- Look at existing examples: `source.txt`, `var_swap.txt`, `push.txt`
- Modify tests to experiment with the language
- Create your own test files following the same patterns

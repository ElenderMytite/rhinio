# TinyWrite Test Suite

`examples` directory contains comprehensive test files for the TinyWrite language interpreter. Each test file focuses on a specific aspect of the language.

## Test Files

### 2. **test_arithmetic.txt**
Tests all arithmetic operations.
- Addition with multiple operands
- Subtraction
- Multiplication
- Division
- Modulo operator

**Run:** `cargo run test_arithmetic.txt`

### 3. **test_comparisons.txt**
Tests all comparison operators.
- Greater than (`>`)
- Less than (`<`)
- Equal (`==`)
- Not equal (`!=`)
- Greater or equal (`>=`)
- Less or equal (`<=`)

**Run:** `cargo run test_comparisons.txt`

### 4. **test_logic.txt**
Tests all logical operators.
- AND (`&`)
- OR (`|`)
- XOR (`^`)
- NOT (`!`)
- NAND (`!&`)
- NOR (`!|`)

**Run:** `cargo run test_logic.txt`

### 5. **test_variables.txt**
Tests variable assignment and retrieval.
- Simple variable assignment
- Variable naming conventions (underscores allowed)
- Variable retrieval

**Run:** `cargo run test_variables.txt`

### 6. **test_expressions.txt**
Tests complex expressions with variables and operations.
- Assigning expression results to variables
- Using variables in expressions
- Nested expressions
- Expression evaluation

**Run:** `cargo run test_expressions.txt`

### 7. **test_vectors.txt**
Tests vector/list operations.
- Vector creation
- Vector length (`:len`)
- Pushing elements (`:push`)
- Getting elements (`:get`)
- Popping elements (`:pop`)

**Run:** `cargo run test_vectors.txt`

### 8. **test_edge_cases.txt**
Tests edge cases and boundary conditions.
- Negative numbers
- Zero handling
- Large numbers
- Single character variable names
- Underscores in variable names

**Run:** `cargo run test_edge_cases.txt`

### 9. **test_advanced.txt**
Tests advanced scenarios combining multiple features.
- Multiple variable assignments
- Variable swapping
- Complex multi-step computations
- Nested operations
- Comparisons with variables

**Run:** `cargo run test_advanced.txt`

## Running All Tests

To run all test files:

```bash
cargo run test_lexer_basic.txt
cargo run test_arithmetic.txt
cargo run test_comparisons.txt
cargo run test_logic.txt
cargo run test_variables.txt
cargo run test_expressions.txt
cargo run test_vectors.txt
cargo run test_edge_cases.txt
cargo run test_advanced.txt
```

Or create a test script:

```bash
#!/bin/bash
for test in test_*.txt; do
    echo "Running $test..."
    cargo run "$test"
    echo "---"
done
```

## Language Syntax Reference

### Variables
```python
x = 42;           # Assignment
x;                # Retrieval
```

### Arithmetic
```python
(+ a b c)         # Addition
(- a b)           # Subtraction
(* a b)           # Multiplication
(/ a b)           # Division
(% a b)           # Modulo
```

### Comparisons
```python
(> a b)           # Greater than
(< a b)           # Less than
(== a b)          # Equal
(!= a b)          # Not equal
(>= a b)          # Greater or equal
(<= a b)          # Less or equal
```

### Logical Operations
```python
(& a b)           # AND
(| a b)           # OR
(^ a b)           # XOR
(! a)             # NOT
(!& a b)          # NAND
(!| a b)          # NOR
```

### Vectors
```python
v = (1 2 3,,);    # Create vector
v :len;           # Get length
v :push 4;        # Push element
v :get 0;         # Get element at index
v :pop;           # Pop element
```

### Comments
```python
# This is a comment
x = 5; # inline comment
```

## Notes

- All statements end with a semicolon (`;`)
- Numbers can be negative (e.g. `-42`)
- When assigning negative number to a variable, do it with parentheses (e.g. `z = (-5);`)
- Numbers can contain undrerscores for clarity (e.g. `1_000_000_000`)
- Variable names can contain letters, digits, and underscores (`_`)
- Expressions are enclosed in parentheses: `(operation arg1 arg2 ...)`
- Operations may distinguish arguments passed on the left and on the right: 
  - if more than 2 arguments provided to - and / operations, they will combine (add and multiply respectively) left and right parts and then do the operation
  - if more than 2 arguments provided to comparison operation, compares respectively nth terms on the left and right
- Comments start with `#` and extend to the end of the line
- most outer expression may not be enclosed in parentheses

# Rust Programming Concepts

## 1. Variables and Mutability

- **Immutability by Default:** Variables in Rust are immutable by default, meaning their value cannot be changed after they are first bound.
- **Mutable Variables (`mut`):** The `mut` keyword is used to explicitly declare a variable as mutable, allowing its value to be reassigned.
- **Constants (`const`):** Values that are bound to a name and can never change. They must be type-annotated and can only be set to a constant expression (not the result of a function call or any value computed at runtime).
- **Shadowing:** Declaring a new variable with the same name as a previous one. This is different from making a variable mutable; it lets you reuse a name while changing the variable's type, or to maintain immutability for the original binding.

## 2. Data Types

Rust has two main categories of data types:

| Type Category   | Specific Types Discussed                                      | Description                                                                                   |
|-----------------|--------------------------------------------------------------|-----------------------------------------------------------------------------------------------|
| Scalar Types    | Integers (`i32`, `u64`), Floating-Point Numbers (`f32`, `f64`), Booleans (`bool`), Characters (`char`) | Represent a single value. Integers can be signed (`i`) or unsigned (`u`) and come in various bit sizes. The default integer type is `i32`, and the default float is `f64`. |
| Compound Types  | Tuples and Arrays                                            | Represent a group of values.                                                                  |

- **Tuples:** A fixed-size grouping of values that can have different types (e.g., `("channel", 100_000)`). Values can be accessed via destructuring or dot notation (e.g., `my_tuple.0`).
- **Arrays:** A fixed-length collection of values where every element must have the same type. Access is via bracket syntax (e.g., `my_array[0]`).

## 3. Functions

- **Definition (`fn`):** Functions are declared using the `fn` keyword.
- **Parameters:** Functions can accept parameters, but their types must be explicitly annotated (e.g., `fn add(x: i32, y: i32)`).
- **Statements vs. Expressions:**
    - *Statements* perform an action but do not return a value (e.g., `let x = 5;` or `println!()`).
    - *Expressions* evaluate to a resulting value (e.g., `5 + 6` or calling a function).
- **Return Values:** The return type is indicated with `-> Type`. The last expression in the function body is implicitly returned (without a semicolon), but you can also use the `return` keyword explicitly.

## 4. Control Flow

- **If Expressions:** Conditional logic is handled with `if`, `else if`, and `else` blocks. The condition must explicitly be a `bool` (`true` or `false`). An `if`/`else` block can also be used as an expression to assign a value to a variable.
- **Loops:** Rust has three kinds of loops:
    - **`loop`:** An infinite loop that continues until a `break` keyword is encountered. It can also return a value using `break value;`.
    - **`while`:** Executes as long as a specified condition remains true.
    - **`for`:** Used primarily to iterate over elements in a collection (like an array) or a range of numbers.

## 5. Comments

- **Single-Line Comments:** Uses two forward slashes (`//`).
- **Block Comments:** Uses a forward slash and an asterisk, and is closed with an asterisk and a forward slash (`/* ... */`).

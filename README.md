
### **Data Types in Rust**

Rust is a statically-typed language, meaning data types must be known at compile time. Rust data types are categorized into **Scalar** and **Compound** types.

---

#### **1. Scalar Types**
Represent a single value:
- **Integer Types**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (signed); `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (unsigned)
- **Floating-Point Types**: `f32`, `f64`
- **Boolean Type**: `bool` (`true`, `false`)
- **Character Type**: `char` (4 bytes, supports Unicode)

**Example**:
```rust
let x: i32 = 42; // Integer
let pi: f64 = 3.14; // Floating-point
let is_rust_fun: bool = true; // Boolean
let heart: char = '❤️'; // Character
```

---

#### **2. Compound Types**
Combine multiple values:
- **Tuples**: Fixed-size collections of various types.
  ```rust
  let tuple: (i32, f64, char) = (10, 3.14, 'x');
  ```
- **Arrays**: Fixed-size collections of the same type.
  ```rust
  let arr: [i32; 3] = [1, 2, 3];
  ```

---

### **Mutable and Immutable**
- **Immutable (default)**: Variables are immutable by default, meaning their values cannot be changed after binding.
  ```rust
  let x = 10;
  // x = 20; // This will throw an error
  ```
- **Mutable**: Declared with `mut`, allowing the value to change.
  ```rust
  let mut x = 10;
  x = 20; // This works
  ```

---

### **Bindings (Variables)**
- **Variable bindings** associate a value with a name.
- Variables are immutable by default, ensuring safety and predictability.
- **Shadowing**: You can redeclare a variable with the same name, even changing its type.
  ```rust
  let x = 5;       // Immutable binding
  let x = x + 1;   // Shadowing allows re-binding
  ```

---

### **Key Points**
1. **Data Types**: Scalar (integer, float, bool, char) and Compound (tuple, array).
2. **Mutable vs Immutable**:
   - Immutable is the default; use `mut` for mutable variables.
   - Mutable variables allow changing the value but not the type.
3. **Binding/Variables**:
   - Immutable bindings promote safety.
   - Shadowing allows redefining a variable for transformations or type changes.


---

### **Vector (`Vec<T>`) vs Array (`[T; N]`)**

#### **Vector (`Vec<T>`)**
- A **vector** is a growable, heap-allocated collection that can store multiple values of the same type.
- Useful when the number of elements is unknown at compile time or needs to change dynamically.

**Example**:
```rust
let mut vec = Vec::new(); // Create an empty vector
vec.push(1);              // Add elements
vec.push(2);
vec.push(3);
println!("{:?}", vec);    // Output: [1, 2, 3]
```

#### **Array**
- An **array** is a fixed-size collection stored on the stack that holds multiple values of the same type.
- The size of the array must be known at compile time.

**Example**:
```rust
let arr: [i32; 3] = [1, 2, 3]; // Fixed-size array
println!("{:?}", arr);         // Output: [1, 2, 3]
```

#### **Differences Between Vector and Array**

| Feature         | **Vector (`Vec<T>`)**                        | **Array (`[T; N]`)**            |
| --------------- | -------------------------------------------- | ------------------------------- |
| **Size**        | Dynamic (growable or shrinkable at runtime)  | Fixed at compile time           |
| **Storage**     | Heap-allocated                               | Stack-allocated                 |
| **Usage**       | For collections with unknown or varying size | For collections with fixed size |
| **Performance** | Slightly slower due to heap allocation       | Faster due to stack allocation  |
| **Syntax**      | `Vec<T>` (e.g., `Vec<i32>`)                  | `[T; N]` (e.g., `[i32; 3]`)     |
| **Mutability**  | Can dynamically add or remove elements       | Fixed; cannot resize            |

---

### **Key Notes**
- Use **vectors** when:
  - You don't know the size of the collection in advance.
  - You need to grow or shrink the collection dynamically.
- Use **arrays** when:
  - The size is constant and known at compile time.
  - You prioritize performance due to stack allocation.



---

### **Macros in Rust**

A **macro** in Rust is a way to write code that writes other code (metaprogramming). Macros are particularly useful for reducing boilerplate and generating code at compile time.

#### **Types of Macros in Rust**
1. **Declarative Macros** (`macro_rules!`)
   - Used for pattern matching and code generation.
   - Examples: `println!`, `vec!`.

   **Example**:
   ```rust
   macro_rules! say_hello {
       () => {
           println!("Hello, Rust!");
       };
   }

   fn main() {
       say_hello!(); // Expands to: println!("Hello, Rust!");
   }
   ```

2. **Procedural Macros**
   - More flexible and powerful than `macro_rules!`.
   - Three types:
     - **Custom Derive**: Adds functionality to structs or enums using attributes like `#[derive(CustomTrait)]`.
     - **Attribute-like Macros**: Similar to annotations, used with `#[...]`.
     - **Function-like Macros**: Invoked like functions.

   **Example of Custom Derive**:
   ```rust
   #[derive(Debug)]
   struct User {
       name: String,
       age: u32,
   }

   fn main() {
       let user = User {
           name: String::from("Sharath"),
           age: 25,
       };
       println!("{:?}", user); // Uses the derive macro for Debug.
   }
   ```

---

### **What Does `!` Mean in Rust?**
The `!` in Rust indicates that you are invoking a macro instead of a regular function.

#### **Examples:**
- **`println!` Macro**:
  ```rust
  fn main() {
      println!("Hello, {}!", "Rust"); // Invokes the println! macro
  }
  ```
- **`vec!` Macro**:
  ```rust
  fn main() {
      let numbers = vec![1, 2, 3]; // Creates a vector using the macro
  }
  ```

---

### **Why Use Macros Instead of Functions?**
1. **Code Generation**: Macros can generate code at compile time.
2. **Variable Argument Lists**: Macros handle a flexible number of arguments (e.g., `println!`).
3. **Optimizations**: Since macros operate at compile time, they can be optimized by the compiler.

---

### **Key Notes**
- Macros allow writing concise and reusable code by generating boilerplate or complex patterns at compile time.
- The `!` is a syntactic indicator that a macro is being called, not a function.
- Use macros for tasks like:
  - Debugging (`println!`, `dbg!`)
  - Creating collections (`vec!`)
  - Customizing behavior with procedural macros (e.g., `#[derive(Debug)]`).


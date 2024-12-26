
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


---

### **`#[derive(Debug)]` in Rust**

`#[derive(Debug)]` is an **attribute** in Rust used to automatically implement the `Debug` trait for a struct or enum. This trait enables you to print the values of a struct or enum using the `{:?}` formatter in macros like `println!`.

#### **Why Use `#[derive(Debug)]`?**
- Rust does not automatically provide a way to print the contents of custom types.
- Adding the `Debug` trait allows you to inspect the values of structs and enums during development or debugging.

#### **Example Usage**

**Without `#[derive(Debug)]`:**
```rust
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Sharath"),
        age: 25,
    };
    // println!("{:?}", user); // This will cause a compile-time error
}
```
- The code above will fail because `Debug` is not implemented for `User`.

**With `#[derive(Debug)]`:**
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
    println!("{:?}", user); // Output: User { name: "Sharath", age: 25 }
}
```

**Pretty Printing with `:#?`:**
To display the output in a more readable, pretty-printed format:
```rust
fn main() {
    let user = User {
        name: String::from("Sharath"),
        age: 25,
    };
    println!("{:#?}", user);
}
```
**Output**:
```
User {
    name: "Sharath",
    age: 25,
}
```

#### **How `#[derive(Debug)]` Works**
- `#[derive(Debug)]` generates an implementation of the `Debug` trait for your type.
- The `Debug` trait is defined in Rust's standard library and provides the `fmt` method to format the type.

**Generated Implementation (Simplified):**
```rust
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ name: {:?}, age: {:?} }}", self.name, self.age)
    }
}
```

#### **Key Notes**
1. **Attribute**: `#[derive(Debug)]` is a Rust attribute for automatic trait derivation.
2. **Purpose**: Makes it easy to inspect custom types during debugging.
3. **Usage**:
   - Use `{:?}` for simple debug output.
   - Use `{:#?}` for pretty-printed debug output.
4. **Customization**: If needed, you can manually implement the `Debug` trait for more control over the formatting.


---
### **`format!` Macro**
The `format!` macro in Rust creates formatted strings without printing them directly. It works similarly to `println!`, but instead of printing, it returns the formatted string.

#### **Usage**
```rust
fn main() {
    let name = "Sharath";
    let age = 25;
    let message = format!("Hello, {}! You are {} years old.", name, age);
    println!("{}", message); // Output: Hello, Sharath! You are 25 years old.
}
```

#### **Key Points**
- **Returns a String**: Unlike `println!` which writes to the console, `format!` returns a `String`.
- **Formatting**: It supports the same formatting as `println!`, including placeholders like `{}` and formatting options like `{:?}`.

---

### **Mutable and Immutable Bindings in Rust**

#### **Bindings in Rust**
A **binding** is an association of a name (variable) to a value. In Rust:
- Variables are **immutable by default**, meaning their value cannot be changed after they are bound.
- To make a variable mutable, you must explicitly use the `mut` keyword.

#### **Immutable Bindings**
By default, variables cannot be reassigned:
```rust
fn main() {
    let x = 10;
    // x = 20; // Compile-time error: cannot assign twice to immutable variable
    println!("{}", x); // Output: 10
}
```

#### **Mutable Bindings**
Using the `mut` keyword, you can make a variable mutable:
```rust
fn main() {
    let mut x = 10;
    x = 20; // Allowed because `x` is mutable
    println!("{}", x); // Output: 20
}
```

---

### **Key Notes**

1. **Immutable by Default**:
   - Ensures safety by preventing unintended changes to variables.
   - Encourages writing predictable and thread-safe code.

2. **`mut` Keyword**:
   - Explicitly allows a variable to be modified.
   - Example:
     ```rust
     let mut count = 0;
     count += 1; // Allowed because of `mut`
     ```

---

### **Using `{:#?}` for Debugging**

The `{:#?}` syntax is used with `println!` to pretty-print values, making them easier to read.

#### **Example**
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
    println!("{:#?}", user);
}
```

**Output:**
```
User {
    name: "Sharath",
    age: 25,
}
```

---

### **Key Notes Summary**
- **`format!`**: Returns a formatted string for reuse.
- **Bindings**: Immutable by default; use `mut` for mutability.
- **`mut` Keyword**: Required for variables that will change.
- **`{:#?}`**: Pretty-prints structures for better readability.

---

### **Inherent Implementation in Rust (`impl`)**

In Rust, you can define **inherent implementations** for structs or enums using the `impl` block. These implementations allow you to associate methods and functions directly with a type.

---

### **Key Concepts**

1. **`impl` Block**:
   - The `impl` block is where you define methods (functions associated with a type) and functions for a struct or enum.
   - These methods can access fields of the struct or enum.

   **Example**:
   ```rust
   struct User {
       name: String,
       age: u32,
   }

   impl User {
       fn greet(&self) {
           println!("Hello, {}!", self.name);
       }
   }

   fn main() {
       let user = User {
           name: String::from("Sharath"),
           age: 25,
       };
       user.greet(); // Output: Hello, Sharath!
   }
   ```

2. **`Self`**:
   - Represents the type for which the `impl` block is written.
   - It can refer to the type (`Self`) or an instance (`self`).
     - **`Self`**: Refers to the type.
     - **`self`**: Refers to the current instance.

   **Example**:
   ```rust
   impl User {
       fn new(name: String, age: u32) -> Self {
           Self { name, age } // Creates a new instance
       }
   }

   fn main() {
       let user = User::new(String::from("Sharath"), 25);
       println!("{} is {} years old.", user.name, user.age);
   }
   ```

3. **`new` Method**:
   - It is a convention in Rust to define a method named `new` in the `impl` block as a constructor for your type.
   - It simplifies creating instances.

   **Example**:
   ```rust
   struct Point {
       x: i32,
       y: i32,
   }

   impl Point {
       fn new(x: i32, y: i32) -> Self {
           Self { x, y }
       }
   }

   fn main() {
       let point = Point::new(5, 10);
       println!("Point({}, {})", point.x, point.y);
   }
   ```

4. **Associated Functions vs Methods**:
   - **Associated Functions**: Do not take `self` as the first parameter and are called using `Type::function()`.
   - **Methods**: Take `self`, `&self`, or `&mut self` as the first parameter and are called using `instance.method()`.

   **Example**:
   ```rust
   struct Calculator;

   impl Calculator {
       fn add(a: i32, b: i32) -> i32 {
           a + b // Associated function
       }

       fn subtract(&self, a: i32, b: i32) -> i32 {
           a - b // Method (requires an instance)
       }
   }

   fn main() {
       println!("{}", Calculator::add(5, 3)); // Output: 8

       let calc = Calculator;
       println!("{}", calc.subtract(5, 3)); // Output: 2
   }
   ```

---

### **Key Notes Summary**
1. **`impl`**: Used to define methods and functions associated with a type.
2. **`Self`**:
   - `Self`: Refers to the type within the `impl` block.
   - `self`: Refers to the current instance of the type.
3. **`new`**:
   - A conventionally named constructor function for creating instances of a type.
4. **Associated Functions vs Methods**:
   - Associated functions don’t require an instance (`Self::function()`).
   - Methods require an instance (`self.method()`).

---
In Rust, **double quotes (`"`)** and **single quotes (`'`)** have distinct meanings and uses:

---

### **Double Quotes (`"`)**
Double quotes are used to define **string literals** (`&str` type). A string literal is a slice of UTF-8 text data that is immutable and stored in the program's binary.

#### **Examples**:
```rust
fn main() {
    let greeting = "Hello, World!"; // &str type
    println!("{}", greeting); // Output: Hello, World!
}
```

#### **Key Points**:
- Strings in double quotes (`"`) are of type `&str`, which is a reference to a string slice.
- `&str` is immutable and stored in memory as a sequence of UTF-8 characters.
- To create a mutable, heap-allocated string, you can use the `String` type:
  ```rust
  let mut dynamic_string = String::from("Hello");
  dynamic_string.push_str(", World!");
  println!("{}", dynamic_string); // Output: Hello, World!
  ```

---

### **Single Quotes (`'`)**
Single quotes are used to define **character literals** (`char` type). A `char` represents a single Unicode scalar value, which could be a letter, number, symbol, or even an emoji.

#### **Examples**:
```rust
fn main() {
    let letter = 'A'; // char type
    let number = '1'; // char type
    let emoji = '😊'; // char type
    println!("{} {} {}", letter, number, emoji); // Output: A 1 😊
}
```

#### **Key Points**:
- A `char` is a 4-byte Unicode scalar value.
- Each `char` represents a single character, unlike strings which can contain multiple characters.
- A `char` is enclosed in single quotes (`'`), and it cannot represent more than one character.

---

### **Key Differences Between Double and Single Quotes**
| **Aspect**   | **Double Quotes (`"`)**          | **Single Quotes (`'`)**        |
| ------------ | -------------------------------- | ------------------------------ |
| **Type**     | `&str` (string slice)            | `char` (single Unicode scalar) |
| **Purpose**  | Represents strings (text data).  | Represents single characters.  |
| **Examples** | `"Hello, Rust!"`                 | `'H'`, `'😊'`, `'1'`            |
| **Length**   | Can contain multiple characters. | Always exactly one character.  |

---

### **Practical Usage Differences**
1. **For Text**:
   - Use double quotes (`"`) for strings.
   - Use single quotes (`'`) for single characters.

2. **Concatenation**:
   - Strings (`&str` or `String`) can be concatenated using `.push_str()` or the `+` operator.
   - Characters (`char`) can only be concatenated after conversion to a string.

   **Example**:
   ```rust
   fn main() {
       let mut text = String::from("Hello");
       text.push(' '); // Adding a char
       text.push_str("Rust!"); // Adding a string
       println!("{}", text); // Output: Hello Rust!
   }
   ```

3. **Iteration**:
   - Strings can be iterated over to yield individual `char` values.
   ```rust
   let greeting = "Hi!";
   for c in greeting.chars() {
       println!("{}", c); // Prints H, i, !
   }
   ```

Let me know if you'd like further clarification!
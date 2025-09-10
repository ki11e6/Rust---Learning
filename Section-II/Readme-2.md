# **Ownership, Borrowing, and Lifetime in Rust**

Rust’s memory safety model is built on three core concepts: **ownership**, **borrowing**, and **lifetime**. These systems ensure that Rust programs are both safe and efficient without requiring a garbage collector.

---

## **1. Ownership**

Ownership is the foundation of Rust's memory management system. Every value in Rust has an **owner**, which is the variable that holds the value. There are three key rules:

1. Each value in Rust has a single owner.
2. When the owner goes out of scope, the value is dropped, and its memory is freed.
3. Ownership can be **transferred** through assignment or function calls (this is called a *move*).

## Example

```rust
fn main() {
    let s1 = String::from("Hello"); // `s1` owns the String
    let s2 = s1; // Ownership is moved to `s2`
    // println!("{}", s1); // Error! `s1` is no longer valid
}
```

When `s1` is moved to `s2`, `s1` becomes invalid to avoid multiple ownerships.

---

### **2. Borrowing**

Borrowing allows you to use a value without taking ownership. This is done through **references**. Borrowing ensures memory safety by enforcing rules:

1. At any given time, you can have **either**:
   - One mutable reference.
   - Any number of immutable references.

2. References must always be valid.

#### Immutable Borrowing

You can borrow a value immutably as many times as you like.

Example:

```rust
fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s); // Borrowing `s` immutably
    println!("The length of '{}' is {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

#### Mutable Borrowing

You can borrow a value mutably but only once at a time.

Example:

```rust
fn main() {
    let mut s = String::from("Hello");
    change(&mut s); // Mutable borrow
    println!("{}", s); // "Hello, world!"
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
```

---

### **3. Lifetime**

Lifetimes ensure that references are valid for as long as they are used. They prevent **dangling references** (references to invalid memory). Rust uses a system of **lifetime annotations** to track how long references are valid.

#### Key Points about Lifetimes

1. Lifetimes are determined automatically most of the time using **lifetime elision rules**.
2. In complex scenarios, you might need to specify lifetimes explicitly with annotations like `'a`.

#### Example of Dangling Reference Prevention

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x; // Error: `x` does not live long enough
    }
    // println!("{}", r); // `r` would point to invalid memory
}
```

#### Explicit Lifetime Annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long");
    let s2 = String::from("short");
    let result = longest(&s1, &s2);
    println!("The longest string is {}", result);
}
```

Here, `'a` specifies that the lifetime of the returned reference is tied to the shorter of the two input references.

---

### **Summary: How They Work Together**

1. **Ownership** ensures that only one variable owns a value at any time, and when the owner goes out of scope, the value is dropped.
2. **Borrowing** allows temporary access to a value without transferring ownership, using either immutable or mutable references.
3. **Lifetimes** ensure that references remain valid for as long as they are used, preventing dangling references.

These systems work together to make Rust's memory management safe and efficient, eliminating common issues like null pointers, use-after-free bugs, and data races.

## 🧩 Ownership, Borrowing & Lifetimes – Introduction

### ⚡ Why Ownership, Borrowing, and Lifetimes?

- Core systems in Rust that ensure **memory safety without garbage collection**.
- They decide **how data is referenced, shared, and mutated**.
- Hard at first but mastering them = conquering 90% of Rust’s learning curve.
- These rules **influence program design** → requires a shift in thinking compared to languages like JavaScript, Python, or C++.

---

### 🧩 Ownership (Intro)

**Working Definition:**

> The ownership system in Rust **limits the ways data can be referenced and mutated**, preventing bugs and improving reasoning about programs.

---

### 🚨 Example Bug in JavaScript

```javascript
const engine = { working: true };

const mustang = { name: "Mustang", engine };
const camaro = { name: "Camaro", engine };

function checkCar(car) {
  if (car.name === "Mustang") {
    car.engine.working = false;
  }
}

checkCar(mustang);
console.log(mustang.engine.working); // false
console.log(camaro.engine.working);  // also false 😱
```

- **Expected**: Only Mustang’s engine should change.
- **Actual**: Both Mustang and Camaro engines change → because they share the same `engine` reference in memory.

---

### 🧠 The Core Problem

- Many languages allow **shared references** by default.
- Mutation through one reference causes **side effects** in other places.
- Leads to **subtle, hard-to-debug bugs**.

---

### ✅ Rust’s Solution

- Rust enforces strict **ownership rules**:

  1. Each value has **exactly one owner**.
  2. Ownership decides who is responsible for cleanup (dropping the value).
  3. Ownership rules **prevent aliasing with mutation** unless explicitly handled (via borrowing or smart pointers).
- If we try to write the Mustang & Camaro example in Rust, the compiler **won’t allow it to compile**.

---

### 🔑 Key Takeaways

1. **Ownership = Safety Net**

   - Forces explicit decisions about who owns and mutates data.
   - Prevents hidden side effects.

2. **Limits for Safety**

   - Ownership restricts usage but guarantees memory safety.

3. **Big Picture**

   - Ownership + Borrowing + Lifetimes ensure:

     - No *use-after-free*.
     - No *dangling pointers*.
     - No *data races*.

## 📝 Ownership Rules (1 & 2)

> Remember: **Rust wants to avoid unexpected updates to values.**

### 🔹 Rule 1: Every value is owned by a single variable (binding)

- When you create a value, it gets an **owner**.
- Example:

```rust
struct Bank {
    name: String,
}

fn main() {
    let bank = Bank { name: String::from("ICICI") };
    // `bank` owns the Bank instance
}
```

Here, `bank` is the **owner** of the `Bank` struct value in memory.

---

### 🔹 Rule 2: Assignment = Move (not copy by default)

- When you assign a value to another variable, **ownership is moved**.
- The old variable becomes invalid → you cannot use it anymore.

```rust
struct Bank {
    name: String,
}

fn main() {
    let bank = Bank { name: String::from("ICICI") };
    let other_bank = bank; // ownership moved from `bank` to `other_bank`

    // println!("{:?}", bank.name); ❌ ERROR: bank no longer owns the value
    println!("{}", other_bank.name); // ✅ works
}
```

- After the move, `bank` has **no value** bound to it.
- Trying to use `bank` will cause a **compile-time error**.

---

### 📊 Visualization

1. Create `bank` → owns `Bank { name: "ICICI" }`.
2. Assign to `other_bank` → value is **moved**.
3. `bank` → invalid (no longer owns the value).
4. Only `other_bank` can be used.

---

### ⚠️ Important Notes (from Rust Docs)

- Some types (like integers, booleans, floats) implement the **Copy trait** → they are *copied*, not moved.
- Heap-allocated types (like `String`, `Vec`, custom structs) are **moved by default**.

Example with integers (Copy type):

```rust
fn main() {
    let x = 5;
    let y = x; // copies value, not moves
    println!("x = {}, y = {}", x, y); // ✅ both valid
}
```

---

✅ So far we’ve got:

- **Rule 1**: Every value has exactly one owner.
- **Rule 2**: Assignment moves ownership; old variable becomes invalid (unless type is `Copy`).

---

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

## 🔗 Borrowing – Introduction

### ⚡ Motivation

- With **ownership rules**, passing a value to a function **moves** it, leaving the original variable invalid.
- But often we want to **share access** to a value without transferring ownership.
- This is where **borrowing** comes in.

---

### 🔹 Borrowing = Using References (`&`)

- Instead of moving ownership, we can pass a **reference** (`&T`) to a value.
- This lets functions **read** the value without taking ownership.

Example:

```rust
#[derive(Debug)]
struct Account {
    balance: i32,
}

fn print_account(acc: &Account) {
    println!("{:?}", acc);
}

fn main() {
    let account = Account { balance: 100 };

    print_account(&account); // borrow, don't move
    print_account(&account); // can use again!

    println!("{:?}", account); // ✅ still valid
}
```

- `&account` creates a **reference** (a pointer to the value, without taking ownership).
- The function receives the reference and can **read** from it.
- The original `account` still owns the value and remains valid.

---

### 📊 Visualization

1. `account` → owns the `Account` struct.
2. `&account` → creates a reference that **points to** the same value.
3. Reference is passed into `print_account`.
4. After the function call, ownership is still with `account`.

---

### ✅ Key Takeaways on Borrowing

1. Borrowing uses **references** (`&T`) instead of moving ownership.
2. Multiple references can exist, as long as they are **immutable** (read-only).
3. Borrowing lets you **reuse values across functions** without losing ownership.
4. Borrowing = *the preferred way of sharing values safely* in Rust.

---

### ⚠️ Notes from Rust Docs

- Borrowing is the **default mechanism** to allow safe, temporary access to data.
- References follow **borrowing rules**:

  - At any time, you can have either:

    - Multiple **immutable** (`&T`) references, OR
    - One **mutable** (`&mut T`) reference.
- This prevents **data races** and **unexpected updates**.

---
Here are the **notes** for this transcript, expanded with extra detail from the official Rust documentation:

---

## Borrow System Basics

- **Borrowing** lets you access a value without transferring ownership.
- A **reference** (`&T`) is a pointer to a value that allows you to use it without moving it.
- Use borrowing when:

  - Passing a value to multiple functions.
  - Storing references to values inside other structs.
  - Avoiding repeated ownership transfers.

---

## Ampersand (`&`) Operator

- Usage depends on **context**:

  1. **Next to a type** → `&Type`

     - Declares a function parameter or variable as a **reference to that type**.
     - Example:

       ```rust
       fn print_account(acc: &Account) { ... }
       ```

  2. **Next to a value** → `&value`

     - Creates a reference (borrow) to an existing owned value.

       ```rust
       let acc_ref = &account;
       ```

- Precision: technically, `&value` is borrowing from the **binding/owner** of the value, not the raw value itself.

---

## Immutable (Read-Only) References

- **Default kind of reference** created by `&`.

- Allow you to *read* but not *modify* the underlying value.

- Example:

  ```rust
  let acc_ref = &account;
  println!("{}", acc_ref.balance); // ✅ OK
  acc_ref.balance = 10;             // ❌ Error
  ```

- **Error reason**:

  > “cannot assign to `...` because it is borrowed as immutable”.

- ✅ You can create **multiple immutable references** at the same time.

  ```rust
  let r1 = &account;
  let r2 = &account;
  println!("{}", r1.holder);
  println!("{}", r2.balance);
  ```

- 🔑 Why?
  Multiple immutable borrows prevent **unexpected data modification bugs** (e.g., two things trying to change the same value without coordination).

---

## Mutable References (will come later)

- Declared with `&mut`.
- You can only have **one mutable reference** to a value at a time (or no immutable references simultaneously).
- Enforces **safe mutation without race conditions**.

---

## Borrowing Rule #3 (from transcript)

- **Many immutable (read-only) references** to a value can coexist.
- They can safely look at the value but cannot change it.
- This is Rust’s safety guarantee to avoid *hidden side effects*.

---

## Borrowing Rule #4 (from transcript)

- **If a value has active references, you cannot move the value.**
- Example:

  ```rust
  let acc_ref1 = &account;
  let acc_ref2 = &account;
  let other_account = account; // ❌ Error: value moved while borrowed
  ```

- Why?

  - Moving would invalidate the references (`acc_ref1`, `acc_ref2`).
  - Rust prevents dangling references at **compile time**.

---

## Big Picture

- Borrowing rules enforce **memory safety without a garbage collector**.
- Core principle:

  > *“When in doubt, Rust prevents unexpected mutations and invalid references.”*

---

## Mutable References in Rust

- **Immutable references (`&T`)** → can read but not modify.
- To **modify without moving ownership**, Rust provides **mutable references (`&mut T`)**.

---

## Tedious Approach Without Mutable References

- One way to modify a value is:

  - Move ownership into a function.
  - Update it inside the function.
  - Return the value.
  - Reassign it to the original binding.
- Example:

  ```rust
  fn change_account(mut acc: Account) -> Account {
      acc.balance = 10;
      acc
  }

  let mut account = Account::new();
  account = change_account(account);
  ```

- ✅ Works,
- ❌ Tedious: must return & reassign every time.

---

## Mutable References (`&mut`)

- Cleaner approach: borrow the value mutably instead of moving it.
- Syntax:

  - Function argument type: `&mut T`
  - Passing reference: `&mut value`
- Example:

  ```rust
  fn change_account(acc: &mut Account) {
      acc.balance = 10; // direct modification
  }

  fn main() {
      let mut account = Account::new();
      change_account(&mut account);
      println!("{}", account.balance); // prints updated balance
  }
  ```

### Key Points

- `&mut` lets you **read and write** through the reference.
- You must declare the original binding as `mut` if you want to pass a mutable reference.

---

## Rules for Mutable References

### Rule #5 — Aliasing XOR Mutability

- At any time:

  - EITHER: many immutable (`&T`) references
  - OR: exactly one mutable (`&mut T`) reference
- ❌ You cannot mix immutable and mutable references simultaneously.
- ❌ You cannot have more than one mutable reference at once.
- Example:

  ```rust
  let mut account = Account::new();
  let r1 = &account;      // immutable borrow
  let r2 = &mut account;  // ❌ Error: cannot borrow as mutable while immutable borrow exists
  ```

👉 This prevents **data races** and ensures deterministic behavior, even in single-threaded code.

---

### Rule #6 — No Owner Modification While Borrowed

- If references exist (immutable or mutable), you **cannot modify through the owner** at the same time.
- Example:

  ```rust
  let mut account = Account::new();
  let r = &mut account;
  account.balance = 20;  // ❌ Error: cannot use `account` while mutable borrow exists
  ```

- Why?
  To prevent **unexpected updates** from the perspective of the borrower:

  - A reference assumes the value won’t change “behind its back” except through itself.

---

## When to Use Mutable References

- Commonly used in **helper functions** that mutate values.
- Cleaner than ownership transfer + return pattern.
- Allows safe, direct updates without redundant moves.

---

## Big Picture

- Mutable references are powerful but tightly controlled by Rust’s borrow checker.
- Rules guarantee:

  - No **aliasing + mutation** (no two things writing/reading inconsistently).
  - No **dangling references**.
  - No **unexpected state changes**.

---

📌 Summary:

- `&T` → many, read-only.
- `&mut T` → one, read/write.
- You cannot mix them simultaneously.
- Owners cannot mutate while references exist.

---

## Borrow System — Rule #7

### Some types are **copied** instead of **moved**

---

## Key Idea

- Normally, assigning or passing a value **moves ownership**.
- But for some types, ownership is **not transferred**. Instead, the value is **copied**.
- These types implement the **`Copy` trait**.

---

## Copy vs. Move

### Move (default behavior)

- For most heap-allocated or complex types (`String`, `Vec<T>`, structs with non-Copy fields):

  - Assignments **transfer ownership**.
  - The old variable is invalidated.

  ```rust
  let s1 = String::from("hello");
  let s2 = s1;        // move
  println!("{}", s1); // ❌ error: s1 no longer valid
  ```

### Copy (special case)

- For simple scalar values (numbers, booleans, chars, etc.):

  - Assignments create a **bitwise copy** of the value.
  - Both variables remain valid.

  ```rust
  let x = 5;
  let y = x;          // copy
  println!("x = {}, y = {}", x, y); // ✅ both usable
  ```

---

## Copy Types

- **Primitive scalar types**:

  - `i32`, `u32`, `f64`, etc.
  - `bool`
  - `char`
- **Tuples** *if all elements are Copy*
  e.g. `(i32, i32)` is Copy, but `(i32, String)` is not.
- **Arrays** *if elements are Copy*
  e.g. `[i32; 3]` is Copy, but `[String; 3]` is not.

⚠️ **Vectors (`Vec<T>`) are not Copy** (they own heap memory).

---

## Why Rule #7 Feels Surprising

- For most types, Rust enforces strict ownership/move rules.
- But with Copy types, the move is replaced with a **cheap copy**.
- At first glance, this seems like ownership rules are being broken:

  ```rust
  let num = 5;
  let other_num = num;   // looks like a move, but actually a copy
  println!("{}", num);   // ✅ still valid
  ```

---

## Behind the Scenes

- When assigning `num` to `other_num`:

  - Instead of invalidating `num`, Rust just duplicates the bits.
  - Both `num` and `other_num` independently hold the value `5`.
- This is why Copy values feel like values in most other programming languages.

---

## `Copy` Trait

- Types that behave this way implement the `Copy` trait.
- Rules:

  - A type can be `Copy` only if all its components are also `Copy`.
  - If a type implements `Drop` (custom destructor), it **cannot** be `Copy`.
    (e.g. `String`, `Vec<T>` are not Copy because they manage heap memory.)
- Example:

  ```rust
  fn takes_copy(x: i32) { println!("{}", x); }

  let a = 10;
  takes_copy(a);   // a is copied, not moved
  println!("{}", a); // ✅ still valid
  ```

---

## Intuition

- **Copy types = cheap to duplicate (stack-allocated, fixed size).**
- **Move types = potentially expensive/dangerous to duplicate (heap-allocated, dynamic).**
- Rust enforces different rules to keep memory safe and efficient.

---

📌 **Summary**

- Rule #7: Some values don’t move, they copy.
- Examples: numbers, booleans, chars, arrays/tuples of Copy types.
- This can feel like ownership rules are being broken — but it’s just a special case for efficiency.
- Powered by the `Copy` trait.

---

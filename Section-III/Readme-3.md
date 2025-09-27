# Structs vs Enums

## ❓ The Big Question

When modeling data in Rust → **Should I use a `struct` or an `enum`?**

Rust provides both, but choosing the right one makes APIs cleaner and reduces unnecessary boilerplate.

---

## 🔑 Rule of Thumb

* **Use an `enum`** when:

  * All variants (things you’re modeling) have the **same set of methods**.
  * Methods have the same **name**, **signature**, and **return type**.
  * You want to model “one of several possible types.”
  * Example: `Book`, `Movie`, and `Audiobook` all share the method `description()`.

* **Use `struct`s** when:

  * Variants share **some methods**, but also have **different methods** that don’t make sense across all types.
  * Example:

    * `Book` → `read()`
    * `Movie` → `play()`
    * `Audiobook` → `listen()`
  * In this case, forcing them into a single enum would give methods that don’t make sense (`Movie` shouldn’t have `read()`).

---

## 📚 Example: Enum Use Case

All items have the same method → ✅ enum is a good fit.

```rust
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String, narrator: String },
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } =>
                format!("Book: {title} by {author}"),
            Media::Movie { title, director } =>
                format!("Movie: {title}, directed by {director}"),
            Media::Audiobook { title, narrator } =>
                format!("Audiobook: {title}, narrated by {narrator}"),
        }
    }
}
```

* All variants support `.description()`.
* Cleaner to handle as a single enum.

---

## 📚 Example: Struct Use Case

Different items need unique methods → ✅ structs are better.

```rust
struct Book {
    title: String,
    author: String,
}
impl Book {
    fn read(&self) {
        println!("Reading {} by {}", self.title, self.author);
    }
}

struct Movie {
    title: String,
    director: String,
}
impl Movie {
    fn play(&self) {
        println!("Playing movie: {} directed by {}", self.title, self.director);
    }
}

struct Audiobook {
    title: String,
    narrator: String,
}
impl Audiobook {
    fn listen(&self) {
        println!("Listening to {} narrated by {}", self.title, self.narrator);
    }
}
```

* Each type has methods unique to its domain.

---

## ⚖️ Practical Considerations

* **Pattern matching** on enums can become verbose:

  * Each `match` arm must list all properties of that variant.
  * If a type has many fields (e.g., `Book` with ISBN, ID, length, etc.), matches get tedious.
* **Structs** allow methods directly tied to data fields without requiring large `match` expressions.
* **Enums** are powerful when modeling finite states (e.g., `Result`, `Option`, `TrafficLight`).

---

## 📌 Key Takeaways

* Use **enums** when all variants behave the same way (same methods).
* Use **structs** when variants differ in behavior or responsibilities.
* Consider **code ergonomics**:

  * If you find yourself writing giant `match` arms with many fields → prefer structs.
* Both can be combined:

  * Enums can hold structs as variants (common pattern in Rust).

---

## 🔑 What is an Enum?

* An **enum** (short for *enumeration*) defines a type by enumerating its possible **variants**.
* Each variant can:

  1. Contain **no data** (unit-like).
  2. Contain **named fields** (struct-like).
  3. Contain **unnamed fields** (tuple-like).

---

## 🛠️ Example: Media Enum

```rust
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),       // tuple variant → episode number
    Placeholder,        // unit variant → no data
}
```

### Variants here

1. `Book`, `Movie`, `Audiobook` → with **named fields**.
2. `Podcast(u32)` → **unnamed tuple field** (episode number).
3. `Placeholder` → **no fields** (unit-like).

---

## 🖊️ Syntax & Creation

```rust
let b = Media::Book { title: "Rust 101".to_string(), author: "Jane Doe".to_string() };
let m = Media::Movie { title: "Ferris the Crab".to_string(), director: "Crabby Joe".to_string() };
let a = Media::Audiobook { title: "Async Adventures".to_string() };
let p = Media::Podcast(10);      // episode number = 10
let ph = Media::Placeholder;     // no data
```

---

## 🔄 Pattern Matching with Enums

* In Rust, `match` must be **exhaustive**: all variants must be handled.

```rust
impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } =>
                format!("Book: {} by {}", title, author),

            Media::Movie { title, director } =>
                format!("Movie: {} directed by {}", title, director),

            Media::Audiobook { title } =>
                format!("Audiobook: {}", title),

            Media::Podcast(episode) =>
                format!("Podcast, episode {}", episode),

            Media::Placeholder =>
                "Placeholder".to_string(),
        }
    }
}
```

---

## 🧠 Key Concepts

### 1. **Unit-like Variants**

* Variants with no fields.
* Example: `Media::Placeholder`
* Useful when the *presence* of the variant conveys enough meaning.

### 2. **Tuple-like Variants**

* Variants with unnamed fields.
* Example: `Media::Podcast(u32)`
* Behaves like a tuple; field names are not explicit.
* ⚠️ Less clear than named fields → should be used when meaning is obvious.

### 3. **Struct-like Variants**

* Variants with named fields.
* Example: `Media::Book { title, author }`
* Provides clarity and self-documenting code.

---

## 📚 Rust Docs Expansion

From the [Rust Book](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html):

* Enums allow **encoding of states and variants in a type-safe way**.
* Commonly used with `match` or `if let`.
* Often paired with `Option<T>` and `Result<T, E>` — two standard library enums.

Example from stdlib:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

---

## ✅ Takeaways

* Enums represent **a type with multiple possible forms**.
* Variants can carry **different kinds of data**.
* `match` ensures **exhaustive handling** of all cases.
* Use **unit-like variants** for simple markers.
* Use **tuple variants** for simple, single-value data (but be cautious about clarity).
* Use **struct-like variants** for descriptive, named fields.

---

## Option Enum and Pattern Matching

## Accessing Vector Elements with `.get()`

* In Rust, you can access individual elements in a `Vec` using the `.get(index)` method.
* Example:

  ```rust
  catalog.items.get(0); // tries to get the first element
  ```

* `.get()` does **not** return the element directly—it returns an `Option<T>`.

---

## What is `Option<T>`?

* `Option` is a **built-in enum** in Rust that represents the possibility of having a value or not.
* Definition (from Rust standard library):

  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```

* **Variants**:

  * `Some(value)` → contains a value of type `T`.
  * `None` → represents no value (instead of `null` in other languages).
* Rust has no `null`, `nil`, or `undefined`. `Option` is the safe alternative.

---

## Example with `.get()`

```rust
match catalog.items.get(0) {
    Some(value) => println!("Item: {:?}", value),
    None => println!("Nothing at that index"),
}
```

* `catalog.items.get(0)` returns `Some(&item)` if the element exists.
* If the index is out of range, it returns `None`.

---

## Why `Option` is Important

* Forces you to **handle both cases**:

  * Value exists (`Some`)
  * Value missing (`None`)
* Prevents runtime errors like *Null Pointer Exceptions* common in other languages.
* Encourages safer, more explicit handling of optional values.

---

## Pattern Matching with `Option`

* `match` is often used:

  ```rust
  match catalog.items.get(100) {
      Some(item) => println!("Found: {:?}", item),
      None => println!("No item at this index"),
  }
  ```

* `if let` can simplify when only one case matters:

  ```rust
  if let Some(item) = catalog.items.get(0) {
      println!("Found: {:?}", item);
  }
  ```

---

## Key Takeaways

* Rust’s `Option` replaces the concept of `null`.
* `Some(value)` and `None` are everywhere in Rust code.
* `.get()` on vectors returns an `Option`, not the item directly.
* Pattern matching (`match` or `if let`) is required to safely extract values.
* This enforces **safety at compile-time** and avoids runtime crashes.

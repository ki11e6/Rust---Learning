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

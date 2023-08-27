# Ownership

## Ownership
**Rules**

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped. (call function drop)

```rust
let x=5;
let y=x;

let s1=String::from("hehe");
let s2=s1;
```
x,y are all valid because they are stack-only type, so they are easy to copy.(called `Copy` trait)
But String type is a pointer type created on heap, they can just be taken ownership like `std::move` in cpp.
s1 is now no longer valid.


But you can use clone to fix it.(like a deep copy).
```rust
let s2=s1.clone;
```

## Reference
**Rules**
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
You can add a `&` to do a reference.
```rust
let s2=&s1;
or
let s3=&mut s1;
```
There's no reason to add a mut bdfore s3, cuz it is its own a mut reference.

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
You can't return a reference of it cuz reference may go out of scope.

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
But you can return the String itself cuz it's on the heap.

## Slice 
Slice type is `&str`

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```
the `clear()` gets a mutable reference while the println use word the immutable reference, they appear the same time.

string literals is itself a slice type.
so we prefer functions with a parameter `&str`.

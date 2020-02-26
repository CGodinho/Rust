# Ownership

## Stack / Heap

Stack uses **LIFO**, with push and pop operations. Only accepts fixed size data.

Heap **allocates** a requested amount of data and returns a pointer. 

Accessing data in the heap is slower and allocation also takes time.
Function call uses the heap.

**Ownership** hides the above details.


## Rules

 * Each value in Rust has an owner;
 * There can only be one owner at a time;
 * When the owner goes out of scope, the value will be dropped.


## Examples

### Scope definition

```rust,no_run
{
    let mut s = String::from("hello");

    // push_str() appends a literal to a String
    s.push_str(", world!");

    println!("{}", s)

}
```

scope ends and s is **free automatically**

### Copy references

```rust,no_run
{
    // No problem basic type copy is executed
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    // Wait! s2 gets ownership of s1, s1 no loger use
    let s2 = s1;
    
    // Error!
    println!("{}, world!", s1);
    // OK!
    println!("{}, world!", s2);
}
```

s1 was moved into s2. s1 is no longer **valid** so on problems with later scope clear.
So it works as a kind of **shallow copy**.

### Deep copy

```rust,no_run
{
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

By calling `clone`, the string gets copied and now there are 2 distinct references.


## Types with copy trait

If a type implements the **Copy Trait**, an automatic deep copy is executed. it includes:

 * Integers;
 * The Boolean type (bool);
 * Floating point types;
 * The character type (char);
 * Tuples, if they only contain types that are also Copy.


## Functions

Types without the copy trait that are sent directly to functions as arguments are also drop.

So use **references** and **borrow** a value:

```rust,no_run
fn calculate_length(s: &String) -> usize {
    // Here, s goes out of scope. But because it does not have ownership, nothing happens
    s.len()
}
```
References allow to refer to some value **without** taking its ownership.

References as function parameters are called **borrowing**.

**NOTE:** The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

## Returns values transfer ownership

Good for building constructors.

```rust,no_run
fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello");

    // some_string is returned and moves out to the calling function
    some_string
}
```

## Mutable references

Values may be changed in a function with mutable references.

But they are restricted to a single reference for piece scope.


```rust,no_run
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```


## The Rules of References

* At any given time, there can be only one mutable reference or any number of immutable references;

* References must always be valid.


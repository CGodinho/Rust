# Ownership

## Stack / Heap

Stack uses **LIFO**, with push and pop operations. Only accepts fixed suze data.

Heap **allocates** a requested amount of data and returns a pointer. 

Accessing data in the heap is slower and allocation also takes time.
Function call uses the heap.

**Ownership** hides the above details.

## Rules

 * Each value in Rust has a variable that’s called its owner;
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
    
    // scope ends and s is free automatically
}
```

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

s1 was moved into s2.  S1 is no longer **valid** so on problems with later scope clear.
So it works as a kind of **shallow copy**.

### Deep copy

```rust,no_run
{
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

By calling `clone`, the string gets copiesd and onow there are 2 distinct references.


## Types with copy trait

 * Integers;
 * The Boolean type (bool);
 * Floating point types;
 * The character type (char);
 * Tuples, if they only contain types that are also Copy.


## Functions

Types without the copy trait that are sent directly to functions as arguments are also drop.
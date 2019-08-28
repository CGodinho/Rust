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



## cc


`let x = 5;`

`x = 6;` <span style="color:red"> -> ERROR</span>

For mutable, use reserve word **mut**:

`let mut x = 5;`

`let mut x = 6;`


## Reassign

Using let it is possible to redefine a immutable variable:

`let spaces = "   ";`

`let spaces = spaces.len();`

## Constants

`const MAX_POINTS: u32 = 100_000;`

**NOTE:** Use uppercase for constants.
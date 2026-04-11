# Language Structure
**ALL OF THESE WILL LIKELY CHANGE**
### Random Ramblings:
- C/Java based Syntax primarily 
- Borrows some basic syntax from Rust
- Very basic
- Little to no synatic Sugar



### Basic Examples:
**ALL OF THESE WILL LIKELY CHANGE** <br>
Function:
```Rust
// Comments are "//" based
fn int example(){
    // Creates a mutable variable with "var"
    // Declares type with "int"
    // All lines must end with ";"
    var mutableVariable: int = 2;
    // Creates an immutable variable with "val"
    val staticVariable: int = 10;

    // Reasignments are done like most languages
    mutableVariable = staticVariable;

    // Uncommenting the next line will cause a compiler error
    // staticVariable = mutableVariable;

    // Return keyword required
    return mutableVariable;
}
```
Struct: <br>
**NOTE:** The `value` and `reference` key words will likely change, and be merged with struct <br>
**TODO:** Uh, Rethink structs and how they are handled on the HL
```Rust
// "value" declares a pass-by-value Object Type
struct value Example1 {
    fieldI: int; // Will be 0-ed out
    fieldB: byte;
    fieldDefault: float = 5.0; // Can declare defaulted fields
}

// "reference" declares a pass-by-reference Object Type
struct reference Example2 {
    fieldI: int; // un-declared fields will be 0-ed out
    fieldB: byte;
    fieldDefault: float = 5.0; // Can declare defaulted fields
}

struct reference NestedStruct {
    subStruct: Example1;
}

fn int exampleStructs(){
    // Can do the last value, but not required
    var example1WithManualInit: example1 = example1 {fieldI: 10, fieldB: 2}
    // "Automatically" inits the value based on the struct, 0s everything but the declared values
    var example1WithAutoInit: example1 = example1 {}
}
```
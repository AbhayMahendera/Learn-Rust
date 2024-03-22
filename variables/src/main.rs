fn main(){
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
    
    let y = 6;
    println!("the value of x is {y}");
    let y = 7;
    println!("the value of x is {y}");

}


// you need to add mut to let the compiler know that this value is mutable.
// it also helps the readers to know that you will be changing the value somewhere in the future.


// variable vs constants
// constants are alway immutable
// can only be set to a constant expression and not the result of a value that could only be computed at runtime.


// naming conventions - constants follow snake case but all of them to be upper case.

// re-declaring a variable with the keyword 'let' makes the second variable overshadow the first variable.
// the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.


// mut keyword dont let us change the type. to explain :

/*
let mut x = 6;
x = "six";

This will give an error.
*/

// shadowing helps us do this 

/*
    let spaces = "   ";
    let spaces = spaces.len();
*/




//             <--------- DATA TYPES --------->

// Rust is statically typed. It must know types of all variables at compile time.


/*


- Integers :

Length  |   Signed  |   Unsigned
--------|-----------|------------
8-Bit   |   i8      |   u8
16      |   i16     |   u16
32      |   i32     |   u32
64      |   i64     |   u64
128     |   i128    |   u128
arch    |   isize   |   usize

Signed numbers are stored using two's complement representation.

Each signed variant can store numbers from :
    -(2^(n-1)) to (2^(n-1)) -1

    n is the number of bits that variant uses.

ex : i8 can store numbers from -2^7 to 2^7 -1 => -128 to 127

Each Unsigned Variant can store values from 0 to 2^(n-1) -1


Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.


Integer Literals in Rust : 
*/
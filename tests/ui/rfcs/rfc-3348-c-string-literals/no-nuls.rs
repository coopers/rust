#![feature(c_str_literals)]

fn main() {
    c"\0";
    //~^ ERROR: null character in C string

    c"\u{00}";
    //~^ ERROR: null character in C string

    c" ";
    //~^ ERROR: null character in C string

    c"\x00";
    //~^ ERROR: null character in C string

    cr" ";
    //~^ ERROR: null character in C string
}

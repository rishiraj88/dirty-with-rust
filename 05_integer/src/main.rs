fn main() {
    // unsigned 8-bit integer type
    let mut x: u8 = 255; // max accommodable in 8-bit type

    println!("x (old): {}", x);

    // check for overflow condition:
    // if the value is promoted to 256,
    // OR the value is set to -256, i.e., the min for 8-bit type
    // OR some error/exception/panic is encountered 
    x = x + 1;

    println!("x (new): {}", x);
}
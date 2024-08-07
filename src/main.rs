fn main() {
    // this codebase contains lesson learnt while studying data in depth with rust
    // in the rust in action textbook

    let a = 50115u16;
    let b = -15421i16;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let a = 42.42f32;

    // the keyword unsafe does not inherently mean dangerous, 
    // but it is a way to let the compiler know that the programmer is taking charge of the situation there
    let frakentype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{:032b}", frakentype);

}

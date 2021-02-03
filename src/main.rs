mod credential;

extern crate optee_teec;

static mut INDY_POOL_CREDENTIALS: &'mut str = "";

fn main() {
    println!("Hello, world!");
}

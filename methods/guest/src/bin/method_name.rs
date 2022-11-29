// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
#![no_std]
use risc0_zkvm::guest::env;
risc0_zkvm::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    let n: u32 = env::read();

    env::commit(&n);
}

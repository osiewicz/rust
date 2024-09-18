//@ run-rustfix
#![feature(never_patterns)]
#![feature(exhaustive_patterns)]
#![allow(incomplete_features)]
#![deny(unreachable_patterns)]

enum Void {}

#[rustfmt::skip]
fn main() {
    let res: Result<(), Void> = Ok(());
    match res {
        Ok(_) => {}
        Err(_) => {} //~ ERROR unreachable
        Err(_) => {}, //~ ERROR unreachable
    }

    match res {
        Ok(_x) => {}
        Err(!), //~ ERROR unreachable
        Err(!) //~ ERROR unreachable
    }
}

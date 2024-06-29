//@ edition: 2021
//@ build-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

#![feature(async_closure)]

fn constrain<T: async FnOnce()>(t: T) -> T {
    t
}

fn call_once<T>(f: impl FnOnce() -> T) -> T {
    f()
}

async fn async_call_once<T>(f: impl async FnOnce() -> T) -> T {
    f().await
}

fn main() {
    let c = constrain(async || {});
    call_once(c);

    let c = constrain(async || {});
    async_call_once(c);
}

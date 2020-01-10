static mut LEVELS: u32 = 0;

// This violates the idea of no shared state, and this doesn't internally
// protect against races, so this function is `unsafe`
unsafe fn bump_levels_unsafe1() -> u32 {
    let ret = LEVELS;
    LEVELS += 1;
    return ret;
}

// Assuming that we have an atomic_add function which returns the old value,
// this function is "safe" but the meaning of the return value may not be what
// callers expect, so it's still marked as `unsafe`
//unsafe fn bump_levels_unsafe2() -> u32 {
//    return atomic_add(&mut LEVELS, 1);
//}

pub fn run() {
    unsafe {
        println!("LEVELS before bump_levels_unsafe1 = {}", LEVELS);
        bump_levels_unsafe1();
        println!("LEVELS after bump_levels_unsafe1 = {}", LEVELS);
    }
}
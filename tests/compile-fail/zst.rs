fn main() {
    let x = &() as *const () as *const i32;
    let _ = unsafe { *x }; //~ ERROR access memory with alignment 1, but alignment 4 is required
}

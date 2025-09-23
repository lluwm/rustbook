fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);

    // This line would cause a compile-time error due to mutable borrow while immutable borrow is active.
    // v points to an array with 3 elements, but after push it may need to reallocate to fit 4 elements.
    // The resize will deallocate the previous array, and allocate a new, bigger array. In the process,
    // num is left dangling, pointing to deallocated memory.

    // Basic principles:
    // data should never be mutated while it is being accessed.

    println!("Third element is {}", *num);
}
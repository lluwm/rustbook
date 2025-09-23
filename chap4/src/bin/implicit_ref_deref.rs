fn main() {
    let x: Box<i32> = Box::new (-1);
    let x_abs1 = i32::abs(*x);  // explicit dereference
    let x_abs2 = x.abs();    // implicit dereference
    println!("x_abs1 = {}, x_abs2 = {}", x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();            // implicit dereference (twice)
    println!("r_abs1 = {}, r_abs2 = {}", r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = String::len(&s);  // explicit reference
    let s_len2 = s.len();               // implicit reference
    println!("s_len1 = {}, s_len2 = {}", s_len1, s_len2);

    // Note: this implicit reference/dereference sounds so scary to me.
}
fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, use safe methods to modify vector elements.
    v[0] = 10; //Directly access and modify elements
    println!("{:?}", v);
}
pub fn basic_loop() -> () {
    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            break;
        }
        println!("x = {}", x);
    }
}

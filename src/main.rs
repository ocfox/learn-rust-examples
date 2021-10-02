use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut t: i32 = 0;
    for i in 100..200 {
        for tt in 2..i {
            if i % tt == 0 {
                t = tt;
                break;
            }
        }
        if t == i {
            print!("{} ", t);
        }
    }

    let end = start.elapsed();
    println!("use ss {:?}", end);
}

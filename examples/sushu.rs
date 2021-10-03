use std::time::Instant;

fn main() {
    let mut t: i32 = 0;
    let start = Instant::now();

    for i in 100..201 {
        for tt in 2..=i {
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
    println!("{:?}", end);
}

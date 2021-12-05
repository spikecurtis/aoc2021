use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut old0: i64 = 0;
    let mut old1: i64 = 0;
    let mut old_total: i64 = 0;
    let mut increases = 0;
    let mut i = 0;
    loop {
        buffer.clear();
        let r = io::stdin().read_line(&mut buffer)?;
        if r == 0 {
            break;
        }
        buffer.pop();
        let cur = buffer.parse::<i64>().unwrap();
        let sum = cur + old0 + old1;
        println!("cur: {}, sum: {}", cur, sum);
        if sum > old_total && i > 2 {
            println!{"inc."};
            increases = increases + 1;
        }
        old_total = sum;
        old0 = old1;
        old1 = cur;
        i = i + 1;
    }
    println!("{}", increases);
    Ok(())
}

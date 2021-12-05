use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    loop {
        buffer.clear();
        let r = io::stdin().read_line(&mut buffer)?;
        if r == 0 {
            break;
        }
        buffer.pop();
        let v: Vec<&str> = buffer.split(' ').collect();
        let cmd = v[0];
        let amount = v[1].parse::<i64>().unwrap();
        if cmd == "forward" {
            horizontal += amount;
            depth += amount * aim;
        }
        if cmd == "down" {
            aim += amount;
        }
        if cmd == "up" {
            aim -= amount;
        }
    }
    println!("horizontal: {}, depth: {}, product: {}", horizontal, depth, horizontal*depth);
    Ok(())
}

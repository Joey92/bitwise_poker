use std::io;

pub fn user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn read_user_input() -> usize {
    loop {
        let choice = user_input().expect("Failed to read choice");
        return match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
}

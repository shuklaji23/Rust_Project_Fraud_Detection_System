use std::io;

pub fn start_console(){
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        let command = &words[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&words[2..]),
            "rm" => {
                let ind: usize = words[2].parse().unwrap();
                todo.remove(ind);
            }
            "stop" => break,
            "help" => {
                todo.help();
                break;
            }
            "reset" => todo.reset(),
            "sort" => todo.sort_items(),
            _ => {
                println!("wrong input")
            }
        }
    }
}
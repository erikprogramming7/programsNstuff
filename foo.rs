use std::io;

fn main() {
    let mut act: bool = true;
    println!("interface thing");
    println!("help for commands (so far)");

    while act {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("error");
        let inp = inp.trim();

        match inp {
            "help" => {
                println!("end");
                println!("help");
                println!("hi");
                println!("bye");
            }
            "end" => {
                println!("terminated");
                act = false;
                println!("exit code {}", act as u8);
            }
            "hi" => println!("hello"),
            "bye" => println!("bye"),
            _ => println!("huh"),

        }
    }
}

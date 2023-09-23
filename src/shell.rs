use std::io::Write;
use std::process::Command;
use std::str::from_utf8;


pub struct RShell {
    current_dir: String
}

impl RShell {
    pub fn new() -> Self {

        let current_dir_from_pwd = Command::new("pwd").output().unwrap();
        Self {
            current_dir: from_utf8(&current_dir_from_pwd.stdout).unwrap().to_string().trim().to_string(),
        }
    }

    fn read_input(&self) -> Result<String, ()> {
        let mut buffer = String::new();
        let stdin = std::io::stdin();

        let read_bytes = stdin.read_line(&mut buffer).unwrap();

        if read_bytes > 0 {
            Ok(buffer)
        } else {
            Err(())
        }
    }

    fn prompt(&self) -> String {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let input = self.read_input().unwrap().trim().to_string();
        return input;
    }

    fn parse_and_execute(&mut self, input: String) -> String {
        let mut tokens: Vec<&str> = input.split(' ').collect();

        let cmd_str = tokens[0];

        let args = &mut tokens[1..];

        for arg in args.iter_mut() {
            *arg = arg.trim();
        }

        match cmd_str {
            "cd" => {
                self.current_dir = format!("{}/{}", self.current_dir, args[0]);
                return format!("Moved to: {}", self.current_dir);
            },
            "pwd" => {
                return self.current_dir.to_owned();
            },
            "exit" => {
                println!("Exiting, Have a nice day!");
                std::process::exit(0);
            },
            x => {
                let result = Command::new(x).args(args).output().unwrap();
                return String::from_utf8_lossy(&result.stdout).to_string();
            }
        };
    }

    pub fn run(&mut self) {
        loop {
            let input = self.prompt();
            let out = self.parse_and_execute(input);

            println!("{}", out);
        }
    }
}

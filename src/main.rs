extern crate rand;

use std::time::Duration;
use rand::Rng;
use std::thread;
use std::io::{self, Write};

#[derive(Debug)]
struct Configuration {
    servers: Vec<String>,
    server: String,
}

#[derive(Debug)]
struct App {
    config: Configuration,
}

impl App {
    fn new (config: Configuration) -> App {
        App { config: config }
    }

    fn help (&self) {
        println!("Available commands:");
        println!("list            - Lists servers");
        println!("switch <server> - Switches to server");
    }

    fn debug (&self) {
        println!("{:?}", self);
    }

    fn list (&self) {
        for s in self.config.servers.iter() {
            println!("{}", s);
        }
    }

    fn switch (&mut self, server: String) {
        for s in self.config.servers.iter() {
            if server.trim() == s {
                println!("You have selected: {}", s);
                self.config.server = s.clone();
            }
        }
    }

    fn a (&self) {
        let servers = self.config.servers.clone(); //Arc::new(self.config.servers.clone());
        let handles: Vec<_> = servers.into_iter().map(|server| {
            thread::spawn(move || {
                let ping = rand::thread_rng().gen_range(50, 1001);
                thread::sleep(Duration::from_millis(ping));
                println!("{}: {}ms", server, ping);
            })
        }).collect();

        for h in handles {
            h.join().unwrap();
        }
    }

    fn route (&mut self, input: &String) {
        let input = input.trim();
        let first_space = input.find(' ').unwrap_or(input.len());
        let (action, value) = input.split_at(first_space);

        match action {
            "help" => {
                self.help();
            },
            "debug" => {
                self.debug();
            },
            "list" => {
                self.list();
            },
            "switch" => {
                self.switch(value.to_string());
            },
            "a" => {
                self.a();
            },
            _ => {}
        }
    }
}

fn main() {
    let config: Configuration = Configuration {
        servers: vec!["hello".to_string(), "world".to_string()],
        server: String::new(),
    };

    let mut app: App = App::new(config);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Invalid input");

        app.route(&input);
    }
}

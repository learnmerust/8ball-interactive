extern crate rand;
extern crate regex;
extern crate shrust;
extern crate ansi_term;
use shrust::{Shell, ShellIO};
use ansi_term::Colour::{Green, Yellow, Red};
use ansi_term::{ANSIString};
use ansi_term::Style;
use regex::Regex;
use rand::Rng;
use std::io::prelude::*;

fn get_response() -> String {
    let responses: [ANSIString; 20] = [
      Green.paint("It is certain"),
      Green.paint("It is decidedly so"),
      Green.paint("Without a doubt"),
      Green.paint("Yes — definitely"),
      Green.paint("You may rely on it"),
      Green.paint("As I see it, yes"),
      Green.paint("Most likely"),
      Green.paint("Outlook good"),
      Green.paint("Yes"),
      Green.paint("Signs point to yes"),
      Yellow.paint("Reply hazy, try again"),
      Yellow.paint("Ask again later"),
      Yellow.paint("Better not tell you now"),
      Yellow.paint("Cannot predict now"),
      Yellow.paint("Concentrate and ask again"),
      Red.paint("Don’t count on it"),
      Red.paint("My reply is no"),
      Red.paint("My sources say no"),
      Red.paint("Outlook not so good"),
      Red.paint("Very doubtful")
    ];
    let num: usize = rand::thread_rng().gen_range(0, responses.len());
    responses[num].to_string()
}

fn check_input(text: &str) -> bool {
    let re = Regex::new(r".\?$").unwrap();
    re.is_match(text)
}

fn main() {
    let mut shell = Shell::new(());
    shell.set_default(|io, _, cmd| {
        let question: String = cmd.to_string();
        if check_input(&question) {
            let response: String = get_response();
            try!(writeln!(io, "{}", response));
            Ok(())
        } else {
            let reply = "Please ask a legitimate question!";
            let reply_styled = Style::new().bold().paint(reply).to_string();
            try!(writeln!(io, "{}", reply_styled));
            Ok(())
        }
    });

    println!("{}", "I am the magic 8ball, ask me a question");
    shell.run_loop(&mut ShellIO::default());
}

extern crate clap;
extern crate lettre;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use clap::{App, Arg};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

fn main() -> std::io::Result<()> {
    let mut app = App::new("ugrmail")
        .version("1.0")
        .about("Sends an email using SMTP")
        .arg(
            Arg::with_name("email_receiver")
                .short('r')
                .long("receiver")
                .value_name("EMAIL")
                .help("Sets the email receiver")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("sender")
                .short('s')
                .long("sender")
                .value_name("EMAIL")
                .help("Sets the email sender")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("smtp_host")
                .short('h')
                .long("host")
                .value_name("HOST")
                .help("Sets the SMTP host")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("smtp_username")
                .short('u')
                .long("username")
                .value_name("USERNAME")
                .help("Sets the SMTP username")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("smtp_password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Sets the SMTP password")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("subject")
                .short('j')
                .long("subject")
                .value_name("SUBJECT")
                .help("Sets the email subject")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the file to read the email body from")
                .takes_value(true)
                .required(true),
        );

    let matches = app.clone().get_matches_safe();

    if let Err(_) = matches {
        app.print_help().unwrap();
        println!(); // Print a newline
        return Ok(());
    }

    let matches = matches.unwrap();

    // Extract arguments
    let email_receiver = matches.value_of("email_receiver").unwrap();
    let sender = matches.value_of("sender").unwrap();
    let smtp_host = matches.value_of("smtp_host").unwrap();
    let smtp_username = matches.value_of("smtp_username").unwrap();
    let smtp_password = matches.value_of("smtp_password").unwrap();
    let subject = matches.value_of("subject").unwrap();
    let file_path = matches.value_of("file").unwrap();

    // Read file
    let file = File::open(&Path::new(file_path))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let email = Message::builder()
        .from(sender.parse().unwrap())
        .reply_to(sender.parse().unwrap())
        .to(email_receiver.parse().unwrap())
        .subject(subject)
        .body(contents)
        .unwrap();

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    let mailer = SmtpTransport::relay(smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    let result = mailer.send(&email);

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    Ok(())
}

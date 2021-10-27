#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(array_map)]
#![feature(never_type)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::{
    net::{TcpListener, TcpStream},
    sync::{Condvar, Mutex}
};
use std::io::{BufRead, BufReader, Write};
use SMTA::imap_types::{Any, Command, CommandType, NoAuth, ResponseState, Tag};

use SMTA::threads::{self, spout::Spout};

fn main() {
    // TODO(bryce): More errors to handle
    threads::create_named_thread("Listener".to_string(), listen).unwrap();

    let mux = Mutex::new(0);
    let forever = Condvar::new();

    loop {
        // NOTE(bryce): Add commands or some proper cli here. This will currently wait forever
        forever.wait(mux.lock().unwrap()).unwrap();
    }
}

// NOTE(bryce): Do better error handling. Possibly open listener on main thread
fn listen() -> ! {
    // TODO(bryce): Handle this error
    let listener = TcpListener::bind("127.0.0.1:143").unwrap();
    // NOTE(bryce): This would cause a memory leak if this function ever returned but it shouldn't
    let connection_spout: &'static Spout<_> = Box::leak(Box::new(Spout::new()));

    // TODO(bryce): Parametrize this
    let thread_count = 8;
    for i in 0..thread_count {
        // TODO(bryce): You've heard it before: This error should be handled
        threads::create_named_thread(format!("Handler: {}", i), move || handle_loop(connection_spout)).unwrap();
    }

    loop {
        if let Ok((stream, remote_address)) = listener.accept() {
            println!("Got connection from: {}", remote_address);
            connection_spout.provide(stream);
        }
    }
}

fn handle_loop(spout: &Spout<TcpStream>) -> ! {
    loop {
        let stream = spout.take();
        println!("Took connection");
        handle(stream);
    }
}

#[derive(Debug)]
enum State {
    Greeting,
    NotAuthed,
    Authed,
    Selected,
    Logout,
    Close,
    TLS
}

fn handle(mut client: TcpStream) {
    // TODO(bryce): Errors here...
    let mut reader = BufReader::new(client.try_clone().unwrap());
    let mut state = State::Greeting;
    let debug_ident = format!("[{} - {:?}]", std::thread::current().name().unwrap(), client.peer_addr().unwrap());
    while !matches!(state, State::Close) {
        println!("{} State: {:?}", debug_ident, state);
        match state {
            State::Greeting => {
                // TODO(bryce): This is actually important to handle
                client.write_all(b"* OK [CAPABILITY STARTTLS AUTH=PLAIN LOGINDISABLED IMAP4rev1] IMAP4rev1 server ready\r\n").unwrap();
                state = State::NotAuthed;
            }
            State::NotAuthed => {
                if let Some(command) = receive_command(&mut reader, &debug_ident) {
                    state = process_command(&debug_ident, &mut client, state, command)
                } else {
                    state = State::Logout;
                }
            }
            State::Authed => {state = State::Logout}
            State::Selected => {state = State::Logout}
            State::Logout => {
                client.write_all(b"* BYE\r\n").unwrap();
                state = State::Close;
            }
            State::Close => {unreachable!()}
            State::TLS => {
                unimplemented!();
            }
        }
    }
}

fn process_command(debug_ident: &str, client: &mut TcpStream, state: State, command: Command) -> State {
    match command.tag {
        Tag::Tagged(tag) => {
            match (command.command, state) {
                (CommandType::Any(any), state) => {
                    match any {
                        Any::Capability => {
                            send_data(debug_ident, client, Tag::Untagged,
                                          "CAPABILITY STARTTLS AUTH=PLAIN LOGINDISABLED IMAP4rev1");
                            send_response(debug_ident, client, Tag::Tagged(tag), ResponseState::Ok,
                                          "CAPABILITY Completed");
                            state
                        }
                    }
                }
                (CommandType::NoAuth(no_auth), State::NotAuthed) => {
                    match no_auth {
                        NoAuth::StartTLS => {
                            send_response(debug_ident, client, Tag::Tagged(tag), ResponseState::Ok,
                                          "Begin TLS negotiation now");
                            State::TLS
                        }
                    }
                }
                _ => State::Logout
            }
        }
        _ => State::Logout
    }
}

fn send_data(debug_ident: &str, client: &mut TcpStream, tag: Tag, data: &str) {
    let mut response = format!("{} {}", tag, data);
    println!("{} Server: {}", debug_ident, response);
    response.push_str("\r\n");
    client.write_all(response.as_bytes()).unwrap();
}

fn send_response(debug_ident: &str, client: &mut TcpStream, tag: Tag, response: ResponseState, data: &str) {
    let mut response = format!("{} {} {}", tag, response, data);
    println!("{} Server: {}", debug_ident, response);
    response.push_str("\r\n");
    client.write_all(response.as_bytes()).unwrap();
}

fn receive_command(client: &mut BufReader<TcpStream>, debug_ident: &str) -> Option<Command> {
    let mut command = String::new();
    // TODO(bryce): This is actually important to handle
    client.read_line(&mut command).unwrap();
    if command.ends_with("\r\n") {
        command.truncate(command.len() - 2);
    } else {
        return None;
    }
    println!("{} Client: {}", debug_ident, command);

    parse_command(&command)
}

fn parse_command(command: &str) -> Option<Command> {
    let (tag, command) = command.split_once(' ')?;
    match tag {
        "*" => None,
        "+" => None,
        _ => Some(Command {
            tag: Tag::Tagged(tag.to_string()),
            command: CommandType::parse(command)?
        }),
    }
}

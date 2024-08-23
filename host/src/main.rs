use interprocess::local_socket::{
    traits::ListenerExt, GenericNamespaced, ListenerOptions, Stream, ToNsName,
};
use serde_json::Value;
use std::{
    io::{self, BufRead, BufReader, Write},
    process::Command,
};

fn main() -> std::io::Result<()> {
    // println!("{json_output:?}");

    // Define a function that checks for errors in incoming connections. We'll use this to filter
    // through connections that fail on initialization for one reason or another.
    fn handle_error(conn: io::Result<Stream>) -> Option<Stream> {
        match conn {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Incoming connection failed: {e}");
                None
            }
        }
    }

    let name = "walkthrough.sock"
        .to_ns_name::<GenericNamespaced>()
        .unwrap();

    let opts = ListenerOptions::new().name(name);

    let listener = match opts.create_sync() {
        Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
            eprintln!(
                "Error: could not start server because the socket file is occupied. Please check if
				we is in use by another process and try again."
            );
            return Err(e);
        }
        x => x.unwrap(),
    };

    let mut buffer = String::with_capacity(128);

    for conn in listener.incoming().filter_map(handle_error) {
        // Wrap the connection into a buffered receiver right away
        // so that we could receive a single line from it.
        let mut conn = BufReader::new(conn);
        println!("Incoming connection!");

        // Since our client example sends first, the server should receive a line and only then
        // send a response. Otherwise, because receiving from and sending to a connection cannot
        // be simultaneous without threads or async, we can deadlock the two processes by having
        // both sides wait for the send buffer to be emptied by the other.
        conn.read_line(&mut buffer).unwrap();

        let output: std::process::Output = Command::new("cmd")
            .args([
                "/C",                                  // Launch cmd with command
                "py",                                  // python
                "../providers/WalkthroughProvider.py", // cli file
                "search",                              // search subcommand
                buffer.as_str(),
                // "final fantasy vii",                   // game title
            ])
            .output()
            .unwrap();

        let json_output: Value = serde_json::from_slice(&output.stdout).unwrap();

        // Now that the receive has come through and the client is waiting on the server's send, do
        // it. (`.get_mut()` is to get the sender, `BufReader` doesn't implement a pass-through
        // `Write`.)
        conn.get_mut().write_all(&output.stdout)?;

        // Print out the result, getting the newline for free!
        print!("Client answered: {buffer}");

        // Clear the buffer so that the next iteration will display new data instead of messages
        // stacking on top of one another.
        buffer.clear();
    }

    Ok(())
}

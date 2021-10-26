use crate::runner::Runner;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use nix::sys::socket;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io::{Error,ErrorKind};

pub fn make_tcp_stream(address: &str, runner: &mut Runner) -> std::io::Result<TcpStream> {
    runner.execute("connect to server", || TcpStream::connect(address))
}

pub fn run_echo_server(address: &str) -> std::io::Result<()> {
    let server = TcpListener::bind(address)?;

    // Old ARM compatible version
    // getrandom(0x1f4b8c8, 16, GRND_NONBLOCK) = -1 ENOSYS (Function not implemented)
    // getrandom  was introduced in version 3.17 of the Linux kernel.  Support was added to glibc in version 2.25.
    let fd: RawFd = server.as_raw_fd();
    loop {
        println!("wait connection at {}", address);
        let client = socket::accept(fd).or(Err(Error::new(ErrorKind::Other, "can't accept")))?;
        let mut stream = unsafe { TcpStream::from_raw_fd(client) };

        println!("accept client");

        stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(std::time::Duration::from_secs(5)))?;

        let mut reader = BufReader::new(stream.try_clone().unwrap());

        let mut line = String::new();
        loop {
            line.clear();
            if let Ok(res) = reader.read_line(&mut line) {
                if res > 0 {
                    let _ = stream.write(format!("{}\n", res - 1).as_bytes());
                } else {
                    break;
                }
            } else {
                println!("client disconnected");
                break;
            }
        }
    }
}

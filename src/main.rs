mod runner;
mod tcp;

use crate::runner::Runner;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

const CMD_START: &str = "start";
const CMD_STOP: &str = "stop";

struct TestContext {
    address: String,
    cycles: usize,
    payload_size: usize,
    runner: Runner,
}

impl TestContext {
    fn new() -> TestContext {
        TestContext {
            address: String::new(),
            cycles: 10,
            payload_size: 256,
            runner: Runner::new(),
        }
    }
}

fn make_payload(size: usize) -> String {
    let mut result = String::new();

    (0..size).for_each(|x| {
        result.push(((48u8) + (x % 60) as u8) as char);
    });

    result
}

fn make_error<T>(name: &str) -> std::io::Result<T> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, name))
}

fn run_test(ctx: &mut TestContext) -> std::io::Result<()> {
    let mut client = tcp::make_tcp_stream(&ctx.address, &mut ctx.runner)?;
    let mut reader = BufReader::new(client.try_clone().unwrap());

    client.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    client.set_write_timeout(Some(std::time::Duration::from_secs(5)))?;

    ctx.runner.execute("send START", || {
        client.write(format!("{}\n", CMD_START).as_bytes())?;
        Ok(())
    })?;

    let mut line = String::new();

    ctx.runner.execute("wait ACK", || {
        reader.read_line(&mut line)?;
        line = line.trim().to_owned();
        if line == CMD_START.len().to_string() {
            Ok(())
        } else {
            make_error("")
        }
    })?;

    let payload = make_payload(ctx.payload_size);
    for _ in 0..ctx.cycles {
        ctx.runner.execute("send request", || {
            client.write(format!("{}\n", payload).as_bytes())?;
            Ok(())
        })?;

        ctx.runner.execute("read response", || {
            line.clear();
            reader.read_line(&mut line)?;
            line = line.trim().to_owned();
            if line == payload.len().to_string() {
                Ok(())
            } else {
                make_error("")
            }
        })?;
    }

    ctx.runner.execute("send STOP", || {
        client.write(format!("{}\n", CMD_STOP).as_bytes())?;
        Ok(())
    })?;

    Ok(())
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().unwrap_or("server".to_owned());
    let address = args.next().unwrap_or("0.0.0.0:8888".to_owned());
    let cycles = args.next().unwrap_or("10".to_owned());
    let payload = args.next().unwrap_or("256".to_owned());

    println!("mode:{}", mode);
    println!("address:{}", address);

    if mode == "server" {
        println!();
        tcp::run_echo_server(&address).unwrap();
    } else {
        println!("cycles:{}", cycles);
        println!("payload:{}", payload);
        println!();

        let mut ctx = TestContext::new();
        ctx.address = address;
        ctx.cycles = usize::from_str(&cycles).unwrap();
        ctx.payload_size = usize::from_str(&payload).unwrap();

        let _ = run_test(&mut ctx);
        ctx.runner.report();
    }
}

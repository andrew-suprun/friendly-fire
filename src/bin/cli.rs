#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
      foreign_links {
        Str(::std::str::Utf8Error);
      }
    }
}

pub(crate) use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = Command::new("./target/debug/friendly-fire")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    {
        // limited borrow of stdin
        let stdin = child.stdin.as_mut().expect("failed to get stdin");
        stdin.write_all(b"test").expect("failed to write to stdin");
    }

    let mut o = child
        .wait_with_output()
        .expect("failed to wait on child")
        .stdout;
    o.push(255);

    let out = std::str::from_utf8(&o)
        .chain_err(|| "cannot parse friendly-fire output")?;

    println!("{:?}", out);
    Ok(())
}

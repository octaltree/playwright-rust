use std::{
    collections::VecDeque,
    convert::TryInto,
    io::{BufReader, Read},
    mem,
    process::{Command, Stdio}
};

fn main() {
    let mut child = Command::new("/tmp/playwright-rust-test/driver/playwright.sh")
        .args(&["run-driver"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    // let mut stdout = BufReader::with_capacity(10000, stdout);
    // let mut buf: [u8; 10000] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut buffer = Vec::new();
    let mut length: Option<u32> = None;
    loop {
        // let mut buf: [u8; 4] = unsafe { mem::MaybeUninit::uninit().assume_init() };
        // stdout.read_exact(&mut buf).unwrap();
        // let length = u32::from_le_bytes(buf);
        // println!("{}", length);
        // let mut buf = Vec::with_capacity(length as usize);
        // stdout.read_exact(&mut buf).unwrap();
        // println!("{:?}", &buf);
        println!("poll transaction");
        {
            // let mut buf: [u8; 10000] = unsafe { mem::MaybeUninit::uninit().assume_init() };
            // let mut buf = Vec::with_capacity(10000);
            let mut buf = [0; 1000];
            println!("foo");
            let n = match stdout.read(&mut buf) {
                Ok(n) => n,
                Err(e) => panic!(e)
            };
            println!("bar{}", n);
            buffer.extend(&buf[..n]);
        }
        dbg!("a");
        macro_rules! pending {
            () => {{
                continue;
            }};
        }
        if length.is_none() {
            if buffer.len() >= 4 {
                let off = buffer.split_off(4);
                let bytes: &[u8] = &buffer;
                length = Some(u32::from_le_bytes(bytes.try_into().unwrap()));
                buffer = off;
            } else {
                // TODO: Is it needed waiting for wake if len==0?
                pending!()
            }
        }
        dbg!("b");
        match length.map(|u| u as usize) {
            None => pending!(),
            Some(l) if buffer.len() < l => pending!(),
            Some(l) => {
                let bytes: &[u8] = &buffer[..l];
                println!("RECV>{:?}", unsafe { std::str::from_utf8_unchecked(bytes) });
                // let msg: message::Response = match serde_json::from_slice(bytes) {
                //    Err(e) => {
                //        log::error!("{:?}", e);
                //        return Poll::Ready(None);
                //    }
                //    Ok(r) => r
                //};
                length = None;
                buffer = buffer[l..].to_owned();
            }
        }
    }
}

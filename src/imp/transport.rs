use std::process::{ChildStdin, ChildStdout};

pub(crate) struct Transport {
    stdin: ChildStdin,
    stdout: ChildStdout
}

pub(crate) struct Message {}

impl Transport {
    pub(crate) fn try_new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        Transport { stdin, stdout }
    }

    fn send(msg: Message) {}
}

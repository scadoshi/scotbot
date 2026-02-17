use crate::chat;

pub trait ExitProcess {
    fn exit_process(&self);
}

impl ExitProcess for chat::State {
    fn exit_process(&self) {
        println!("Farewell!");
    }
}

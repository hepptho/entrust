use std::io;
use std::sync::mpsc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ServerEvent {
    Started,
    RequestHandled,
    Stopped,
}

pub(super) trait EventSender {
    fn send_server_event(&self, event: ServerEvent) -> io::Result<()>;
}

impl EventSender for Option<mpsc::Sender<ServerEvent>> {
    fn send_server_event(&self, event: ServerEvent) -> io::Result<()> {
        self.as_ref()
            .map(|s| s.send(event))
            .transpose()
            .map(|_| ())
            .map_err(io::Error::other)
    }
}

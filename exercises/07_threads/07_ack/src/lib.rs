use std::sync::mpsc::{Receiver, Sender};
use crate::{data::{Ticket, TicketDraft}, store::{TicketId, TicketStore}};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { // TODO: Implement Insert{}
        draft: TicketDraft,
        response_sender: Sender<TicketId> // the ID created for the ticket that was just inserted
     },
    Get { // TODO: Implement Get{}
        id: TicketId,
        response_sender: Sender<Option<Ticket>> // Option because the requested Ticket may not exist
     }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {draft, response_sender}) => {
                // TODO: insert the ticket into the TicketStore, then send the ID back to the client
                let id = store.add_ticket(draft);
                let _ = response_sender.send(id);
            }
            Ok(Command::Get {
                id, response_sender
            }) => {
                // TODO: get the requested ticket, then send it back to the client
                let ticket = store.get(id);
                let _ = response_sender.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}

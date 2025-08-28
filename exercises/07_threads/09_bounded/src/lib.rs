// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, sync_channel};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>, // TODO:
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        // TODO:
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(10);
        let command: Command = Command::Insert {
                draft,
                response_channel: response_sender,
            };
        self.sender
            // send to the server the Command::Insert() command. In the command, we have the
            // draft ticket and the channel that we want the server to communicate with us on.
            .send(command)
            .unwrap();
        let ticket_id = response_receiver.recv().unwrap();
        Ok(ticket_id)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        // TODO:
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(10);
        let command: Command = Command::Get {
                id: id,
                response_channel: response_sender,
            }; // construct the command/request to be sent to the server
        self.sender
            .send(command)
            .unwrap();
        Ok(response_receiver.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    // TODO:
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    // TODO:
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>, // TODO:
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>, // TODO:
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);// TODO:
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned()); // TODO:
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}

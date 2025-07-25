use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    // TODO: Implement this struct...
    // well, a ticketstore client should store tickets, so it should have a TicketStore?
    //  no, because we are not implementing ticketstore itself. We are implementing
    //  only the channel portion.
    // it should also have a channel to send events to the server.
    sender: Sender<Command>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        // TODO:

        // I am the client, I want to create a ticket...
        // I will ask the server to create a ticket for me.
        // I will send it a command, requesting to "Insert" a new ticket.
        // In my "Insert" command, i have a draft ticket that will be processed by the server.
        // Once the ticket is successfully inserted into the TicketStore, that ticket will have
        // an associated ticket ID. The server will return that ticket ID to me via the channel
        // that I want it to talk to me on.

        let (response_sender, response_receiver) = std::sync::mpsc::channel();
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
        ticket_id
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        // TODO:

        // I want to get an existing ticket from the server by sending it a request that contains
        //   the ID of the ticket I want to retrieve.
        // I send my "Get" command. In the command's body, I include the ticket ID to be retrieved
        //   and the channel I wish to communicate on.
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        let command: Command = Command::Get {
                id: id,
                response_channel: response_sender,
            }; // construct the command/request to be sent to the server
        self.sender
            .send(command)
            .unwrap();
        response_receiver.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    // TODO: Implement the rest
    // I need to create the client communication channel
    TicketStoreClient { sender }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}

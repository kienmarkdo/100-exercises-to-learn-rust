use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Implement the `IntoIterator` trait for `&TicketStore` so that the test compiles and passes.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

// Answer
/*
We were implementing it on a ref to a ticketstore
We have to use lifetimes
We need to know that the iterator is valid for the right length of lifetime
it needs to be a ref of 'a of ticketstore

let x;
x = 0;

equiv of that is

impl<'a>
'a

declare lifetime with impl 'a, allowing use to assign it at &'a

item needs to be a ref of &'a because we will be returning references
needs to be the same &'a Ticket
Ticket is bound to TicketStore's lifetime. that's why both are &'a

Std slice iter has 'a because lifetime can't be deduced for us, so we are just explicitly mention it
*/
impl <'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = std::slice::Iter<'a, Ticket>; // return a standard slice iterator; this iterator will return references to Ticket
    // this "std::slice::Iter" is giving us back references
    // if you say "type IntoIter = std::slice::Iter<'a, Self::Item>;", it will be a double reference

    fn into_iter(self) -> Self::IntoIter { // into_iter consumes TicketStore
        self.tickets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = (&store).into_iter().collect();
        assert_eq!(tickets, tickets2);
    }
}

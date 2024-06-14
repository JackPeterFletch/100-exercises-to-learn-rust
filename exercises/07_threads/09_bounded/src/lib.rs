// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    capacity: usize,
    server_sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<TicketId>> {
        let (sender, receiver) = sync_channel(self.capacity);
        let command = Command::Insert {
            draft,
            response_channel: sender
        };

        self.server_sender.try_send(command).expect("TODO: panic message");

        Ok(receiver.recv().expect("").expect(""))

    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<TicketId>> {
        let (sender, receiver) = sync_channel(self.capacity);

        let command = Command::Get {
            id,
            response_channel: sender
        };

        self.server_sender.try_send(command).expect("TODO: panic message");

        Ok(receiver.recv().expect("").expect(""))
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        capacity,
        server_sender: sender
    }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<Result<TicketId, TrySendError<TicketId>>>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Result<Option<Ticket>, TrySendError<Option<Ticket>>>>,
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
                let _ = response_channel.send(Ok(id));
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(Ok(ticket.cloned()));
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}

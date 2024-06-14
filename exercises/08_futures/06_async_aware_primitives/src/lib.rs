use tokio::sync::mpsc::{channel, Receiver, Sender};

/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?


pub struct Message {
    payload: String,
    response_channel: Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: Receiver<Message>) {
    loop {

        let result = receiver.recv().await;

        if let Some(msg) = result {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = channel(10);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use std::sync::mpsc;
    use tokio::sync::mpsc::channel;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = channel(10);
        let (response_sender, mut response_receiver) = channel(10);
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}

// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    // TODO:

    // Accept connections on both first and second TcpListener concurrently

    // Send the same reply to ALL clients (not just an echo, but a fixed response)

    // Share a single `reply` value between both server tasks

    // Challenge: Since T doesn't implement Clone, we can't just copy it.
    //  We need to share it between multiple tasks safely.
    // How do we get multiple tasks to share ownership of the same data safely?
    //  We can use `Arc<T>`

    // Wrap the reply in Arc to share it between tasks
    let reply = Arc::new(reply);

    // Spawn two tasks - one for each listener
    let handle1 = tokio::spawn(handle_listener(first, Arc::clone(&reply)));
    let handle2 = tokio::spawn(handle_listener(second, reply));

    // Wait for both tasks to complete
    let (_, _) = tokio::join!(handle1, handle2);
}

async fn handle_listener<T>(listener: TcpListener, reply: Arc<T>)
where
    T: Display + Send + Sync + 'static,
{
    loop {
        // Accept a connection
        let (mut socket, _) = listener.accept().await.unwrap();

        // Send the fixed reply
        let reply_bytes = reply.to_string().into_bytes();
        let _ = socket.write_all(&reply_bytes).await;

        // Connection automatically closes when socket is dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}

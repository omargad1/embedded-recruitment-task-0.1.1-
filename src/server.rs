use crate::message::{client_message, server_message, ClientMessage, ServerMessage, AddResponse};
use log::{error, info, warn};
use prost::Message;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    
};
use tokio::{
    net::{TcpListener, TcpStream}, // Asynchronous TCP networking
    sync::Notify,                 // For signaling shutdowns
    io::{AsyncReadExt, AsyncWriteExt}, // Asynchronous I/O
};

const MAX_MESSAGE_SIZE: usize = 4096; // Define the maximum size for a message


struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream }
    }

 pub async fn handle(&mut self) -> tokio::io::Result<()> {    // make it async function
       
        let mut buffer = vec![0u8; MAX_MESSAGE_SIZE]; //  buffer to handle messages
        
        loop {
            let bytes_read = self.stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                info!("Client disconnected.");
                return Ok(());
            }

          
            info!("Bytes read: {}", bytes_read);
            if bytes_read > MAX_MESSAGE_SIZE {
                error!(
                    "Message size {} exceeds the maximum allowed size {}. Rejecting.",
                    bytes_read, MAX_MESSAGE_SIZE
                );
            } else {
                info!("Processing message of size: {}", bytes_read);
            }


    
            // Decode ClientMessage
            match ClientMessage::decode(&buffer[..bytes_read]) {
                Ok(decoded_message) => match decoded_message.message {
                    Some(client_message::Message::AddRequest(add_request)) => {
                        info!(
                            "Received AddRequest: a={}, b={}",
                            add_request.a, add_request.b
                        );
    
                        // Handle AddRequest
                        let result = add_request.a + add_request.b;
                        let response = ServerMessage {
                            message: Some(server_message::Message::AddResponse(AddResponse { result })),
                        };
    
                        // Encode and send response
                        let payload = response.encode_to_vec();
                        self.stream.write_all(&payload).await?;
                    }
                    Some(client_message::Message::EchoMessage(echo_message)) => {
                        info!("Received EchoMessage: {}", echo_message.content);
    
                        // Echo back the message
                        let response = ServerMessage {
                            message: Some(server_message::Message::EchoMessage(echo_message)),
                        };
    
                        let payload = response.encode_to_vec();
                        self.stream.write_all(&payload).await?;
                    }
                    _ => {
                        error!("Unsupported message type");
                    }
                },
                Err(e) => {
                    error!("Failed to decode message: {}", e);
                }
            }
    
            self.stream.flush().await?;
        }
 }

}





pub struct Server {
    listener: TcpListener,   // A Tokio TcpListener object that listens for incoming client connections asynchronously.

    is_running: Arc<AtomicBool>,   // A shared, thread-safe boolean flag to track the server's running state.

    shutdown_notify: Arc<Notify>,  // A Tokio synchronization primitive that allows signaling multiple tasks to shut down.
}

impl Server {


   // add getter method to get the server address 
   pub fn local_addr(&self) -> tokio::io::Result<std::net::SocketAddr> {
    self.listener.local_addr()
   } 

    /// Creates a new server instance
    pub async fn new(addr: &str) -> tokio::io::Result<Self> {     // Creates a new Server instance, binds it to an address, and initializes its fields.

        let listener = TcpListener::bind(addr).await?;    // Asynchronously binds the server to the given address.
        
        info!("Server running on {}", listener.local_addr()?); // Log the actual port
 
        let is_running = Arc::new(AtomicBool::new(true));    // Initially set to true to indicate the server is active.

        
        let shutdown_notify = Arc::new(Notify::new());   // Used for signaling shutdown events to the server and its tasks
         


        Ok(Server {
            listener,
            is_running,
            shutdown_notify,
        })
    }

    /// Runs the server, listening for incoming connections and handling them
    pub async fn run(&self) -> tokio::io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst); // Set the server as running
        info!("Server is running on {}", self.listener.local_addr()?);

       

        while self.is_running.load(Ordering::SeqCst) {
          tokio::select!{     // Allows waiting on multiple asynchronous operations simultaneously.
            result= self.listener.accept() => {  // Asynchronously accepts new client connections.
              match result{
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr);

                    // Handle the client request
                    let mut client = Client::new(stream);

                   // Spawns a new asynchronous task to handle each client connection
                   tokio::spawn(async move {
                            if let Err(e) = client.handle().await {
                                error!("Error handling client {}: {}", addr, e);
                            }
                    });
                }
                
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
              } 
            } 
            
            // Listens for a shutdown signal and exits the loop when notified.
            _ = self.shutdown_notify.notified() => {
                info!("Shutdown signal received. Stopping server.");
                break;
            }
          }
        }

        info!("Server stopped.");
        Ok(())
    }

    /// Stops the server by setting the `is_running` flag to `false`
    pub fn stop(&self) {
        if self.is_running.load(Ordering::SeqCst) {
            self.is_running.store(false, Ordering::SeqCst);  // Updating the is_running flag to false
            self.shutdown_notify.notify_waiters(); // Notifies all tasks that are waiting for the shutdown signal.
            info!("Shutdown signal sent.");
        } else {
            warn!("Server was already stopped or not running.");
        }
    }
}



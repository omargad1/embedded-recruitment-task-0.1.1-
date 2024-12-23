use embedded_recruitment_task::{
    message::{client_message, server_message, AddRequest, EchoMessage},
    server::Server,
};
use std::{
    sync::Arc,
    
};
use tokio::runtime::Runtime; // import tokio runtime for async
use std::io::Write; // Import the Write trait for flush method

mod client;

fn setup_server_thread(server: Arc<Server>, runtime: &Runtime) -> tokio::task::JoinHandle<()> {
    runtime.spawn(async move {
        server.run().await.expect("Server encountered an error");
    })
}

fn create_server(runtime: &Runtime) -> Arc<Server> {
    runtime.block_on(async {
        Arc::new(Server::new("localhost:0").await.expect("Failed to start server"))
    })
}


///  test case verifies that the server can handle malformed messages
#[test]
fn test_client_connection() {
    
      // Create the Tokio runtime
      let runtime = Runtime::new().unwrap();
    
    // Set up the server in a separate thread
    let server = create_server(&runtime);

    let server_addr = server.local_addr().expect("Failed to get server address");  
    let port = server_addr.port(); // Retrieve the dynamic port 


    let handle = setup_server_thread(server.clone(),&runtime);

    // Create and connect the client
    let mut client = client::Client::new("localhost", port.into(), 1000);  
    assert!(client.connect().is_ok(), "Failed to connect to the server on prot {}",port);

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

     // Stop the server and wait for the thread to finish
     server.stop();
     runtime.block_on(async {
         handle.await.unwrap();
     })
}


#[test]
fn test_client_echo_message() {
  // Create the Tokio runtime
  let runtime = Runtime::new().unwrap();
    
  // Set up the server in a separate thread
  let server = create_server(&runtime);

  let server_addr = server.local_addr().expect("Failed to get server address");  
  let port = server_addr.port(); // Retrieve the dynamic port 


  let handle = setup_server_thread(server.clone(),&runtime);

  // Create and connect the client
  let mut client = client::Client::new("localhost", port.into(), 1000);  
  assert!(client.connect().is_ok(), "Failed to connect to the server on prot {}",port);

    // Prepare the message
    let mut echo_message = EchoMessage::default();
    echo_message.content = "Hello, World!".to_string();
    let message = client_message::Message::EchoMessage(echo_message.clone());

    // Send the message to the server
    assert!(client.send(message).is_ok(), "Failed to send message");

    // Receive the echoed message
    let response = client.receive();
    assert!(
        response.is_ok(),
        "Failed to receive response for EchoMessage"
    );

    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(
                echo.content, echo_message.content,
                "Echoed message content does not match"
            );
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

     // Stop the server and wait for the thread to finish
     server.stop();
     runtime.block_on(async {
         handle.await.unwrap();
     });
}

#[test]
fn test_multiple_echo_messages() {
   // Create the Tokio runtime
   let runtime = Runtime::new().unwrap();
    
   // Set up the server in a separate thread
   let server = create_server(&runtime);
 
   let server_addr = server.local_addr().expect("Failed to get server address");  
   let port = server_addr.port(); // Retrieve the dynamic port 
 
 
   let handle = setup_server_thread(server.clone(),&runtime);
 
   // Create and connect the client
   let mut client = client::Client::new("localhost", port.into(), 1000); 
   assert!(client.connect().is_ok(), "Failed to connect to the server on prot {}",port);

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    // Send and receive multiple messages
    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message);

        // Send the message to the server
        assert!(client.send(message).is_ok(), "Failed to send message");

        // Receive the echoed message
        let response = client.receive();
        assert!(
            response.is_ok(),
            "Failed to receive response for EchoMessage"
        );

        match response.unwrap().message {
            Some(server_message::Message::EchoMessage(echo)) => {
                assert_eq!(
                    echo.content, message_content,
                    "Echoed message content does not match"
                );
            }
            _ => panic!("Expected EchoMessage, but received a different message"),
        }
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

      // Stop the server and wait for the thread to finish
      server.stop();
      runtime.block_on(async {
          handle.await.unwrap();
      });
}


#[test]
fn test_multiple_clients() {
  // Create the Tokio runtime
  let runtime = Runtime::new().unwrap();
    
  // Set up the server in a separate thread
  let server = create_server(&runtime);

  let server_addr = server.local_addr().expect("Failed to get server address"); 
  let port = server_addr.port(); // Retrieve the dynamic port 


  let handle = setup_server_thread(server.clone(),&runtime);

  // Retrieve the dynamically assigned port from the server and use it for all clients
  let mut clients: Vec<client::Client> = (0..3)
  .map(|_| client::Client::new("localhost", port.into(), 1000))
  .collect();


    for client in clients.iter_mut() {
        assert!(client.connect().is_ok(), "Failed to connect to the server{}",port);
    }

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    // Send and receive multiple messages for each client
    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message.clone());

        for client in clients.iter_mut() {
            // Send the message to the server
            assert!(
                client.send(message.clone()).is_ok(),
                "Failed to send message"
            );

            // Receive the echoed message
            let response = client.receive();
            assert!(
                response.is_ok(),
                "Failed to receive response for EchoMessage"
            );

            match response.unwrap().message {
                Some(server_message::Message::EchoMessage(echo)) => {
                    assert_eq!(
                        echo.content, message_content,
                        "Echoed message content does not match"
                    );
                }
                _ => panic!("Expected EchoMessage, but received a different message"),
            }
        }
    }

    // Disconnect the clients
    for client in clients.iter_mut() {
        assert!(
            client.disconnect().is_ok(),
            "Failed to disconnect from the server"
        );
    }

      // Stop the server and wait for the thread to finish
      server.stop();
      runtime.block_on(async {
          handle.await.unwrap();
      });
}


#[test]
fn test_client_add_request() {
   
 // Create the Tokio runtime
 let runtime = Runtime::new().unwrap();
    
 // Set up the server in a separate thread
 let server = create_server(&runtime);

 let server_addr = server.local_addr().expect("Failed to get server address");  
 let port = server_addr.port(); // Retrieve the dynamic port 

 println!("Server is running on port: {}", port);

 let handle = setup_server_thread(server.clone(),&runtime);

  
    // Test synchronous client operations
    let mut client = client::Client::new("localhost", port.into(), 1000);
       
    println!("Client will connect to port: {}", port);

    println!("Attempting to connect...");
    assert!(client.connect().is_ok(), "Failed to connect to the server"); // added the connect function 
    println!("Connection successful. Sending message...");


    // Prepare the message
    let mut add_request = AddRequest::default();
    add_request.a = 10;
    add_request.b = 20;
    let message = client_message::Message::AddRequest(add_request.clone());

    // Send the message to the server
    assert!(client.send(message).is_ok(), "Failed to send message!!!");

    // Receive the response
    let response = client.receive();
    assert!(
        response.is_ok(),
        "Failed to receive response for AddRequest"
    );

    match response.unwrap().message {
        Some(server_message::Message::AddResponse(add_response)) => {
            assert_eq!(
                add_response.result,
                add_request.a + add_request.b,
                "AddResponse result does not match"
            );
        }
        _ => panic!("Expected AddResponse, but received a different message"),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for it to finish
    server.stop();
    runtime.block_on(async {
        handle.await.unwrap();
    });
}



// test ensures the server processes an AddRequest correctly and returns an appropriate AddResponse
#[test]
fn test_add_response() {
    // Initialize the Tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Set up the server
    let server = create_server(&runtime);
    let server_addr = server.local_addr().unwrap();
    let port = server_addr.port();
    let handle = setup_server_thread(server.clone(), &runtime);

    // Create and connect the client
    let mut client = client::Client::new("localhost", port.into(), 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare an AddRequest
    let mut add_request = AddRequest::default();
    add_request.a = 15; // Example value for a
    add_request.b = 25; // Example value for b
    let client_message = client_message::Message::AddRequest(add_request.clone());

    // Send the AddRequest to the server
    assert!(
        client.send(client_message).is_ok(),
        "Failed to send AddRequest to server"
    );

    // Receive the AddResponse
    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive AddResponse from server");

    // Validate the response
    match response.unwrap().message {
        Some(server_message::Message::AddResponse(add_response)) => {
            assert_eq!(
                add_response.result,
                add_request.a + add_request.b,
                "AddResponse result is incorrect"
            );
        }
        _ => panic!("Unexpected message type received. Expected AddResponse."),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server
    server.stop();
    runtime.block_on(async {
        handle.await.unwrap();
    });
}




// this test sends malformed message and 

#[test]
fn test_malformed_message() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Set up the server
    let server = create_server(&runtime);
    let server_addr = server.local_addr().unwrap();
    let port = server_addr.port();
    let handle = setup_server_thread(server.clone(), &runtime);

    println!("Connecting to server on port: {}", port);

    // Create a raw TCP connection to the server
    let mut raw_stream = std::net::TcpStream::connect(("localhost", port)).expect("Failed to connect to server");

    println!("Sending malformed message...");

    // Send malformed data
    let malformed_data = vec![0u8, 255u8, 128u8]; // Random bytes, not a valid Protobuf message
    raw_stream
        .write_all(&malformed_data)
        .expect("Failed to send malformed data");
    raw_stream.flush().expect("Failed to flush stream"); // Flush to ensure the data is sent

    println!("Malformed message sent.");

    // Clean up
    raw_stream.shutdown(std::net::Shutdown::Both).expect("Failed to close raw stream");

    server.stop();
    runtime.block_on(async {
        handle.await.unwrap();
    });
}



// his test creates multiple clients, each sending requests to the server concurrently, 
// and verifies that all responses are processed correctly.

#[test]
fn test_concurrent_requests() {
    // Initialize the Tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Set up the server
    let server = create_server(&runtime);
    let server_addr = server.local_addr().unwrap();
    let port = server_addr.port();
    let handle = setup_server_thread(server.clone(), &runtime);

    // Create and connect multiple clients
    let mut clients: Vec<_> = (0..10)
        .map(|_| client::Client::new("localhost", port.into(), 1000))
        .collect();

    for client in clients.iter_mut() {
        assert!(client.connect().is_ok(), "Failed to connect to the server");
    }

    // Simulate concurrent requests using threads
    let handles: Vec<_> = clients
        .into_iter()
        .map(|mut client| {
            std::thread::spawn(move || {
                // Send an EchoMessage request
                let mut echo_message = EchoMessage::default();
                echo_message.content = "Concurrent Test".to_string();
                let client_message = client_message::Message::EchoMessage(echo_message);

                assert!(
                    client.send(client_message).is_ok(),
                    "Failed to send message to server"
                );

                // Receive the response
                let response = client.receive();
                assert!(
                    response.is_ok(),
                    "Failed to receive concurrent response"
                );

                // Validate the response
                match response.unwrap().message {
                    Some(server_message::Message::EchoMessage(echo_response)) => {
                        assert_eq!(
                            echo_response.content,
                            "Concurrent Test",
                            "Server did not echo the message correctly"
                        );
                    }
                    _ => panic!("Unexpected response type received"),
                }

                // Disconnect the client
                assert!(
                    client.disconnect().is_ok(),
                    "Failed to disconnect from the server"
                );
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("A client thread panicked");
    }

    // Stop the server
    server.stop();
    runtime.block_on(async {
        handle.await.unwrap();
    });
}



// this test Simulates a high number of clients 
// connecting to the server simultaneously.

#[test]
fn test_high_client_load() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Set up the server
    let server = create_server(&runtime);
    let server_addr = server.local_addr().unwrap();
    let port = server_addr.port();
    let handle = setup_server_thread(server.clone(), &runtime);

    // Simulate 100 clients
    let mut clients: Vec<_> = (0..100)
        .map(|_| client::Client::new("localhost", port.into(), 1000))
        .collect();

    for client in clients.iter_mut() {
        assert!(client.connect().is_ok(), "Failed to connect to the server");
    }

    // Ensure all clients are connected
    assert_eq!(clients.len(), 100);

    // Disconnect all clients
    for client in clients.iter_mut() {
        assert!(client.disconnect().is_ok(), "Failed to disconnect from the server");
    }

    // Stop the server
    server.stop();
    runtime.block_on(async {
        handle.await.unwrap();
    });
}

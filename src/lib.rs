
/// This module contains the server logic for handling client connections and requests.
pub mod server;



/// This module includes Protobuf-generated message structures.
/// It is auto-generated during the build process.
pub mod message {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

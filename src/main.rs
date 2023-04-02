use clap::Parser;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Parse the address
    let addr = Address::parse().as_string();
    println!("Connecting to {}", addr);

    // Connect to the server
    let _conn = match TcpStream::connect(addr).await {
        Ok(conn) => {
            println!("Connected to the server! Press Ctrl+C to exit.");
            conn
        }
        Err(err) => {
            return Err(err);
        }
    };

    // Wait until the user interrupts the program
    tokio::signal::ctrl_c().await
}

/// A tool that connects to an address over TCP
#[derive(Parser)]
struct Address {
    /// The address to connect to
    address: String,
}

impl Address {
    fn as_string(self) -> String {
        self.address
    }
}

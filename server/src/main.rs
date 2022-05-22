fn main() {
    let server = Server::new("127.0.0.1: 8080".to_string());
    server.run();
}

struct Server {
    address: String,
}

// Difference between methods and functions in struct implementations is that methods
// are passed the struct as self. To avoid deallocation pass a reference to self.

impl Server {
    // Main Constructor - Function
    // Self alias refers to own type.
    fn new(address: String) -> Self {
        Server { address }
    }

    //Method
    fn run(self) {
        println!("Server is listening at {}.", self.address)
    }
}

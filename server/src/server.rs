pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
    Server { address }
  }

  //Method
  pub fn run(self) {
    println!("Server is listening at {}.", self.address)
  }
}

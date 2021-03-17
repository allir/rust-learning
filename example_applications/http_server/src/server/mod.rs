pub struct Server {
    address: String
}

impl Server{
    pub fn new(address:String, port: String) -> Self {
        Self { address: format!("{}:{}", address, port) }
    }

    pub fn run(&self) {
        println!("Listeining on {}", self.address);
    }
}

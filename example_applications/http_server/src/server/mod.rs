pub struct Server {
    address: String
}

impl Server{
    pub fn new(mut address:String, port: String) -> Self {
        address.push_str(":");
        address.push_str(&port);
        Self { address }
    }

    pub fn run(&self) {
        println!("Listeining on {}", self.address);
    }
}

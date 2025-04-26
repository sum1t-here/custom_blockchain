use std::time::SystemTime;
#[derive(Debug)]

struct Block {
    nonce: i32,
    previous_hash: Vec<u8>,
    time_stamp: u128,
    transactions: Vec<Vec<u8>>,
}

impl Block {
    fn new(nonce: i32, previous_hash: Vec<u8> ) -> Self {
        // this method will take control of the input of the previous_hash
        let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Block {
            nonce: nonce,
            previous_hash: previous_hash,
            time_stamp: time_now.as_nanos(),
            transactions: Vec::<Vec<u8>>::new(),
        }
    }

    fn print(&self) {
        println!("timestamp: {:x}", self.time_stamp);
        println!("nonce: {}", self.nonce);
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions);
    }
}

fn main() {
    // convert a string to bytes array
    // convert it to a String, into_bytes() => Vec<u8>
    let b = Block::new(0, "This is the genesis block".to_string().into_bytes());
    b.print();

    println!("The Genesis block is: {:?}", b);
}
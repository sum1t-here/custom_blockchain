use std::time::SystemTime;

#[derive(Debug)]
pub struct Block {
    pub nonce: i32,
    pub previous_hash: Vec<u8>,
    pub time_stamp: u128,
    pub transactions: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct BlockChain {
    pub transaction_pool: Vec<Vec<u8>>,
    pub chain: Vec<Block>,
}

impl Block {
    fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        // this method will take control of the input of the previous_hash
        let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Block {
            nonce: nonce,
            previous_hash: previous_hash,
            time_stamp: time_now.as_nanos(),
            transactions: Vec::<Vec<u8>>::new(), // Initializes an empty list of transactions,
            // each transaction will be stored as a Vec<u8> (e.g., serialized data)
        }
    }

    fn print(&self) {
        println!("timestamp: {:x}", self.time_stamp);
        println!("nonce: {}", self.nonce);
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions);
    }
}

impl BlockChain {
    pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };

        bc.create_block(0, "Hash for very first blog".to_string().into_bytes());
        bc
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b);
    }

    pub fn print(&self) {
        //  using iterator to loop over vector
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} Block {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
            println!("{}", "*".repeat(59));
        }
    }
}

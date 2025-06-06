use sha2::{ Digest, Sha256 };
use std::cmp::PartialEq;
use std::ops::AddAssign;
use std::ops::Index;
use std::time::Instant;
use std::time::SystemTime;
use transaction::*;

pub mod transaction;

pub trait Serialization<T> {
    fn serialization(&self) -> Vec<u8>;
    fn deserialization(bytes: Vec<u8>) -> T;
}

pub enum BlockSearch {
    // tag value
    SearchByIndex(usize),
    SearchByPreviousHash(Vec<u8>),
    SearchByBlockHash(Vec<u8>),
    SearchByNonce(i32),
    SearchByTimestamp(u128),
    SearchByTransaction(Vec<u8>),
}

pub enum BlockSearchResult<'a> {
    Success(&'a Block),
    FailOfEmptyBlocks,
    FailOfIndex(usize),
    FailOfPreviousHash(Vec<u8>),
    FailOfBlockHash(Vec<u8>),
    FailOfNonce(i32),
    FailOfTimeStamp(u128),
    FailOfTransaction(Vec<u8>),
}

#[derive(Debug)]
pub struct Block {
    pub nonce: i32,
    pub previous_hash: Vec<u8>,
    pub time_stamp: u128,
    pub transactions: Vec<Vec<u8>>,
}

impl AddAssign<i32> for Block {
    fn add_assign(&mut self, rhs: i32) {
        self.nonce += rhs;
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        let self_hash = self.hash();
        let other_hash = other.hash();
        self_hash == other_hash
    }
}

impl Index<usize> for BlockChain {
    type Output = Block;
    fn index(&self, idx: usize) -> &Self::Output {
        let res = self.chain.get(idx);
        match res {
            Some(block) => {
                return block;
            }
            None => {
                panic!("index out of range for the chain");
            }
        }
    }
}

#[derive(Debug)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
    //the address for the miner
    blockchain_address: String,
}

impl Block {
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
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

    pub fn print(&self) {
        println!("timestamp: {:x}", self.time_stamp);
        println!("nonce: {}", self.nonce);
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions);
        for (idx, tx) in self.transactions.iter().enumerate() {
            let transaction = Transaction::deserialization(tx.to_vec());
            println!("Transaction {}:", idx);
            println!(
                "  From (bytes): {:?}  => '{}'",
                transaction.sender_address,
                String::from_utf8_lossy(&transaction.sender_address)
            );
            println!(
                "  To (bytes): {:?}  => '{}'",
                transaction.recipient_address,
                String::from_utf8_lossy(&transaction.recipient_address)
            );
            println!("  Value: {}", transaction.value);
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();

        // add all these to bin
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamp.to_be_bytes());
        for tx in self.transactions.iter() {
            bin.extend(tx.clone());
        }

        let mut hasher = Sha256::new();
        hasher.update(bin);
        // return the result to vec
        hasher.finalize().to_vec()
    }
}

impl BlockChain {
    const DIFFICULTY: usize = 5;
    const MINING_SENDER: &str = "THE BLOCKCHAIN";
    const MINING_REWARD: u64 = 1;

    pub fn new(address: String) -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
            blockchain_address: address,
        };

        let b = Block::new(0, vec![0 as u8; 32]);

        bc.chain.push(b);
        bc.mining();
        bc
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let mut b = Block::new(nonce, previous_hash);
        for tx in self.transaction_pool.iter() {
            b.transactions.push(tx.clone());
        }
        self.transaction_pool.clear();
        let now = Instant::now();
        let proof_hash = BlockChain::do_proof_of_work(&mut b);
        let elapsed = now.elapsed();
        println!("compute time: {:?}\nproof for the current block is {:?}", elapsed, proof_hash);
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

    pub fn last_block(&self) -> &Block {
        if self.chain.len() > 1 {
            return &self.chain[self.chain.len() - 1];
        }

        &self.chain[0]
    }

    pub fn search_block(&self, search: BlockSearch) -> BlockSearchResult {
        for (idx, block) in self.chain.iter().enumerate() {
            match search {
                BlockSearch::SearchByIndex(index) => {
                    if idx == index {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfIndex(idx);
                    }
                }

                BlockSearch::SearchByPreviousHash(ref hash) => {
                    /*
                       enum matching can cause data ownership transfer, the hash value
                       attach to search is transfer to the local variable of hash here,
                       when the block is executed the value of hash is dropped, then in
                       next round we will not have any value to get
                    */
                    if block.previous_hash == *hash {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfPreviousHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByBlockHash(ref hash) => {
                    if block.hash() == *hash {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfPreviousHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByNonce(nonce) => {
                    if block.nonce == nonce {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfNonce(nonce);
                    }
                }

                BlockSearch::SearchByTimestamp(time_stamp) => {
                    if block.time_stamp == time_stamp {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfTimeStamp(time_stamp);
                    }
                }

                BlockSearch::SearchByTransaction(ref transaction) => {
                    for tx in block.transactions.iter() {
                        if tx == transaction {
                            return BlockSearchResult::Success(block);
                        }

                        if idx >= self.chain.len() {
                            return BlockSearchResult::FailOfTransaction(transaction.to_vec());
                        }
                    }
                }
            }
        }

        return BlockSearchResult::FailOfEmptyBlocks;
    }

    pub fn add_transaction(&mut self, tx: &impl Serialization<Transaction>) {
        for tx_in_pool in self.transaction_pool.iter() {
            if *tx_in_pool == tx.serialization() {
                break;
            }
        }

        self.transaction_pool.push(tx.serialization())
    }

    fn do_proof_of_work(block: &mut Block) -> String {
        loop {
            let hash = block.hash();
            let hash_str = hex::encode(&hash);
            // Check if the hash string starts with the required number of leading zeroes (i.e., difficulty target)
            if hash_str[0..BlockChain::DIFFICULTY] == "0".repeat(BlockChain::DIFFICULTY) {
                // If the hash meets the difficulty criteria, return it as a valid proof-of-work
                return hash_str;
            }
            // If not valid, increment the block's nonce to try again
            *block += 1;
        }
    }

    pub fn mining(&mut self) -> bool {
        /*
        if a block is mined, a transaction will created and the chain will send
        a coin to the miner
        */
        let tx = Transaction::new(
            BlockChain::MINING_SENDER.clone().into(),
            self.blockchain_address.clone().into(),
            BlockChain::MINING_REWARD
        );
        self.add_transaction(&tx);

        self.create_block(0, self.last_block().hash());

        true
    }

    pub fn calculate_total_amt(&self, address: String) -> i64 {
        let mut total_amt: i64 = 0;

        for i in 0..self.chain.len() {
            let block = &self[i];
            for t in block.transactions.iter() {
                let tx = Transaction::deserialization(t.clone());
                let value = tx.value;

                /*
                into is a trait used for converting one type into another type, String implement the trait
                of into with many different type, therefore we need to convert String to the trait with the 
                right type,here we want string convert itself to Vec<u8>, then we need to convert String
                to type Into<Vec<u8>>.
                */
                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.recipient_address {
                    total_amt += value as i64;
                }

                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.sender_address {
                    total_amt -= value as i64;
                }
            }
        }

        total_amt
    }
}

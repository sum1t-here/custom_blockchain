use std::time::SystemTime;
use sha2::{ Digest, Sha256 };
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

#[derive(Debug)]
pub struct BlockChain {
    pub transaction_pool: Vec<Vec<u8>>,
    pub chain: Vec<Block>,
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
    pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };

        bc.create_block(0, vec![0 as u8; 32]);
        bc
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let mut b = Block::new(nonce, previous_hash);
        for tx in self.transaction_pool.iter() {
            b.transactions.push(tx.clone());
        }
        self.transaction_pool.clear();
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
}

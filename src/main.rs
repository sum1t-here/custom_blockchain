pub mod blockchain;
use crate::blockchain::{ transaction::Transaction, Serialization };

use blockchain::{ Block, BlockChain, BlockSearch, BlockSearchResult };

fn get_block_search_result(result: BlockSearchResult) {
    match result {
        BlockSearchResult::Success(block) => {
            println!("Found given block {:?} ", block);
        }

        BlockSearchResult::FailOfIndex(index) => {
            println!("Failed to find block with given index: {}", index);
        }

        BlockSearchResult::FailOfEmptyBlocks => {
            println!("The block chain is empty");
        }

        BlockSearchResult::FailOfPreviousHash(hash) => {
            println!("No block has previous hash as {:?}", hash);
        }

        BlockSearchResult::FailOfBlockHash(hash) => {
            println!("No block has hash as {:?}", hash);
        }

        BlockSearchResult::FailOfNonce(nonce) => {
            println!("No block has nonce as {}", nonce);
        }

        BlockSearchResult::FailOfTimeStamp(time_stamp) => {
            println!("No block has timestamp as {}", time_stamp);
        }

        BlockSearchResult::FailOfTransaction(transaction) => {
            println!("No block contain the given transaction, {:?}", transaction);
        }
    }
}

fn main() {
    // testing the block creation logic

    // let b = Block::new(0, "This is the genesis block".to_string().into_bytes());
    // b.print();
    // println!("The Genesis block is: {:?}", b);

    // testing the block chain logic

    // let mut block_chain = BlockChain::new();
    // println!("Block chain : {:?}", block_chain);
    // block_chain.print();
    // let previous_hash = block_chain.last_block().hash();
    // // let hash_to_find = previous_hash.clone();

    // block_chain.create_block(1, previous_hash);
    // block_chain.print();

    // let tx = Transaction::new("sender".as_bytes().to_vec(), "recipients".as_bytes().to_vec(), 100);
    // println!("Transaction before sereialization: {}", tx);
    // let tx_bin = tx.serialization();
    // println!("bin of tx: {:?}", tx_bin);
    // let tx_1 = Transaction::deserialization(tx_bin);
    // println!("transaction from bin: {}", tx_1);

    // block_chain.add_transaction(&tx);

    // let previous_hash = block_chain.last_block().hash();
    // block_chain.create_block(2, previous_hash);
    // block_chain.print();

    // let result = block_chain.search_block(BlockSearch::SearchByIndex(1));
    // get_block_search_result(result);
    // let result = block_chain.search_block(BlockSearch::SearchByIndex(5));
    // get_block_search_result(result);
    // let result = block_chain.search_block(BlockSearch::SearchByBlockHash(hash_to_find));
    // get_block_search_result(result);

    // let mut blockchain = BlockChain::new();
    // blockchain.print();

    // let mut block1 = Block::new(0, "previous hash".as_bytes().to_vec());
    // let mut block2 = Block::new(0, "previous hash".as_bytes().to_vec());
    // println!("block1 == block2 : {}", block1 == block2);

    let block_chain = BlockChain::new();
    let block = &block_chain[0];
    println!("The first block is {:?}", block);
}

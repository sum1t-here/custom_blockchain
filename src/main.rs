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
    let my_blockchain_address = "my blockchain address";
    let mut blockchain = BlockChain::new(my_blockchain_address.into());
    blockchain.print();

    blockchain.add_transaction(&Transaction::new("A".into(), "B".into(), 1));
    blockchain.mining();
    blockchain.print();

    blockchain.add_transaction(&Transaction::new("C".into(), "D".into(), 2));
    blockchain.add_transaction(&Transaction::new("X".into(), "Y".into(), 5));
    blockchain.mining();
    blockchain.print();

    println!(
        "Value for miner {}",
        blockchain.calculate_total_amt(my_blockchain_address.to_string())
    );
    println!("Value for miner C {}", blockchain.calculate_total_amt("C".to_string()));
    println!("Value for miner D {}", blockchain.calculate_total_amt("D".to_string()))
}

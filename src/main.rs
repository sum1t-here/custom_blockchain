pub mod blockchain;
use blockchain::BlockChain;

fn main() {
    // let b = Block::new(0, "This is the genesis block".to_string().into_bytes());
    // b.print();

    // println!("The Genesis block is: {:?}", b);

    let mut block_chain = BlockChain::new();
    println!("Block chain : {:?}", block_chain);

    block_chain.print();

    block_chain.create_block(1, "hash 1".to_string().into_bytes());
    block_chain.print();
    block_chain.create_block(2, "hash 2".to_string().into_bytes());
    block_chain.print();
}

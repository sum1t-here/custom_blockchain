pub mod blockchain;
use blockchain::BlockChain;

fn main() {
    // testing the block creation logic

    // let b = Block::new(0, "This is the genesis block".to_string().into_bytes());
    // b.print();
    // println!("The Genesis block is: {:?}", b);

    // testing the block chain logic

    let mut block_chain = BlockChain::new();
    println!("Block chain : {:?}", block_chain);
    block_chain.print();
    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);
    block_chain.print();
    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(2, previous_hash);
    block_chain.print();
}

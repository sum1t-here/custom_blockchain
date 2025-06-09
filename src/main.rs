pub mod wallet;
use wallet::Wallet;

fn main() {
    let wallet = Wallet::new();
    println!("private key: {}", wallet.private_key_str());
    println!("public key: {}", wallet.public_key_str());
    println!("address: {}", wallet.get_address());

    let transaction = wallet.sign_transaction(&"0x1234567890".to_string(), 100);
    println!("transaction : {:?}", transaction);
    println!("verify: {}", Wallet::verify_transaction(&transaction));
}

use p256::ecdsa::{ signature::{ Signer, Verifier }, Signature, SigningKey, VerifyingKey };
use rand_core::OsRng;

fn main() {
    // closure
    let gen_signing_key = || { SigningKey::random(&mut OsRng) };

    // generate key pair
    // 32 bytes random number
    let signing_key = gen_signing_key();
    let mut verifying_key: Option<VerifyingKey> = None;
    // mut closure
    let mut gen_verifying_key = |private_key: &SigningKey| {
        verifying_key = Some(VerifyingKey::from(private_key));
    };

    gen_verifying_key(&signing_key);

    println!("private key: {:?}", signing_key.to_bytes());
    println!("public key: {:?}", verifying_key.unwrap().to_encoded_point(false));

    let message = b"Hello World!!!";

    // sign message with private key
    let signature: Signature = signing_key.sign(message);
    println!("signature: {:?}", signature);

    let res = verifying_key.unwrap().verify(message, &signature);
    match res {
        Ok(_) => println!("Signature verfifed ok"),
        Err(_) => println!("Signature verification failed"),
    }
}

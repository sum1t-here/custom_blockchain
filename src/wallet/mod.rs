use p256::ecdsa::{ signature::{ Signer, Verifier }, Signature, SigningKey, VerifyingKey };
use rand_core::OsRng;
use sha2::{ Sha256, Digest };
use ripemd160::{ Ripemd160, Digest as RipDigest };
use serde::Serialize;
/*
    1. Do sha256 hash on the x,y of public key
    2. Do ripemd160 hash on the result of step 1 and we will get 20 bytes result
    3. add a version byte at the head of result from step 2 (0x00 for mainnet)
    4. do sha256 hash on the result of step 3
    5. do sha256 hash on the result of step 4
    6. take the first 4 bytes of result of step 5 as checksum
    7. append the 4 bytes from step 6 to the end of the result from step 3
    8. encode result of step 7 by base58
*/

pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    address: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub signature: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Self {
        // Generate a random signing (private) key
        let signing_key = SigningKey::random(&mut OsRng);

        // Derive the corresponding verifying (public) key
        // `.verifying_key()` returns a reference, so we clone it to get an owned value
        let verifying_key = signing_key.verifying_key().clone();

        let mut address = String::new();
        let mut gen_address = || {
            let key_points = verifying_key.to_encoded_point(false);

            if let (Some(x), Some(y)) = (key_points.x(), key_points.y()) {
                let mut pub_key_bytes = Vec::with_capacity(x.len() + y.len());
                pub_key_bytes.extend_from_slice(x);
                pub_key_bytes.extend_from_slice(y);

                // sha256 on public key
                let hash = Sha256::digest(&pub_key_bytes);
                //ripemd160 hash on sha256 hash
                let mut hasher = Ripemd160::new();
                hasher.update(&hash);
                let mut hash_result = hasher.finalize().to_vec();
                //add byte version in front of ripemd160 hash (0x00 for mainnet)
                hash_result.insert(0, 0x00);
                //do sha256 hash on the result
                let hash2 = Sha256::digest(&hash_result);
                //take the first 4 bytes

                //do shash256 on the previous sha256 hash
                let hash3 = Sha256::digest(&hash2);
                let checksum = &hash3[0..4];
                //add checksum adn the end of extended ripemd160 hash
                let full_hash = [hash_result, checksum.to_vec()].concat();
                //base58 encode the result
                address = bs58::encode(full_hash).into_string();
            } else {
                address = String::new();
            }
        };

        gen_address();

        Self {
            signing_key,
            verifying_key,
            address,
        }
    }

    pub fn private_key_str(&self) -> String {
        // Serialize the private key to raw bytes and encode as hex
        hex::encode(self.signing_key.to_bytes())
    }

    pub fn public_key_str(&self) -> String {
        // Get the uncompressed public key (includes x and y coordinates)
        let key_points = self.verifying_key.to_encoded_point(false);

        // If x and y coordinates are present, encode them
        if let (Some(x), Some(y)) = (key_points.x(), key_points.y()) {
            // Convert both x and y to hex and concatenate
            let pub_str = hex::encode(x) + hex::encode(y).as_str();
            pub_str
        } else {
            // Return an empty string
            String::new()
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn sign_transaction(&self, receiver: &String, amount: u64) -> Transaction {
        let mut transaction = Transaction {
            sender: self.address.clone(),
            recipient: receiver.clone(),
            amount,
            signature: String::new(),
            public_key: self.public_key_str(),
        };

        let serialized_str = serde_json::to_string(&transaction).unwrap();
        let serialized = serialized_str.as_bytes();
        let sig: Signature = self.signing_key.sign(serialized);
        transaction.signature = hex::encode(sig.to_bytes());
        transaction
    }

    pub fn verify_transaction(transaction: &Transaction) -> bool {
        let signature_str = transaction.signature.clone();
        let signature_bin = hex::decode(signature_str).unwrap();
        let mut transaction_clone = transaction.clone();
        transaction_clone.signature = String::new();

        let serialized_str = serde_json::to_string(&transaction_clone).unwrap();
        let serialized = serialized_str.as_bytes();

        //convert the signature from string to instance of Signature struct
        //need to make sure the binary data is 64 bytes long
        let sig_array: [u8; 64] = signature_bin.try_into().unwrap();

        //param for from_bytes is GenericArray
        let signature = match Signature::from_bytes(&sig_array.into()) {
            Ok(sig) => sig,
            Err(e) => {
                println!("error: {:?}", e);
                return false;
            }
        };

        let public_key_str = transaction_clone.public_key.clone();
        //conver the binary data into VerifyingKey
        let mut public_key_bin = hex::decode(public_key_str).unwrap();
        /*
        if we want to convert binary data into VerifyingKey, we need to make sure the
        binary data is in sec1 format: [0x04 || x coordinate || y coordinate]
        public_key_bin => [x || y]
        insert (0x04)
        */
        public_key_bin.insert(0, 0x04);
        let public_key = VerifyingKey::from_sec1_bytes(&public_key_bin).unwrap();
        public_key.verify(serialized, &signature).is_ok()
    }
}

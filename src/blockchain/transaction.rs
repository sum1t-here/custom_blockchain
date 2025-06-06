use crate::blockchain::*;
use borsh::{ BorshDeserialize, BorshSerialize };
use std::fmt;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct Transaction {
    pub sender_address: Vec<u8>,
    pub recipient_address: Vec<u8>,
    pub value: u64,
}

impl Transaction {
    pub fn new(sender: Vec<u8>, recipient: Vec<u8>, value: u64) -> Transaction {
        Transaction {
            sender_address: sender,
            recipient_address: recipient,
            value,
        }
    }
}

impl Serialization<Transaction> for Transaction {
    fn serialization(&self) -> Vec<u8> {
        /*
            1. 8 bytes for length of sender address vector
            2. bytes for sender address
            3. 8 bytes for length of recipient address vector
            4. bytes of recipient address vector
            5. 8 bytes for length of the value
            6. bytes of the value field
        */

        let mut bin = Vec::<u8>::new();

        // let len_sender = self.sender_address.len();
        // bin.extend(len_sender.to_be_bytes().to_vec());
        // bin.extend(&self.sender_address);

        // let len_recipient = self.recipient_address.len();
        // bin.extend(len_recipient.to_be_bytes().to_vec());
        // bin.extend(&self.recipient_address);

        // bin.extend(self.value.to_be_bytes().to_vec());

        // bin

        self.serialize(&mut bin).unwrap();

        bin
    }

    fn deserialization(bytes: Vec<u8>) -> Transaction {
        // let mut pos = 0;

        // // try_into trait convert slice into array
        // let len_sender = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        // let mut sender_address = Vec::<u8>::new();
        // pos += 8;
        // sender_address.extend_from_slice(&bytes[pos..pos + len_sender]);
        // pos += len_sender;

        // let len_recipient = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        // pos += 8;

        // let mut recipient_address = Vec::<u8>::new();
        // recipient_address.extend_from_slice(bytes[pos..pos + len_recipient].try_into().unwrap());
        // pos += len_recipient;

        // let value = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        // Transaction {
        //     sender_address,
        //     recipient_address,
        //     value,
        // }

        let tx = Transaction::try_from_slice(&bytes).expect("Borsh deserialization failed");

        tx
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nsender address: {:?}\nrecipient address:{:?}\nvalue:{}\n{}\n",
            "-".repeat(40),
            self.sender_address,
            self.recipient_address,
            self.value,
            "-".repeat(40)
        )
    }
}

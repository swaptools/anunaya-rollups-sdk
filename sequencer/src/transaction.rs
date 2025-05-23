use crate::error::SequencerError;
use ::serde::{Deserialize, Serialize};
use alloy::primitives::{Address, Signature, TxNonce as Nonce};

type Amount = u64;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub amount: Amount,
    pub destination: Address,
    pub nonce: Nonce,
}

impl Transaction {
    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_string(&self)
            .expect("Serialization should not fail")
            .as_bytes()
            .to_vec()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Signature,
}

impl SignedTransaction {
    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_string(&self)
            .expect("Serialization should not fail")
            .as_bytes()
            .to_vec()
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
    }

    pub fn recover(&self) -> Result<Address, SequencerError> {
        let bytes = self.transaction.encode();
        self.signature
            .recover_address_from_msg(bytes)
            .map_err(|e| SequencerError::SignatureError(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;
    use alloy::signers::{Signer, local::PrivateKeySigner};

    use super::*;
    #[tokio::test]
    async fn test_transaction_signature() {
        // Set up a random signer.
        let alice = PrivateKeySigner::random();

        let transaction = Transaction {
            amount: 100,
            destination: alice.address(),
            nonce: 1,
        };
        let signature = alice.sign_message(&transaction.encode()).await.unwrap();
        let signed_transaction = SignedTransaction {
            transaction,
            signature,
        };

        let recovered_address = signed_transaction
            .recover()
            .expect("Should recover address");
        assert_eq!(recovered_address, alice.address());
    }
}

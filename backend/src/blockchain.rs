use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Block {
    pub id: Uuid,
    pub hash: String,
    pub previous_hash: String,
    pub transactions: String,
    pub nonce: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Block {
    pub fn new(previous_hash: &str, transactions: &str) -> Self {
        let mut block = Self {
            id: Uuid::new_v4(),
            hash: String::new(),
            previous_hash: previous_hash.to_string(),
            transactions: transactions.to_string(),
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.previous_hash, self.transactions, self.nonce, self.timestamp
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

extern crate ring;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate hex;

use ring::digest;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u32
}

#[derive(Serialize, Debug)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    prev_hash: String,
    merkle_root: String,
    difficulty: u32
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: BlockHeader,
    num_transactions: u32,
    transactions: Vec<Transaction>
}

pub struct Chain {
    blocks: Vec<Block>,
    curr_transactions: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    initial_block_reward: u32,
    subsidy_halving_interval: u32
}

// struct methods
impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            blocks: Vec::new(),
            curr_transactions: Vec::new(),
            difficulty,
            miner_addr,
            initial_block_reward: 100,
            subsidy_halving_interval: 100
        };
        chain.generate_new_block();
        chain
    }

    pub fn add_transaction(&mut self, sender: String, recipient: String, amount: u32) -> bool {
        self.curr_transactions.push(Transaction{
            sender,
            recipient,
            amount
        });
        true
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let input = input.as_bytes();
        let result = digest::digest(DIGEST_ALG, input);
        let result = result.as_ref();
        let result = result.to_vec();
        hex::encode(result)
    }

    pub fn calculate_merkle_root(curr_transactions: Vec<Transaction>) -> String {
        let mut merkle_tree : Vec<String> = Vec::new();

        for tx in &curr_transactions {
            let hash = Chain::hash(tx);
            merkle_tree.push(hash);
        }

        // why do I need this?
        if merkle_tree.len() % 2 == 1 {
            let last = merkle_tree.last().cloned().unwrap();
            merkle_tree.push(last);
        }

        while merkle_tree.len() > 1 {
            let mut h1 = merkle_tree.remove(0);
            let mut h2 = merkle_tree.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle_tree.push(nh);
        }

        merkle_tree.pop().unwrap()
    }

    pub fn calculate_block_reward(&self) -> u32 {
        let chain_height = self.blocks.len() as u32;
        let halves = chain_height / self.subsidy_halving_interval;
        if halves > 0 {
            self.initial_block_reward / (2*halves)
        } else {
            self.initial_block_reward
        }
    }

    pub fn get_prev_hash(&self) -> String {
        let block = match self.blocks.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap() // for genesis block
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            prev_hash: self.get_prev_hash(),
            merkle_root: String::new(),
            difficulty: self.difficulty
        };
        let coinbase_transaction = Transaction {
            sender: String::from("coinbase"),
            recipient: self.miner_addr.clone(),
            amount: self.calculate_block_reward()
        };
        let mut block = Block {
            header,
            num_transactions: 0,
            transactions: vec![]
            //transactions: vec![coinbase_transaction]
        };
        block.transactions.push(coinbase_transaction);
        block.transactions.append(&mut self.curr_transactions);
        block.num_transactions = block.transactions.len() as u32;
        block.header.merkle_root = Chain::calculate_merkle_root(block.transactions.clone());
        Chain::mine(&mut block.header);

        println!("{:?}", &block); // print with debug flag
        self.blocks.push(block);

        true
    }

    pub fn mine(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[ ..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Mined block: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

}
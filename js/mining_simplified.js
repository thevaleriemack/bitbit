const crypto = require('crypto');
/*
The hash puzzle includes 3 pieces of data:
A nonce, the hash of the previous block, and a set of transactions
*/

function concatenate(nonce, prev_hash, transactions) {
  return nonce + prev_hash + transactions.join("");
}

// Now we want to hash this concatenated value.

function hashSHA256(concatenated) {
  // Bitcoin uses the SHA256 hash algorithm for mining
  const hash = crypto.createHash('sha256');
  hash.update(""+concatenated);
  return hash.digest('hex');
}

/*
If we get a hash value inside of our defined target space we have found a
solution to the puzzle. In this case, we will print the hash solution (which
will be this block's hash) and return true.
*/

function solveHashPuzzle(nonce, prev_hash, transactions) {
  const concatenated = concatenate(nonce, prev_hash, transactions);
  const proposed_solution = hashSHA256(concatenated);
  /*
  For the sake of example, we will define our target space as any hash output
  with 2 leading zeros. In Bitcoin it is actually 32 leading zeros.
  */
  if (proposed_solution.substring(0,2) == "00") {
    console.log("Solution found:", proposed_solution);
    return true;
  }
  return false;
}

// Now let's mine!

function mine_simple(max_nonce, prev_hash, transactions,) {
  console.log('\nMining...');
  // Note: max_nonce is just used for demonstration purposes to avoid an endless
  // loop
  let nonce = 0; // Initalized to zero -- Is this true in Bitcoin?
  while (
    solveHashPuzzle(nonce, prev_hash, transactions) == false
    && nonce < max_nonce
  ) {
    nonce++;
  }
  console.log(`Nonce that produced solution: ${nonce}`);
}

// Uncomment the code below to see the program run

/*
const hash = crypto.createHash('sha256');
hash.update("Satoshi Nakamoto");
const _max_nonce = 100000;
const prev_hash = hash.digest('hex');
const transactions = ["tx1", "tx2", "tx3"];

mine_simple(_max_nonce, prev_hash, transactions);
*/

/*
Notice the nonce value tells us how many attempts were required to find a
solution. This is also an approximation for difficulty.
*/

module.exports = {
  concatenate,
  hashSHA256
};

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
  const hash = crypto.createHash('sha256');
  hash.update(""+concatenated);
  return hash.digest('hex');
}

module.exports = {
  concatenate,
  hashSHA256
};

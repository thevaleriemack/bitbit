const crypto  = require('crypto');

function privtopub(priv) {
  const ecdh = crypto.createECDH('secp256k1');
  ecdh.setPrivateKey(priv); // priv is a buffer
  return ecdh.getPublicKey('hex');
}

function hashSHA256(value, format='') {
  const hash = crypto.createHash('sha256');
  hash.update(""+value); // must be a string
  return hash.digest(format); // returns buffer by default
}

function Person(first_name, last_name) {
  /*
  It would be better to hash something random that no one would be able to guess
  */
  const private_key_buffer = hashSHA256(first_name+" "+last_name);
  this.private_key = private_key_buffer.toString('hex');
  this.public_key = privtopub(private_key_buffer);
  this.address = null;
}

// alice = new Person("Alice", "Nakamoto");
// console.log(alice);

module.exports = Person;

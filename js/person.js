const crypto  = require('crypto');

function privtopub(priv) {
  const ecdh = crypto.createECDH('secp256k1');
  ecdh.setPrivateKey(priv); // priv is a buffer
  return ecdh.getPublicKey('hex');
}

function sha256(value, secret='theval', format='') {
  // returns buffer
  return crypto.createHmac('sha256', secret)
               .update(value)
               .digest(format);
}

function Person(first_name, last_name) {
  /*
  It would be better to hash something random that no one would be able to guess
  */
  this.private_key = sha256(first_name+" "+last_name, undefined, 'hex');
  this.public_key = privtopub(sha256(first_name+" "+last_name));
  this.address = null;
}

alice = new Person("Alice", "Nakamoto");
console.log(alice);

module.exports = Person;

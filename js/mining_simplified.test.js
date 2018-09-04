const expect = require('chai').expect;

const ms = require('./mining_simplified.js');
const crypto = require('crypto');

describe('concatenate', function() {
  let nonce = 0;
  let prev_hash = "Satoshi Nakamoto";
  let transactions = ["tx1", "tx2", "tx3"];

  it('should return joined arguments into a single string in order',
    () => {
      concatenated = ms.concatenate(nonce, prev_hash, transactions);
      expect(concatenated).to.equal("0Satoshi Nakamototx1tx2tx3");
      expect(concatenated).to.be.a('string');
    });
  it('should accept strings or integers as the first argument',
    () => {
      ex = ms.concatenate(11, prev_hash, transactions);
      expect(ex).to.equal("11Satoshi Nakamototx1tx2tx3");
      ex1 = ms.concatenate("Hello ", prev_hash, transactions);
      expect(ex1).to.equal("Hello Satoshi Nakamototx1tx2tx3");
    });
});

describe('hashSHA256', function() {
  it('should return the hexidecimal hash of its input',
    () => {
      ex = ms.hashSHA256("AliceNakamoto");
      expect(ex).to.equal("f8fb4442566daf26d1a3d003e1b42cdd686ac68e0c1814d41249bcc8f8aa1355");
  });
  it('should be consistent',
    () => {
      ex = ms.hashSHA256("AliceNakamoto");
      ex1 = ms.hashSHA256("AliceNakamoto");
      expect(ex).to.equal(ex1);
  });
  it('should convert arguments to strings',
    () => {
      ex = ms.hashSHA256(1123);
      expect(ex).to.equal("4739ee3bd29e4f415da8ba9298a087e0fdc9c61378420ba8fbbab298bd74c4df");
  });
});

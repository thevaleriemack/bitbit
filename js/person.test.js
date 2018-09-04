const expect = require('chai').expect;

const Person = require('./person.js');

describe('new Person("Alice", "Nakamoto")', function() {
  const alice = new Person("Alice", "Nakamoto");
  const private_key = '2d3ba8456fb4b275cfc0e5a0e189b4603174ddbbb55d743853564e2f7b24155a';
  const public_key = '04513bcbb2a4d384cad45fd21b5a3ab4c47d8517ef0f54b0c12bc30eb6f21f847aa31476bf782677c263d2bf1efbe2168a99442e7a613550125d5aabe194940294';
  it('should create a person with keys and an address based on "Alice Nakamoto"',
    () => {
      expect(alice.private_key).to.equal(private_key);
      expect(alice.public_key).to.equal(public_key);
      expect(alice.address).to.be.null;
    });
});

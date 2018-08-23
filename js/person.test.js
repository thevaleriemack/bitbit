const expect = require('chai').expect;

const Person = require('./person.js');

describe('new Person("Alice", "Nakamoto")', function() {
  const alice = new Person("Alice", "Nakamoto");
  const private_key = '1dd8c0c13e49e7c5b60107f1c837bc8da2e118496084d2db2848d34f9ce0586a';
  const public_key = '04a41d97c9932764b042cd05223b54eb19ec350cf83400fbc38ca4f573782393e82e4bdf62603600f715f0e5a273dfb536513bf269282e980049bffd5bb285aca2';
  it('should create a person with keys and an address based on Alice Nakamoto',
    () => {
      expect(alice.private_key).to.equal(private_key);
      expect(alice.public_key).to.equal(public_key);
      expect(alice.address).to.be.null;
    });
});

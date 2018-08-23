const expect = require('chai').expect;

const hello = require('./sample.js');

describe('hello()', function() {
  it('should return "Hello, Mocha"', ()=> {
    expect(hello()).to.equal("Hello, Mocha");
  });
});

pragma solidity >=0.4.0 <0.6.0;
// semantic versioning but in major 0
// https://semver.org/
// for now, breaking changes are introduced with each new minor release

contract SimpleStorage {
    // declare state variables, permanently stored in contract storage
    uint256 storedData; // unsigned integers up to 2^256 - 1

    function set(uint x) public { // uint is equivalent to uint256
        storedData = x; // x is in memory, storedData is in storage
    }

    function get() public view returns (uint) {
        return storedData;
    }

    // You may also see function signatures with a "named" or "implicit" return value
    // function get() public view returns (uint data) {
    //     data = storedData;
    // }
}
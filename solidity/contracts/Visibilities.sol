pragma solidity ^0.4.24;

contract Visibilities {
    /*
     * Public vs External
     * ==================
     * In public functions, Solidity immediately copies array arguments to
     * memory, while external functions can read directly from calldata.
     * Memory allocation is expensive, whereas reading from calldata is cheap.
     */
    
    function testPublic(uint[20] a) public pure returns (uint) {
        /*
         * This function produces a warning from the compiler:
         *
         * Gas requirement of function Visibilities.testPublic(uint256[20])
         * high: infinite. If the gas requirement of a function is higher than
         * the block gas limit, it cannot be executed. Please avoid loops in
         * your functions or actions that modify large areas of storage (this
         * includes clearing or copying arrays in storage)
         */
        // TODO: use gasleft()
        // tx cost: 25642
        // exec cost: 530
        return a[10]*2;
    }
    
    function testExternal(uint[20] a) external pure returns (uint) {
        // TODO: use gasleft()
        // tx cost: 25451
        // exec cost: 339
        return a[10]*2;
    }
    
}

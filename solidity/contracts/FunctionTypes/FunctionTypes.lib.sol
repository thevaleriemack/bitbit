pragma solidity ^0.4.24;

// logic
/* 
 * Library functions can only be called directly (i.e. without the use of
 * DELEGATECALL) if they do not modify the state (i.e. if they are view or pure
 * functions), because libraries are assumed to be stateless.
 */

library FunctionTypesLibrary {

    function makeMetal(
        uint256 a,
        uint256 b,
        function (uint256, uint256) internal pure returns (uint256) fun
        ) internal pure returns (uint256) {
        return fun(a, b);
    }

    function makeGold(uint256 a, uint256 b) internal pure returns (uint256) {
        return (a + b) * 10;
    }

    function makeSilver(uint256 a, uint256 b) internal pure returns (uint256) {
        return (a + b) * 2;
    }
}

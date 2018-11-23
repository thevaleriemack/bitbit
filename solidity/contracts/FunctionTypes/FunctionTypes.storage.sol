pragma solidity ^0.4.24;
import { FunctionTypes as lib } from "./FunctionTypes.lib.sol";

// storage

contract FunctionTypes {

    mapping (
        address => function (uint256, uint256) internal pure returns (uint256)
    ) addressFun;

    event Calculation(uint256 out);

    function registerMaker() public payable {
        addressFun[msg.sender] = msg.value > 31415 ? lib.makeGold : lib.makeSilver;
    }

    function makeMetal(uint256 a, uint256 b) public {
        emit Calculation(lib.makeMetal(a, b, addressFun[msg.sender]));
    }
    
}
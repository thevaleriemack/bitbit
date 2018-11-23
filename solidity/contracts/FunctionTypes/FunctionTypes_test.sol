pragma solidity >=0.4.0 <0.6.0;
import "remix_tests.sol"; // injected by remix-tests npm module
import {
    FunctionTypes as FunctionTypesLib
} from "./FunctionTypes.lib.sol";
import {
    FunctionTypes as FunctionTypesStorage
} from "./FunctionTypes.storage.sol";

contract test_Storage {
    FunctionTypesStorage functionTypesStorage;
    FunctionTypesLib functionTypesLib;

    function beforeAll() public {
        functionTypesStorage = new FunctionTypesStorage();
    }

    function testRegisterMaker() public returns (bool) {
        address(functionTypesStorage)
            .delegatecall.gas(40000).value(30000)("registerMaker");
        bool t1 = Assert.equal(
            functionTypesLib.addressFun[msg.sender],
            functionTypesLib.makeSilver,
            "Expected function to be makeSilver"
        );
        address(functionTypesStorage)
            .delegatecall.gas(40000).value(32000)("registerMaker");
        bool t2 = Assert.equal(
            functionTypesLib.addressFun[msg.sender],
            functionTypesLib.makeGold,
            "Expected function to be makeGold"
        );
        return(t1 && t2);
    }
}

contract test_Library {
    FunctionTypesLib functionTypesLib;

    function beforeAll() public {
        functionTypesLib = new FunctionTypesLib();
    }

    function testMakeGold() public returns (bool) {
        bool t1 = Assert.equal(
            functionTypesLib.makeGold(2, 3),
            50,
            "Expected 50 from makeGold(2, 3)"
        );
        bool t2 = Assert.equal(
            functionTypesLib.makeGold(0, 0),
            0,
            "Expected 0 from makeGold(0, 0)"
        );
        return(t1 && t2);
    }
}

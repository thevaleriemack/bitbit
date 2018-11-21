pragma solidity >=0.4.16 <0.6.0;

contract StorageVsMemory {
    /* 
     * Storage vs Memory
     * ==================
     * It is important that we understand what data held in storage vs memory.
     */
    
    // state variables are held in storage by default
    // the getter functions generated for public state variables are external
    uint[11] base2;

    function test() external returns (uint[11]) {
        testStorage(base2);
        testMemory(base2);
        return base2;
    }
    
    function testAndShowGas() external returns (
        uint256 gasUsedForStorage,
        uint256 gasUsedForMemory,
        uint[11] items
        ) {
        uint256 startGas = gasleft();
        
        testStorage(base2);
        gasUsedForStorage = startGas - gasleft();
        
        startGas = gasleft();
        
        testMemory(base2);
        gasUsedForMemory = startGas - gasleft();
        
        items = base2;
    }

    function testMemory(uint[11] memory a) internal pure {
        // We can use the pure modifier on this function because of the fact
        // that it is not reading or modifying data allocated to storage.
        a[10] = 4201;
    }

    function testStorage(uint[11] storage a) internal {
        a[10] = 1024;
    }

}

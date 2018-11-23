using System;
using System.Threading.Tasks;
using System.Numerics;
using Nethereum.Hex.HexTypes;
using Nethereum.ABI.FunctionEncoding.Attributes;
using Nethereum.Web3;

namespace DefaultNamespace
{
   public class StorageVsMemoryService
   {
        private readonly Web3 web3;

        public static string ABI = @"[{'constant':false,'inputs':[],'name':'testAndShowGas','outputs':[{'name':'gasUsedForStorage','type':'uint256'},{'name':'gasUsedForMemory','type':'uint256'},{'name':'items','type':'uint256[11]'}],'payable':false,'stateMutability':'nonpayable','type':'function'},{'constant':false,'inputs':[],'name':'test','outputs':[{'name':'','type':'uint256[11]'}],'payable':false,'stateMutability':'nonpayable','type':'function'}]";

        public static string BYTE_CODE = "0x608060405234801561001057600080fd5b5061026d806100206000396000f30060806040526004361061004b5763ffffffff7c0100000000000000000000000000000000000000000000000000000000600035041663185498288114610050578063f8a8fd6d146100ae575b600080fd5b34801561005c57600080fd5b506100656100fc565b6040518084815260200183815260200182600b60200280838360005b83811015610099578181015183820152602001610081565b50505050905001935050505060405180910390f35b3480156100ba57600080fd5b506100c3610193565b604051808261016080838360005b838110156100e95781810151838201526020016100d1565b5050505090500191505060405180910390f35b600080610107610221565b60005a9050610116600061020a565b5a810393505a604080516101608101918290529192506101579190600090600b9082845b81548152602001906001019080831161013a575050505050610215565b5a604080516101608101918290529183039450600090600b9082845b815481526020019060010190808311610173575050505050915050909192565b61019b610221565b6101a5600061020a565b604080516101608101918290526000805482526101d692600b9060016020850180831161013a575050505050610215565b6040805161016081019182905290600090600b9082845b8154815260200190600101908083116101ed575050505050905090565b61040081600a015550565b61106961014090910152565b61016060405190810160405280600b9060208202803883395091929150505600a165627a7a72305820c09c835392406ab0283f752fb753a98989272dae5e2690235631754ce61ec1d10029";

        public static Task<string> DeployContractAsync(Web3 web3, string addressFrom,  HexBigInteger gas = null, HexBigInteger valueAmount = null) 
        {
            return web3.Eth.DeployContract.SendRequestAsync(ABI, BYTE_CODE, addressFrom, gas, valueAmount );
        }

        private Contract contract;

        public StorageVsMemoryService(Web3 web3, string address)
        {
            this.web3 = web3;
            this.contract = web3.Eth.GetContract(ABI, address);
        }

        public Function GetFunctionTestAndShowGas() {
            return contract.GetFunction("testAndShowGas");
        }
        public Function GetFunctionTest() {
            return contract.GetFunction("test");
        }


        public Task<BigInteger[]> TestAsyncCall() {
            var function = GetFunctionTest();
            return function.CallAsync<BigInteger[]>();
        }

        public Task<string> TestAndShowGasAsync(string addressFrom,  HexBigInteger gas = null, HexBigInteger valueAmount = null) {
            var function = GetFunctionTestAndShowGas();
            return function.SendTransactionAsync(addressFrom, gas, valueAmount);
        }
        public Task<string> TestAsync(string addressFrom,  HexBigInteger gas = null, HexBigInteger valueAmount = null) {
            var function = GetFunctionTest();
            return function.SendTransactionAsync(addressFrom, gas, valueAmount);
        }

        public Task<TestAndShowGasDTO> TestAndShowGasAsyncCall() {
            var function = GetFunctionTestAndShowGas();
            return function.CallDeserializingToObjectAsync<TestAndShowGasDTO>();
        }


    }

    [FunctionOutput]
    public class TestAndShowGasDTO 
    {
        [Parameter("uint256", "gasUsedForStorage", 1)]
        public BigInteger GasUsedForStorage {get; set;}

        [Parameter("uint256", "gasUsedForMemory", 2)]
        public BigInteger GasUsedForMemory {get; set;}

        [Parameter("uint256[11]", "items", 3)]
        public BigInteger[] Items {get; set;}

    }



}


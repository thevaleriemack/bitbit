# Solidity Example Code

### Remix
https://remix.ethereum.org

#### Compilation

#### Testing

Remix has its own testing library. The IDE will generate a template for you to unit test your contracts.

#### Deployment

#### Execution

If you compiled your contract outside of Remix, you can upload the ABI as a .abi file to the Remix IDE.
Then under the deployment tab you must provide an address for a deployed instance of the contract.

### VSCode (Visual Studio Code)

#### Installation
Install an extension for Solidity like this one developed by Juan Blanco:
https://github.com/juanfranblanco/vscode-solidity

#### Compilation
1. Go to the 'View' tab and select 'Command Palette' **or** type CMD+SHIFT+P
2. Type in `> Solidity` and you will see the commands
3. Select the command for compiling
4. A **bin/** folder will be generated containing:
    - **.abi** file with the contract ABI array
    - **.bin** file with the contract's bytecode
    - **.json** file with contract ABI array, bytecode, runtimeBytecode, and other metadata

#### Deployment
For .NET developers: https://nethereum.readthedocs.io/en/latest/getting-started/

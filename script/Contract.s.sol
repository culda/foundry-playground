// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/Contract.sol";

contract ContractScript is Script {
    function run() external {
        vm.startBroadcast();

        Contract c = new Contract(10**18);
        uint256 x = c.getX();

        vm.stopBroadcast();
    }
}

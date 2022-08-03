// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Contract {
    uint256 _x;

    constructor(uint256 x) {
        _x = x;
    }

    function getX() external view returns (uint256) {
        return _x;
    }

    function updateX(uint256 x) external {
        _x = x;
    }
}

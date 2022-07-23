pragma solidity ^0.8.15;

// import 'v3-periphery/interfaces/ISwapRouter.sol';
import 'v3-core-interfaces/callback/IUniswapV3FlashCallback.sol';
import 'v3-periphery/base/PeripheryPayments.sol';

contract PairFlash is IUniswapV3FlashCallback, PeripheryPayments {
    using LowGasSafeMath for uint256;
    using LowGasSafeMath for int256;

    // ISwapRouter public Immutable swapRouter;

}
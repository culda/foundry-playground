pragma solidity =0.7.6;

// import 'v3-periphery/interfaces/ISwapRouter.sol';
import 'v3-core/contracts/interfaces/callback/IUniswapV3FlashCallback.sol';
import 'v3-periphery/base/PeripheryPayments.sol';
import 'openzeppelin-contracts/contracts/token/ERC20/IERC20.sol';

contract PairFlash is IUniswapV3FlashCallback, PeripheryPayments {
    using LowGasSafeMath for uint256;
    using LowGasSafeMath for int256;

    // ISwapRouter public Immutable swapRouter;

}
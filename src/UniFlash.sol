import '@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';

contract PairFlash is IUniswapV3FlashCallback, PeripheryPayments {
    using LowGasSafeMath for uint256;
    using LowGasSafeMath for int256;

    // ISwapRouter public Immutable swapRouter;

}
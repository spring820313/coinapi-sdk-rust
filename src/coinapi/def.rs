
const kCoinSymbolBtc: &'static str = "BTC";
const kCoinSymbolEth: &'static str = "ETH";
const kCoinSymbolWicc: &'static str = "WICC";

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum CoinType {
    kCoinTypeBTC = 1,
    kCoinTypeETH,
    kCoinTypeWICC,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum NetworkType {
    kNetworkTypeMain = 1,
    kNetworkTypeTest,
    kNetworkTypeRegTest,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum TxType {
    kTxTypeNone = 0,
    kTxTypeLbtcRegister,
    kTxTypeLbtcVote,
    kTxTypeLbtcCancelVote,
    kTxTypeQtumTokenTransfer,
    kTxTypeWiccRegisterAccount,
    kTxTypeWiccCommon,
    kTxTypeWiccTransferSpc,
    kTxTypeWiccBet
}
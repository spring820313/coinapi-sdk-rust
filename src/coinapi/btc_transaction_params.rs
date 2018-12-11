extern crate libc;
use coinapi::str_t::str_t;
use coinapi::bigint::bigint_t;
use coinapi::array_t::array_t;
use coinapi::btseed::btseed;

#[repr(C)]
pub struct btc_transaction_params
{
    pub seed: *mut btseed,
    pub fromAddress: *mut str_t,
    pub sendAddress: *mut str_t,
    pub sendValue: *mut bigint_t,
    pub feePerKb: *mut bigint_t,
    pub password: *mut str_t,
    pub priKey: *mut str_t,
    pub recipientsPayFees: libc::c_int,
    pub btcvoutFormList: *mut array_t,
    pub txType: libc::c_int,
    pub data: *mut array_t,
}
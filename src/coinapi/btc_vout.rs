extern crate libc;
use coinapi::str_t::str_t;
use coinapi::bigint::bigint_t;

#[repr(C)]
pub struct btc_vout
{
    pub hash: *mut str_t,
    pub value: *mut bigint_t,
    pub n: libc::c_int,
    pub coinBase: libc::c_int,
}
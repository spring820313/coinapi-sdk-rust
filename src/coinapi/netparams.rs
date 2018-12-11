extern crate libc;

use coinapi::str_t::str_t;
use coinapi::def::CoinType;
use coinapi::def::NetworkType;
use coinapi::keypath::KeyPath;

#[repr(C)]
pub struct NetParams
{
    pub symbol : *mut str_t,
    pub coinType : CoinType,
    pub nettype : NetworkType,
    pub keyPath : KeyPath,
    pub version : libc::c_uint,
    pub HDprivate : libc::c_uint,
    pub HDpublic : libc::c_uint,
    pub P2KH : libc::c_uint,
    pub P2SH : libc::c_uint,
    pub keyprefixes : libc::c_uchar,
    pub ApiVersion : libc::c_ushort,
    pub N : libc::c_uint,
    pub R : libc::c_uint,
    pub P : libc::c_uint,
}
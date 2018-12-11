extern crate libc;

use coinapi::btseed::btseed;
use coinapi::str_t::str_t;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btwallet
{
    pub btSeed : *mut btseed,
    pub pubkey: *mut str_t,
    pub address: *mut str_t,
    pub symbol: *mut str_t,
}

impl btwallet {
    pub fn new() -> btwallet {
        btwallet{
            btSeed: ptr::null_mut(),
            pubkey: ptr::null_mut(),
            address: ptr::null_mut(),
            symbol: ptr::null_mut(),
        }
    }
}
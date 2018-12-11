extern crate libc;

use coinapi::array_t::array_t;
use coinapi::str_t::str_t;
use coinapi::encrypted_data::encrypted_data;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btseed
{
    pub seed : *mut array_t,
    pub mnemonicCode : *mut array_t,
    pub encryptedMnemonicCode: encrypted_data,
    pub encryptedSeed: encrypted_data,
    pub creationTimeSeconds: libc::c_long,
    pub pwdhash: *mut str_t,
    pub randomSalt: *mut array_t,
}
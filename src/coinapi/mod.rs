extern crate libc;

pub mod def;
pub mod str_t;
pub mod array_t;
pub mod encrypted_data;
pub mod map_t;
pub mod obj_t;
pub mod value;
pub mod bigint;
pub mod keypath;
pub mod netparams;
pub mod btseed;
pub mod btwallet;
pub mod btc_vout;
pub mod btc_transaction_params;

#[link(name = "coinapi-nojni")]
extern {
    pub fn init() -> *mut libc::c_void;
    pub fn cleanup(h : *mut libc::c_void) -> libc::c_void;
    pub fn createAllCoinMnemonicCode(h : *mut libc::c_void) -> *mut libc::c_char;
    pub fn checkMnemonicCode(h : *mut libc::c_void, w: *mut libc::c_char) -> libc::c_int;
    pub fn str_create(str : *const libc::c_char, size : libc::c_uint, capacity : libc::c_uint) -> *mut str_t::str_t;
    pub fn str_unref(s : *mut str_t::str_t) -> *mut str_t::str_t;
    pub fn createWallet(h : *mut libc::c_void, words: *mut libc::c_char, password: *const libc::c_char, netParams: *mut netparams::NetParams, bw: *mut btwallet::btwallet) -> libc::c_int;
    pub fn createSignTransaction(h : *mut libc::c_void, signParams : *mut libc::c_void, netParams: *mut netparams::NetParams) -> *mut map_t::map_t;
    pub fn bigint_to_string(a: *mut bigint::bigint_t, base: libc::c_uint, dest: *mut libc::c_char) -> libc::c_void;
    pub fn bigint_from_string(a: *mut libc::c_char, base: libc::c_uint) -> *mut bigint::bigint_t;
    pub fn array_create() -> *mut array_t::array_t;
    pub fn array_append(array: *mut array_t::array_t, data: value::value_t) -> libc::c_int;
    pub fn value_set_pointer(v: *mut value::value_t, value: *mut libc::c_void, destroy: value::destroy_t) ->*mut value::value_t;
    pub fn btc_vout_free(bv: *mut btc_vout::btc_vout) ->libc::c_void;
    pub fn value_pointer(v: value::value_t) ->*mut libc::c_void;
    pub fn value_str(v: value::value_t) -> *mut str_t::str_t;
}
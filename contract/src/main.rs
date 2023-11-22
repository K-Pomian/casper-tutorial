#![no_std]
#![no_main]

// This code imports necessary aspects of external crates that we will use in our contract code.
extern crate alloc;

// Importing Rust types.
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
// Importing aspects of the Casper platform.
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
// Importing specific Casper types.
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue,
};

// Creating constants for values within the contract package.
const CONTRACT_PACKAGE_NAME: &str = "kpomian_counter_package_name";
const CONTRACT_ACCESS_UREF: &str = "kpomian_counter_access_uref";

// Creating constants for the various contract entry points.
const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
const ENTRY_POINT_COUNTER_GET: &str = "counter_get";

// Creating constants for values within the contract.
const CONTRACT_VERSION_KEY: &str = "version";
const CONTRACT_KEY: &str = "counter";
const COUNT_KEY: &str = "count";

#[no_mangle]
pub extern "C" fn counter_get() {
    let uref = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let value = storage::read::<i32>(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let value_typed = CLValue::from_t(value).unwrap_or_revert_with(ApiError::CLTypeMismatch);

    runtime::ret(value_typed);
}

#[no_mangle]
pub extern "C" fn counter_inc() {
    let uref = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    storage::add(uref, 1);
}

#[no_mangle]
pub extern "C" fn counter_dec() {
    let uref = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);

    storage::add(uref, -1);
}

#[no_mangle]
pub extern "C" fn call() {
    let mut named_keys = NamedKeys::new();

    let counter = storage::new_uref(0_i64);
    let counter_key_name = String::from(COUNT_KEY);
    named_keys.insert(counter_key_name, counter.into());

    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_GET,
        Vec::new(),
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_INC,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (stored_contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(CONTRACT_PACKAGE_NAME.to_string()),
        Some(CONTRACT_ACCESS_UREF.to_string()),
    );

    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}

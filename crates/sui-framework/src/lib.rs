// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_binary_format::CompiledModule;
use move_core_types::gas_algebra::InternalGas;
use once_cell::sync::Lazy;
use std::path::Path;
use sui_framework_build::compiled_package::{BuildConfig, CompiledPackage};
use sui_types::{
    base_types::ObjectID,
    digests::TransactionDigest,
    error::SuiResult,
    move_package::MovePackage,
    object::{Object, OBJECT_START_VERSION},
    MOVE_STDLIB_OBJECT_ID,
};

pub mod natives;

static SUI_FRAMEWORK_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sui-framework"));
static SUI_FRAMEWORK: Lazy<Vec<CompiledModule>> = Lazy::new(|| {
    // deserialization here uses overall max version of supported Move bytecode rather than the max
    // supported by the framework but it's OK
    get_sui_framework_bytes()
        .into_iter()
        .map(|module| CompiledModule::deserialize(&module).unwrap())
        .collect()
});

static SUI_FRAMEWORK_TEST: Lazy<Vec<CompiledModule>> = Lazy::new(|| {
    const SUI_FRAMEWORK_TEST_BYTES: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/sui-framework-test"));

    let serialized_modules: Vec<Vec<u8>> = bcs::from_bytes(SUI_FRAMEWORK_TEST_BYTES).unwrap();

    // deserialization here uses overall max version of supported Move bytecode rather than the max
    // supported by the framework but it's OK
    serialized_modules
        .into_iter()
        .map(|module| CompiledModule::deserialize(&module).unwrap())
        .collect()
});

static MOVE_STDLIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/move-stdlib"));
static MOVE_STDLIB: Lazy<Vec<CompiledModule>> = Lazy::new(|| {
    // deserialization here uses overall max version of supported Move bytecode rather than the max
    // supported by the framework but it's OK
    get_move_stdlib_bytes()
        .into_iter()
        .map(|module| CompiledModule::deserialize(&module).unwrap())
        .collect()
});

static MOVE_STDLIB_TEST: Lazy<Vec<CompiledModule>> = Lazy::new(|| {
    const MOVE_STDLIB_TEST_BYTES: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/move-stdlib-test"));

    let serialized_modules: Vec<Vec<u8>> = bcs::from_bytes(MOVE_STDLIB_TEST_BYTES).unwrap();

    // deserialization here uses overall max version of supported Move bytecode rather than the max
    // supported by the framework but it's OK
    serialized_modules
        .into_iter()
        .map(|module| CompiledModule::deserialize(&module).unwrap())
        .collect()
});

pub fn get_sui_framework() -> Vec<CompiledModule> {
    Lazy::force(&SUI_FRAMEWORK).to_owned()
}

pub fn get_sui_framework_bytes() -> Vec<Vec<u8>> {
    bcs::from_bytes(SUI_FRAMEWORK_BYTES).unwrap()
}

pub fn get_sui_framework_transitive_dependencies() -> Vec<ObjectID> {
    vec![MOVE_STDLIB_OBJECT_ID]
}

pub fn get_sui_framework_package() -> MovePackage {
    MovePackage::new_system(
        OBJECT_START_VERSION,
        get_sui_framework(),
        get_sui_framework_transitive_dependencies(),
    )
}

pub fn get_sui_framework_object() -> Object {
    Object::new_system_package(
        get_sui_framework(),
        OBJECT_START_VERSION,
        get_sui_framework_transitive_dependencies(),
        TransactionDigest::genesis(),
    )
}

pub fn get_sui_framework_test() -> Vec<CompiledModule> {
    Lazy::force(&SUI_FRAMEWORK_TEST).to_owned()
}

pub fn get_move_stdlib() -> Vec<CompiledModule> {
    Lazy::force(&MOVE_STDLIB).to_owned()
}

pub fn get_move_stdlib_bytes() -> Vec<Vec<u8>> {
    bcs::from_bytes(MOVE_STDLIB_BYTES).unwrap()
}

pub fn get_move_stdlib_transitive_dependencies() -> Vec<ObjectID> {
    vec![]
}

pub fn get_move_stdlib_package() -> MovePackage {
    MovePackage::new_system(
        OBJECT_START_VERSION,
        get_move_stdlib(),
        get_move_stdlib_transitive_dependencies(),
    )
}

pub fn get_move_stdlib_object() -> Object {
    Object::new_system_package(
        get_move_stdlib(),
        OBJECT_START_VERSION,
        get_move_stdlib_transitive_dependencies(),
        TransactionDigest::genesis(),
    )
}

pub fn get_move_stdlib_test() -> Vec<CompiledModule> {
    Lazy::force(&MOVE_STDLIB_TEST).to_owned()
}

pub const DEFAULT_FRAMEWORK_PATH: &str = env!("CARGO_MANIFEST_DIR");

// TODO: remove these in favor of new costs
pub fn legacy_test_cost() -> InternalGas {
    InternalGas::new(0)
}

pub fn legacy_emit_cost() -> InternalGas {
    InternalGas::new(52)
}

pub fn legacy_create_signer_cost() -> InternalGas {
    InternalGas::new(24)
}

pub fn legacy_empty_cost() -> InternalGas {
    InternalGas::new(84)
}

pub fn legacy_length_cost() -> InternalGas {
    InternalGas::new(98)
}

/// Wrapper of the build command that verifies the framework version. Should eventually be removed once we can
/// do this in the obvious way (via version checks)
pub fn build_move_package(path: &Path, config: BuildConfig) -> SuiResult<CompiledPackage> {
    //let test_mode = config.config.test_mode;
    let pkg = config.build(path.to_path_buf())?;
    /*if test_mode {
        pkg.verify_framework_version(get_sui_framework_test(), get_move_stdlib_test())?;
    } else {
        pkg.verify_framework_version(get_sui_framework(), get_move_stdlib())?;
    }*/
    Ok(pkg)
}

pub fn make_system_modules() -> Vec<Vec<CompiledModule>> {
    vec![get_move_stdlib(), get_sui_framework()]
}

pub fn make_system_packages() -> Vec<MovePackage> {
    vec![get_move_stdlib_package(), get_sui_framework_package()]
}

pub fn make_system_objects() -> Vec<Object> {
    vec![get_move_stdlib_object(), get_sui_framework_object()]
}

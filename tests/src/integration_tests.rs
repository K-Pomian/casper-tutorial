fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        PRODUCTION_RUN_GENESIS_REQUEST,
    };
    use casper_types::{runtime_args, ContractHash, RuntimeArgs};

    const COUNTER_WASM: &str = "contract.wasm";

    const CONTRACT_KEY: &str = "counter";
    const COUNT_KEY: &str = "count";
    const CONTRACT_VERSION_KEY: &str = "version";

    const ENTRY_POINT_COUNTER_GET: &str = "counter_get";
    const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
    const ENTRY_POINT_COUNTER_DEC: &str = "counter_dec";
    const ENTRY_POINT_COUNTER_RESET: &str = "counter_reset";

    fn install_system_contracts() -> InMemoryWasmTestBuilder {
        let mut builder = InMemoryWasmTestBuilder::default();

        builder
            .run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        builder
    }

    fn install_counter_contract(builder: &mut InMemoryWasmTestBuilder) -> ContractHash {
        let contract_installation_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, COUNTER_WASM, runtime_args! {})
                .build();
        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect(format!("Value for key {} not found.", CONTRACT_KEY).as_str())
            .into_hash()
            .map(ContractHash::new)
            .expect("The hash is not a contract hash.")
    }

    #[test]
    fn test_contract_deploy() {
        let mut builder = install_system_contracts();
        let contract_hash = install_counter_contract(&mut builder);

        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let version_key = *account
            .named_keys()
            .get(CONTRACT_VERSION_KEY)
            .expect("Version uref should exist.");
        let version = builder
            .query(None, version_key, &[])
            .expect(
                format!(
                    "{} key should have associated stored value",
                    CONTRACT_VERSION_KEY
                )
                .as_str(),
            )
            .as_cl_value()
            .expect(format!("{} is not associated with CL value", CONTRACT_VERSION_KEY).as_str())
            .clone()
            .into_t::<u32>()
            .expect("Stored value should be i32");

        assert_eq!(version, 1);

        let contract = builder
            .get_contract(contract_hash)
            .expect("The contract should exist");

        assert!(contract.has_entry_point(ENTRY_POINT_COUNTER_INC));
        assert!(contract.has_entry_point(ENTRY_POINT_COUNTER_GET));
        assert!(contract.has_entry_point(ENTRY_POINT_COUNTER_DEC));
        assert!(contract.has_entry_point(ENTRY_POINT_COUNTER_RESET));
    }

    #[test]
    fn test_counter_inc() {
        let mut builder = install_system_contracts();
        let contract_hash = install_counter_contract(&mut builder);
        let contract = builder.get_contract(contract_hash).unwrap();
        let count_key = *contract
            .named_keys()
            .get(COUNT_KEY)
            .expect("Count uref should exist");

        let count_before = builder
            .query(None, count_key, &[])
            .expect(format!("No stored value found associated with {} key.", COUNT_KEY).as_str())
            .as_cl_value()
            .expect(format!("{} is not associated with CL value", COUNT_KEY).as_str())
            .clone()
            .into_t::<i64>()
            .expect("Stored value should be i64");

        let increment_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            ENTRY_POINT_COUNTER_INC,
            runtime_args! {},
        )
        .build();
        builder.exec(increment_request).expect_success().commit();

        let count_after = builder
            .query(None, count_key, &[])
            .expect(format!("No stored value found associated with {} key.", COUNT_KEY).as_str())
            .as_cl_value()
            .expect(format!("{} is not associated with CL value", COUNT_KEY).as_str())
            .clone()
            .into_t::<i64>()
            .expect("Stored value should be i64");

        assert!(count_after - count_before == 1);
    }

    #[test]
    fn test_counter_dec() {
        let mut builder = install_system_contracts();
        let contract_hash = install_counter_contract(&mut builder);
        let contract = builder.get_contract(contract_hash).unwrap();

        let count_key = *contract
            .named_keys()
            .get(COUNT_KEY)
            .expect("Count uref should exist");
        let count_before = builder
            .query(None, count_key, &[])
            .expect(format!("No stored value found associated with {} key.", COUNT_KEY).as_str())
            .as_cl_value()
            .expect(format!("{} is not associated with CL value", COUNT_KEY).as_str())
            .clone()
            .into_t::<i64>()
            .expect("Stored value should be i64");

        let counter_dec_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            ENTRY_POINT_COUNTER_DEC,
            runtime_args! {},
        )
        .build();
        builder.exec(counter_dec_request).expect_success().commit();

        let count_after = builder
            .query(None, count_key, &[])
            .expect(format!("No stored value found associated with {} key.", COUNT_KEY).as_str())
            .as_cl_value()
            .expect(format!("{} is not associated with CL value", COUNT_KEY).as_str())
            .clone()
            .into_t::<i64>()
            .expect("Stored value should be i64");

        assert!(count_after - count_before == -1);
    }

    #[test]
    fn test_counter_reset() {
        let mut builder = install_system_contracts();
        let contract_hash = install_counter_contract(&mut builder);
        let contract = builder.get_contract(contract_hash).unwrap();

        let increment_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            ENTRY_POINT_COUNTER_INC,
            runtime_args! {},
        )
        .build();
        builder.exec(increment_request).expect_success().commit();

        let reset_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            ENTRY_POINT_COUNTER_RESET,
            runtime_args! {},
        )
        .build();
        builder.exec(reset_request).expect_success().commit();

        let count_key = *contract
            .named_keys()
            .get(COUNT_KEY)
            .expect("Count uref should exist");
        let count = builder
            .query(None, count_key, &[])
            .expect(format!("No stored value found associated with {} key.", COUNT_KEY).as_str())
            .as_cl_value()
            .expect(format!("{} is not associated with CL value", COUNT_KEY).as_str())
            .clone()
            .into_t::<i64>()
            .expect("Stored value should be i64");
        assert_eq!(count, 0);
    }
}

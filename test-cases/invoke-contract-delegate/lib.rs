#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod invoke_contract_delegate {
    use ink::env::call::build_call;
    #[ink(storage)]
    pub struct InvokeContractDelegate {}

    impl InvokeContractDelegate {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn delegate_call(&self, code_hash: Hash) -> u128 {
            build_call::<ink::env::DefaultEnvironment>()
                .delegate(code_hash)
                .exec_input(ink::env::call::ExecutionInput::new(
                    ink::env::call::Selector::new(ink::selector_bytes!("get_value")),
                ))
                .returns::<u128>()
                .invoke()
        }
    }

    impl Default for InvokeContractDelegate {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn delegate_contract_works() {
            let contract = InvokeContractDelegate::new();
            let code_hash = Hash::from([0x42; 32]);
            let res = contract.delegate_call(code_hash);
            assert_eq!(res, 25);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test(additional_contracts = "contract-to-call/Cargo.toml")]
        async fn invoke_contract_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let original_contract_contructor = InvokeContractDelegateRef::new();

            // When

            let original_contract_acc_id = client
                .instantiate(
                    "invoke-contract-delegate",
                    &ink_e2e::alice(),
                    original_contract_contructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let contract_to_call_acc_id = client
                .upload("contract-to-call", &ink_e2e::alice(), None)
                .await
                .expect("instantiate failed")
                .code_hash;

            // Then
            let delegate_call =
                build_message::<InvokeContractDelegateRef>(original_contract_acc_id)
                    .call(|contract| contract.delegate_call(contract_to_call_acc_id));
            let profit_res = client
                .call(&ink_e2e::alice(), delegate_call, 0, None)
                .await
                .expect("call failed");

            println!("profit_res: {:?}", profit_res.return_value());
            Ok(())
        }
    }
}

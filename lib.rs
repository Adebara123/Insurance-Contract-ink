#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod insure {

    use ink::prelude::vec;
    use ink::storage::Mapping;
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RiskLevel {
        VeryLow,
        Low,
        Medium,
        High,
        VeryHigh,
    }

    pub struct RiskAssessor {
        total_cover_provided: Balance,
        inital_cover_creation_date: Balance,
        last_withdrawal: Balance,
    }

    pub struct Users {
        total_cover_bought: Balance,
        cover_paid_for: Balance,
        date_bought: Balance,
        requested_cover: bool,
    }

    pub struct Protocol {
        id: u64,
        total_cover: Balance,
        cover_left: Balance,
        total_cover_paid: Balance,
        protocol_name: String,
        first_risk_provider: AccountId,
        domain_name: String,
        description: String,
        risk_level: RiskLevel,
        current_users: Vec<AccountId>,
        users_data: Mapping<AccountId, Users>,
        risk_assessors: Maping<AccountId, RiskAssessor>,
    }

    pub struct ProtocolData {
        total_cover: Balance,
        cover_left: Balance,
        total_cover_paid: Balance,
        protocol_name: String,
        domain_name: String,
        description: String,
        risk_level: RiskLevel,
    }



    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Insurance {
        admin: AccountId,
        id: u64,
        token_address: AccountId, // Change it to token address
        protocol_fee: u8,
        total_fee_gotten: Balance,
        governance_fee: u8,
        governance_address: AccountId,
        very_low: u8,
        low: u8,
        medium: u8,
        high: u8,
        very_high: u8,
        percentage: u16,
        year: u16,
        all_protocols: Mapping<u64, Protocol>

    }


    #[ink(event)]
    pub struct NewInsure {
        #[ink(topic)]
        protocol_name: String,
        protocol_domain: String,
        total_cover_created: Balance,
    }

    impl Insurance {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(token_address: AccountId, governance_address: AccountId) -> Self {
            Self { 
                admin: self.env().caller(),
                id: 1,
                token_address,
                protocol_fee: 2,
                total_fee_gotten: 0,
                governance_fee: 3,
                governance_address,
                very_low: 0,
                low: 20,
                medium: 40,
                high: 60,
                very_high: 80,
                percentage: 1000,
                year: 365, 
                all_protocols: Mapping::default(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn create_new_insurance(&mut self) {
            
        }

        /// Simply retu/// Constructor that initializes the `bool` value to `false`.
        // ///
        // /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }rns the current value of our `bool`.
        #[ink(message)]
        pub fn createOnExistinginsure(&mut self) {
          
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GovernanceRef::default();

            // When
            let contract_account_id = client
                .instantiate("governance", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<GovernanceRef>(contract_account_id.clone())
                .call(|governance| governance.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GovernanceRef::new(false);
            let contract_account_id = client
                .instantiate("governance", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<GovernanceRef>(contract_account_id.clone())
                .call(|governance| governance.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<GovernanceRef>(contract_account_id.clone())
                .call(|governance| governance.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<GovernanceRef>(contract_account_id.clone())
                .call(|governance| governance.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}

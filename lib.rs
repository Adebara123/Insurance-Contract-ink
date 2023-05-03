#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod insure {

    // =====================================
    // MAKING IMPORTATIONS 
    // =====================================
    use openbrush::{
        contracts::{
            traits::psp22::PSP22Ref,
        },
    }; // this would be used for psp22 token interaction 
    use ink::prelude::vec;
    use ink::storage::Mapping;
    use ink::env::CallFlags;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]


    pub enum Error {
        NotAdmin,
        AmountShouldBeGreaterThanZero,
        InsufficientFunds,
        NotEnoughAllowance,
        TokenTransferFailed,
        Overflow
    }


    pub enum RiskLevel {
        VeryLow,
        Low,
        Medium,
        High,
        VeryHigh,
    }

    // ===================================
    // EVENTS DECLARATION 
    // ===================================

    #[ink(event)]
    pub struct NewInsure {
        #[ink(topic)]
        protocol_name: String,
        protocol_domain: String,
        total_cover_created: Balance,
        creator_address: AccountId,
        risk_level: RiskLevel,
        creation_time: Balance,
        protocol_id: u64,
    }

    #[ink(event)] 
    pub struct AddOnExistingInsure {
        #[ink(topic)]
        protocol_name: String,
        protocol_domain: String,
        cover_added: Balance,
        creator_address: AccountId,
        creation_time: Balance,
    }

    #[ink(event)] 
    pub struct CoverBought {
        #[ink(topic)]
        protocol: String,
        total_cover_bought: Balance,
        amount_paid: Balance,
        total_period: Balance,
        risk_level: RiskLevel,
    }


    // ===================================
    // STRUCTURES 
    // ===================================


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

    #[ink(impl)]
    impl Insurance {
         // ================================================
        // MODIFIERS, AUTHORIZATION GATE AND SANITY CHECKS 
        // ================================================
        
        fn only_owner(&self) -> Result<(), Error> {
            if self.env().caller() == self.admin {
                Ok(())
            } else {
                Err(Error::NotAdmin)
            }
        }

        fn zero_address(&self) -> AccountId {
            [0u8; 32].into()
        }

        fn transfer(
            &self,
            to: AccountId,
            token: AccountId,
            amount: Balance
        ) -> Result<(), Error> { 
            PSP22Ref::transfer(
                &token,
                to,
                amount,
                vec![])
                    .unwrap_or_else(|error| {
                    panic!(
                        "Failed to transfer PSP22 2 tokens to caller : {:?}",
                        error
                    )
            });

            Ok(())
        }


        fn transfer_from(
            &self,
            from: AccountId,
            to: AccountId,
            token: AccountId,
            amount: Balance
        ) -> Result<(), Error> { 
            // checking the balance of the sender to see if the sender has enough balance to run this transfer 
            let user_current_balance = PSP22Ref::balance_of(
                &token,
                from
            );

            if user_current_balance < amount {
                return Err(Error::InsufficientFunds)
            }

            // checking if enough allowance has been made for this operation 
            let staking_contract_allowance = PSP22Ref::allowance(
                &token,
                from,
                to
            );

            if staking_contract_allowance < amount {
                return Err(Error::NotEnoughAllowance)
            }

            let staking_contract_initial_balance = PSP22Ref::balance_of(
                &token,
                to
            );


            // making the transfer call to the token contract 
            if PSP22Ref::transfer_from_builder(
                &token,
                from,
                to,
                amount,
                vec![])
                    .call_flags(CallFlags::default()
                    .set_allow_reentry(true))
                    .try_invoke()
                    .expect("Transfer failed")
                    .is_err(){
                        return Err(Error::TokenTransferFailed);
            }

            let staking_contract_balance_after_transfer = PSP22Ref::balance_of(
                &token,
                to
            );

            let mut actual_token_staked:Balance = 0;
        
            // calculating the actual amount that came in to the contract, some token might have taxes, just confirming transfer for economic safety
            match staking_contract_balance_after_transfer.checked_sub(staking_contract_initial_balance) {
                Some(result) => {
                    actual_token_staked = result;
                }
                None => {
                    return Err(Error::Overflow);
                }
            };

            Ok(())
        }
    }


    #[ink(impl)]
    impl Insurance {

        // ====================================
        // MAIN IMPLEMENTATION BLOCK 
        // ====================================
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

    // ***************** //
    
    // WRITE FUNCTIONS
    
    // ***************** //

        
        #[ink(message)]
        pub fn create_new_insurance(
            &mut self,
            _protocol_name: &str,
            _protocol_domain: &str,
            _description: &str,
            total_cover_amount: Balance
            ) -> Result<(), Error>
        {
            
            }
        }


        #[ink(message)]
        pub fn createOnExistinginsure(&mut self) {
          
        }
    }


}
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod stake_voting {
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::{
        traits::{PackedLayout, SpreadAllocate, SpreadLayout},
        Mapping,
    };

    #[derive(
        Debug,
        Default,
        Clone,
        PackedLayout,
        SpreadLayout,
        scale::Decode,
        scale::Encode,
        scale_info::TypeInfo,
    )]
    pub struct StakeOperatorInformation {
        stake_operator_id: AccountId,
        name: String,
        own_staked: Balance,
        other_staked: Balance,
        commission: u8,
        total_stakers: u16,
        vote_points: Balance,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct StakeOperatorsVotes {
        list_of_stake_operators: Vec<StakeOperatorInformation>,
        stake_operator_id_to_position: Mapping<AccountId, u32>,
    }

    impl StakeOperatorsVotes {
        #[ink(constructor)]
        pub fn default() -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|_: &mut Self| {})
        }

        #[ink(message)]
        pub fn add_one_stake_operator(
            &mut self,
            stake_operator_information: StakeOperatorInformation,
        ) {
            let pos = self.list_of_stake_operators.len() as u32;
            self.stake_operator_id_to_position
                .insert(stake_operator_information.stake_operator_id.clone(), &pos);
            self.list_of_stake_operators
                .push(stake_operator_information);
        }

        #[ink(message)]
        pub fn add_multiple_stake_operators(
            &mut self,
            stake_operator_information_list: Vec<StakeOperatorInformation>,
        ) {
            let initial_pos = self.list_of_stake_operators.len() as u32;
            for (i, stake_operator_information) in
                stake_operator_information_list.into_iter().enumerate()
            {
                let pos = initial_pos + i as u32;
                self.stake_operator_id_to_position
                    .insert(stake_operator_information.stake_operator_id.clone(), &pos);
                self.list_of_stake_operators
                    .push(stake_operator_information);
            }
        }

        #[ink(message)]
        pub fn get_stake_operators(&self) -> Vec<StakeOperatorInformation> {
            self.list_of_stake_operators.clone()
        }
    }
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let stake_voting = StakeVoting::default();
    //         assert_eq!(stake_voting.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut stake_voting = StakeVoting::new(false);
    //         assert_eq!(stake_voting.get(), false);
    //         stake_voting.flip();
    //         assert_eq!(stake_voting.get(), true);
    //     }
    // }
}

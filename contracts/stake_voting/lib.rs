#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod stake_voting {
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::{
        traits::{PackedLayout, SpreadAllocate, SpreadLayout},
        Mapping,
    };

    #[derive(Debug, Default, Clone, PackedLayout, SpreadLayout, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
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
        stake_operator_id_to_voter_id: Mapping<(AccountId, AccountId), bool>,
        stake_operator_id_to_position: Mapping<AccountId, u32>,
        position_to_stake_operator_information: Mapping<u32, StakeOperatorInformation>,
        length_of_stake_operator_information_list: u32,
    }

    impl StakeOperatorsVotes {
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                // occupying 0th slot with empty element
                contract.length_of_stake_operator_information_list = 1;
                contract.stake_operator_id_to_position = Mapping::default();
                contract.stake_operator_id_to_voter_id = Mapping::default();
                contract.position_to_stake_operator_information = Mapping::default();
                contract
                    .position_to_stake_operator_information
                    .insert(0, &StakeOperatorInformation::default());
            })
        }

        #[ink(message)]
        pub fn add_vote(&mut self, stake_operator_id: AccountId) {

            // let pos = self.stake_operator_id_to_position.get(stake_operator_id);
            // if let None = pos {
            //     return;
            // }

            // let pos = pos.unwrap();
            // let mut stake_operator_information = self
            //     .position_to_stake_operator_information
            //     .get(pos)
            //     .unwrap_or_default();
            // stake_operator_information.vote_points =
            //     stake_operator_information.vote_points + ink_env::balance();

            // self.stake_operator_id_to_voter_id
            //     .insert((stake_operator_id, ink_env::caller()), &true);

            // self.length_of_stake_operator_information_list += 1;
            // self.position_to_stake_operator_information
            //     .insert(pos, &stake_operator_information);
            // self.stake_operator_id_to_position
            //     .insert(stake_operator_information.stake_operator_id, &pos);
        }

        // TODO: add vote, add get_caller_already_voted_for_list_of_stake_operator_ids

        #[ink(message)]
        pub fn add_one_stake_operator(
            &mut self,
            stake_operator_information: StakeOperatorInformation,
        ) {
            let pos = self.length_of_stake_operator_information_list;
            self.length_of_stake_operator_information_list += 1;
            self.position_to_stake_operator_information
                .insert(pos, &stake_operator_information);
            self.stake_operator_id_to_position
                .insert(stake_operator_information.stake_operator_id, &pos);
        }

        #[ink(message)]
        pub fn add_multiple_stake_operators(
            &mut self,
            stake_operator_information_list: Vec<StakeOperatorInformation>,
        ) {
            let initial_pos = self.length_of_stake_operator_information_list;
            self.length_of_stake_operator_information_list +=
                stake_operator_information_list.len() as u32;

            for (i, stake_operator_information) in
                stake_operator_information_list.into_iter().enumerate()
            {
                let pos = initial_pos + i as u32;
                self.position_to_stake_operator_information
                    .insert(pos, &stake_operator_information);
                self.stake_operator_id_to_position
                    .insert(stake_operator_information.stake_operator_id, &pos);
            }
        }

        #[ink(message)]
        pub fn get_stake_operators(&self) -> Vec<StakeOperatorInformation> {
            let mut output = Vec::new();
            for i in 1..self.length_of_stake_operator_information_list {
                output.push(
                    self.position_to_stake_operator_information
                        .get(i)
                        .unwrap_or_default(),
                );
            }

            output
        }

        #[ink(message)]
        pub fn get_stake_operator_by_account_id(
            &self,
            stake_operator_id: AccountId,
        ) -> StakeOperatorInformation {
            let pos = self
                .stake_operator_id_to_position
                .get(stake_operator_id)
                .unwrap_or_default();

            self.position_to_stake_operator_information
                .get(pos)
                .unwrap_or_default()
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

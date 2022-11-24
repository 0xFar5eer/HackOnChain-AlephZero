#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod stake_voting {
    use ink_prelude::{string::String, vec, vec::Vec};
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
        pub fn initialize(&mut self) {
            // adding testnet validators list
            let addrs: Vec<[u8; 32]> = vec![
                "5Eo5ZxVUGbT6D8cfAvAxQFhzt3ZqBEb5oE8KCWR9vjvTPSMy",
                "5GW5kbpuYn8Wa2253xLNLn9dZYWJUPJmW7VwmjnziDWdGxiX",
                "5CGTtuqDbBQokPQjpa4mQyNKyvYxKpgtZEskDkJxzho1NhbM",
                "5HYzfrjAMGB6zWW3oTg7dhGdWB8cawyU84fCpGar9QhupweS",
                "5Dkh7kuPm4NMfkmDG1LaVZVWXW3WHYwh7BKEFfNvPiGDrARH",
                "5HNnDD5djTaiUt3A6yf6f1E9oDiM5w5fcNBTLLCoMKf1TEdS",
                "5Grh6bLQmoxinEeiijAfSbGYrYiKhxnxcM2m96s5A64VyAiF",
                "5DATX2UZZgxAsumbVEsmup2q6LR9Bn81F7KW7PsShgUw8t12",
                "5FnyjESMB4EBQn1W1vnNKZ5oVUYUmQbTPG4hZbJJm8697TKt",
                "5GN3rbR41UYWtjoxeuyvBfWEPopH4C2R4z7qhtz2ysF5hmrt",
            ]
            .into_iter()
            .map(|a| a.as_bytes().try_into().unwrap())
            .collect::<Vec<_>>();

            let stake_operators = vec![
                StakeOperatorInformation {
                    name: String::from("AZF/SHANNON"),
                    commission: 0,
                    other_staked: 3_218_000,
                    own_staked: 642_722,
                    total_stakers: 618,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[0]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/SIERPINSKI"),
                    commission: 0,
                    other_staked: 334_565,
                    own_staked: 73_606,
                    total_stakers: 509,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[1]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/CANTOR"),
                    commission: 0,
                    other_staked: 567_941,
                    own_staked: 229_798,
                    total_stakers: 515,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[2]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/HILBERT"),
                    commission: 0,
                    other_staked: 303_097,
                    own_staked: 140_053,
                    total_stakers: 515,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[3]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/FERMAT"),
                    commission: 0,
                    other_staked: 498_871,
                    own_staked: 142_356,
                    total_stakers: 511,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[4]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/RAMANUJAN"),
                    commission: 0,
                    other_staked: 699_972,
                    own_staked: 300_787,
                    total_stakers: 515,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[5]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/GALOIS"),
                    commission: 0,
                    other_staked: 243_081,
                    own_staked: 73_132,
                    total_stakers: 504,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[6]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/RIEMANN"),
                    commission: 0,
                    other_staked: 202_892,
                    own_staked: 119_877,
                    total_stakers: 517,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[7]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/CAUCHY"),
                    commission: 0,
                    other_staked: 202_892,
                    own_staked: 50_355,
                    total_stakers: 500,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[8]),
                },
                StakeOperatorInformation {
                    name: String::from("AZF/LAPLACE"),
                    commission: 0,
                    other_staked: 859_746,
                    own_staked: 224_547,
                    total_stakers: 502,
                    vote_points: 0,
                    stake_operator_id: AccountId::from(addrs[9]),
                },
            ];

            self.length_of_stake_operator_information_list += stake_operators.len() as u32;
            for (i, stake_operator_information) in stake_operators.into_iter().enumerate() {
                let pos = i as u32 + 1;
                self.stake_operator_id_to_position
                    .insert(stake_operator_information.stake_operator_id, &pos);
                self.position_to_stake_operator_information
                    .insert(pos, &stake_operator_information);
            }
        }

        #[ink(message)]
        pub fn add_vote(&mut self, stake_operator_id: AccountId) {
            let pos = self.stake_operator_id_to_position.get(stake_operator_id);
            if let None = pos {
                return;
            }

            // check if already voted
            let voted = self
                .stake_operator_id_to_voter_id
                .get((stake_operator_id, Self::env().caller()));
            if let Some(voted) = voted {
                if voted {
                    return;
                }
            }

            let pos = pos.unwrap();
            let mut stake_operator_information = self
                .position_to_stake_operator_information
                .get(pos)
                .unwrap_or_default();
            stake_operator_information.vote_points = stake_operator_information.vote_points + 1;

            self.stake_operator_id_to_voter_id
                .insert((stake_operator_id, Self::env().caller()), &true);
        }

        #[ink(message)]
        pub fn get_stake_operator_ids_already_voted_for(&self) -> Vec<AccountId> {
            let mut output = Vec::new();
            let caller = Self::env().caller();
            for i in 1..self.length_of_stake_operator_information_list {
                let stake_operator_information = self
                    .position_to_stake_operator_information
                    .get(i)
                    .unwrap_or_default();
                let voter = self
                    .stake_operator_id_to_voter_id
                    .get((stake_operator_information.stake_operator_id, caller));
                if let None = voter {
                    continue;
                }

                let voter = voter.unwrap();
                if voter {
                    output.push(stake_operator_information.stake_operator_id);
                }
            }

            output
        }

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

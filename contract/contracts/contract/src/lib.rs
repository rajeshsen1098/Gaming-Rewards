#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Map};

#[contract]
pub struct GamingRewards;

#[contractimpl]
impl GamingRewards {
    // Store rewards for a player
    pub fn reward_player(env: Env, player: Address, amount: i128) {
        player.require_auth();

        let key = symbol_short!("reward");

        let mut rewards: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        let current = rewards.get(player.clone()).unwrap_or(0);
        rewards.set(player.clone(), current + amount);

        env.storage().instance().set(&key, &rewards);
    }

    // Check reward balance
    pub fn get_reward(env: Env, player: Address) -> i128 {
        let key = symbol_short!("reward");

        let rewards: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        rewards.get(player).unwrap_or(0)
    }

    // Claim rewards
    pub fn claim_reward(env: Env, player: Address) -> i128 {
        player.require_auth();

        let key = symbol_short!("reward");

        let mut rewards: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        let amount = rewards.get(player.clone()).unwrap_or(0);

        rewards.set(player.clone(), 0);
        env.storage().instance().set(&key, &rewards);

        amount
    }
}
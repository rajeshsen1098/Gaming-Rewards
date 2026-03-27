<img width="1536" height="1024" alt="ChatGPT Image Mar 27, 2026, 01_57_20 PM" src="https://github.com/user-attachments/assets/2721adca-2ece-4988-875f-2b045ab26dea" />

# 🎮 Gaming Rewards – Soroban Smart Contract (Stellar)

---
##DEMO
<img width="1908" height="872" alt="Screenshot 2026-03-27 132916" src="https://github.com/user-attachments/assets/db9e255f-8ce5-40c4-b554-9f0f7983d716" />
<img width="1906" height="851" alt="Screenshot 2026-03-27 134132" src="https://github.com/user-attachments/assets/44617041-2ae8-4e08-adc9-4bba5ffbe00d" />


---

## 🚀 Overview

Gaming Rewards is a decentralized reward distribution system built on **Soroban (Stellar Smart Contracts)**. It enables games to reward players transparently and securely based on achievements, scores, or milestones.

This project demonstrates how blockchain can be integrated into gaming ecosystems to create **trustless reward mechanisms**.

---

## 🧩 Problem Statement

Traditional gaming reward systems are:

* ❌ Centralized and opaque
* ❌ Prone to manipulation
* ❌ Difficult to verify

👉 Gaming Rewards solves this by using **on-chain logic** to ensure fairness and transparency.

---

## 💡 Solution

A Soroban smart contract that:

* Tracks player rewards
* Stores balances on-chain
* Allows secure claiming of rewards

---

## ⚙️ Features

* 🎯 On-chain reward tracking
* 🔐 Secure player authentication
* 📊 Transparent reward system
* ⚡ Fast and low-cost execution (Stellar)
* 🔄 Claim rewards anytime

---

## 🏗️ Architecture

### Components:

* **Smart Contract (Soroban)**

  * Handles reward logic
  * Stores balances

* **Client (CLI / Future Frontend)**

  * Interacts with contract

```
User → Soroban Contract → Storage (Rewards)
```

---

## 🧠 Smart Contract Logic

### Functions:

* `reward_player(player, amount)` → Adds reward
* `get_reward(player)` → Fetch balance
* `claim_reward(player)` → Withdraw reward

---

## 🔗 Deployed Contract
👉Freighter wallet address:
  GAZKBYR6XL7QNKNNAX2LWLADIQQRFQHIE5UVCCJBS4TW4MJOFHIQVSKA

👉 View on Stellar Expert:
 [https://stellar.expert/explorer/testnet/contract/CAQHNSTNB4KYDLALKWC6XCPL2BVZXVQRGNL7KNKKOAFZTGXRX73RJSL7](https://stellar.expert/explorer/testnet/contract/CAQHNSTNB4KYDLALKWC6XCPL2BVZXVQRGNL7KNKKOAFZTGXRX73RJSL7)

---

## 🛠️ Tech Stack

* **Rust** (Smart Contract)
* **Soroban SDK**
* **Stellar Testnet**

---

## ⚡ Getting Started

### 1. Clone Repository

```
git clone <your-repo-url>
cd gaming-rewards
```

### 2. Build Contract

```
cargo build --target wasm32-unknown-unknown --release
```

### 3. Deploy Contract

```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source gamer \
  --network testnet
```

---

## 🧪 Example Usage

### Reward a Player

```
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source gamer \
  --network testnet \
  -- reward_player \
  --player <ADDRESS> \
  --amount 100
```

### Check Balance

```
get_reward
```

### Claim Rewards

```
claim_reward
```

---

## 🔮 Future Enhancements

* 💰 Token-based rewards (USDC / custom token)
* 🏆 Leaderboard system
* 🎮 Game integration APIs
* 🧾 Event logging
* 👑 Admin control panel


---

## 🤝 Contributing

Contributions are welcome! Feel free to fork and improve.

---

## 📜 License

MIT License

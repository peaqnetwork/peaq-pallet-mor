# Peaq Pallet MOR

### Introduction

The Peaq-MOR pallet supports the distribution of rewards to machine owners. Block-rewards in the Peaq-Parachain generally distributed by the block-reward-pallet, but the percentage given to machine owners can be distributed further by this pallet. For example, currently machine owners can be rewarded for registering a new machine on the network, or for having their machines continiously online on the network. This version's main purpose is for demonstration of the PEAQ network, so this current development state of the pallet also provides the functionality to pay for using a machine.

### Terminology

- `machine` - By machine a true device in real world is meant, e.g. a charging station or electrical car. For demonstration purpose this can be a Raspberry Pi. A machine has its own account and will be identified by the Peaq-DID pallet.

- `machine owner` - In abstract here we talk about a person who owns that machine and will administrate it. In a blockchain's point of view we talk about an account.

- `reward` - Rewards are fungible tokens, which will be transfered either to the machine owner's account or the machine's account directly. Rewarding means the transfer of fungible tokens to an account (from the machine or its owner).

- `pot` - This pallet has a seperate account to administrate collected block-rewards and to be able to distribute them to machines and machine owners.

- `defined time period` - You will read several times the term "defined time period". When machines are online, they will not be rewarded by each block finalization. Instead they will get rewarded after a time period, e.g. 20 minutes. This time period is interally defined and machines will be tracked if they have been online on the network for that time period. After that time period machines were online, they can be rewarded.

### Current Use Cases

- `register_new_machine` - The owner of a machine can, after adding at least one attribute of the machine to the Peaq-DID pallet, register his machine here too and will get rewarded. The rewards will be paid on his account. This reward will be minted by the blockchain.

- `get_online_rewards` - An owner of a machine can request rewards for having its machine online for a defined time period. These rewards can be requested for each machine beeing continiously online on the network. This reward will be taken out of the pallet's pot. The amount will have the same scalar of the amount of tokens, the Peaq-MOR pallet has collected within that defined time period.

- `pay_machine_usage` - If another user want to use a machine, he has to pay for the usage of that machine. This will be simulated by this extrinsic. When calling the fee for machine usage will be taken from the user's account who used the machine and will be transfered to the machine's account.

### Reward Pot

Currently the block-reward-pallet distribute a percentage of the blockchain's block rewards to the pot of this pallet. These rewarding will happen with each block finalization. The percentage of how much of the block rewards will be transfered to this pallet can be configured in the block-reward-pallet. The pot will collect the percentage of that block rewards all the time.

The pallet will track how much rewards it gets transfered by each block and will calculate the collected tokens of that defined time period. For example, if the defined time period is 20 minutes, which means we are talking about 200 blocks will be created in that time period. The pallet will store the last 200 block rewards, which have been collected and compute the sum of it. That sum is the amount which will be transfered to a machine owner in that moment, he request his online-rewards. This sum/amount will be upated continiously.

### Integration / Implementation

For further details about the integration of this pallet to a network-node, or about the implementational details, please have a look into the Rust-documentation of the pallet and into the source code of the pallet.
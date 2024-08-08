# Nested For Loop vs Hashmap Test

This repository contains a test implementation of a nested for loop against a hashmap. 

## Test Description

In this test, we will be using a sample dataset to evaluate the performance of a nested for loop and a hashmap implementation. The dataset consists of [describe your dataset here].

## Test Results

The test results are as follows (Ran on Macbook Pro M1, 32GB RAM):

```
Nested Loop Duration: 1.584µs
Deduplicated Instruction Accounts with Nested Loop:
AccountMeta { pubkey: PubKey("Account1"), is_signer: true, is_writable: true }
AccountMeta { pubkey: PubKey("Account2"), is_signer: false, is_writable: true }
HashMap Duration: 4.75µs
Deduplicated Instruction Accounts with HashMap:
AccountMeta { pubkey: PubKey("Account1"), is_signer: true, is_writable: true }
AccountMeta { pubkey: PubKey("Account2"), is_signer: false, is_writable: true }

------

Nested Loop Duration: 1.708µs
Deduplicated Instruction Accounts with Nested Loop:
AccountMeta { pubkey: PubKey("Account1"), is_signer: true, is_writable: true }
AccountMeta { pubkey: PubKey("Account2"), is_signer: false, is_writable: true }
HashMap Duration: 4.5µs
Deduplicated Instruction Accounts with HashMap:
AccountMeta { pubkey: PubKey("Account1"), is_signer: true, is_writable: true }
AccountMeta { pubkey: PubKey("Account2"), is_signer: false, is_writable: true }


------

Nested Loop Duration: 1.625µs
Deduplicated Instruction Accounts with Nested Loop:
AccountMeta { pubkey: PubKey("Account1"), is_signer: true, is_writable: true }
AccountMeta { pubkey: PubKey("Account2"), is_signer: false, is_writable: true }
HashMap Duration: 4.708µs
Deduplicated Instruction Accounts with HashMap:
AccountMeta { pubkey: PubKey("Account2"), is_signer: false, is_writable: true }
AccountMeta { pubkey: PubKey("Account1"), is_signer: true, is_writable: true }
```


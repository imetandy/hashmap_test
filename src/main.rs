use std::collections::HashMap;
use std::time::Instant;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PubKey(String);

impl fmt::Display for PubKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
struct AccountMeta {
    pubkey: PubKey,
    is_signer: bool,
    is_writable: bool,
}


pub type IndexOfAccount = u16;

#[derive(Debug)]
struct InstructionAccount {
    pub index_in_transaction: IndexOfAccount,
    pub index_in_caller: IndexOfAccount,
    pub index_in_callee: IndexOfAccount,
    pub is_signer: bool,
    pub is_writable: bool,
}


fn deduplicate_accounts_nested(accounts: Vec<AccountMeta>) -> Vec<AccountMeta> {
    let mut deduplicated_accounts: Vec<AccountMeta> = Vec::new();

    for account_meta in accounts {
        let mut found = false;
        for dedup_account in &mut deduplicated_accounts {
            if dedup_account.pubkey == account_meta.pubkey {
                dedup_account.is_signer |= account_meta.is_signer;
                dedup_account.is_writable |= account_meta.is_writable;
                found = true;
                break;
            }
        }
        if !found {
            deduplicated_accounts.push(account_meta);
        }
    }

    deduplicated_accounts
}

fn deduplicate_accounts_hashmap(accounts: Vec<AccountMeta>) -> Vec<AccountMeta> {
    let mut account_index_map: HashMap<PubKey, AccountMeta> = HashMap::new();

    for account_meta in accounts {
        account_index_map
            .entry(account_meta.pubkey.clone())
            .and_modify(|e| {
                e.is_signer |= account_meta.is_signer;
                e.is_writable |= account_meta.is_writable;
            })
            .or_insert(account_meta);
    }

    account_index_map.into_values().collect()
}


fn deduplicate_accounts_hashmap_as_tuple(instruction_accounts: Vec<AccountMeta>) -> Vec<AccountMeta> {

     // HashMap to store the final results.
     let mut results: HashMap<PubKey, (usize, AccountMeta, Vec<usize>)> = HashMap::new();

     // Counter for assigning new sequential index
     let mut new_index = 0;
 
     for (index, account_meta) in instruction_accounts.iter().enumerate() {
         let entry = results.entry(account_meta.pubkey.clone()).or_insert_with(|| {
             let idx = new_index;
             new_index += 1;  // Increment the new index for each unique pubkey
             (idx, account_meta.clone(), vec![])
         });
 
         // Update the stored AccountMeta if the new one has higher is_signer or is_writable.
         if account_meta.is_signer || account_meta.is_writable {
             entry.1.is_signer |= account_meta.is_signer;
             entry.1.is_writable |= account_meta.is_writable;
         }
 
         // Record the position of the AccountMeta in the original vector.
         entry.2.push(index);
     }

     // Return the deduplicated accounts
     results.into_values().map(|(_, account_meta, _)| account_meta).collect()
}

// INCLUDING AHZAMAKKHTAR IMPLEMENTATION FOR COMPARISON

// New implementation with HashMap -> Time Complexity O(n)
fn prepare_instruction_new(
    instruction_accounts: &[AccountMeta],
) -> Result<Vec<InstructionAccount>, String> {
    // (Only need this to recreate the scenario)
    let mut pubkey_to_index: HashMap<PubKey, IndexOfAccount> = HashMap::new();
    //
    let mut deduplicated_instruction_accounts: HashMap<IndexOfAccount, InstructionAccount> = HashMap::new();
    let mut duplicate_indices: HashMap<PubKey, IndexOfAccount> = HashMap::new(); 
    for (instruction_account_index, account_meta) in instruction_accounts.iter().enumerate() {
        
        let index_in_transaction = if let Some(&index) = pubkey_to_index.get(&account_meta.pubkey) {
            index
        } else {
            
            let new_index = deduplicated_instruction_accounts.len() as IndexOfAccount;
            let instruction_account = InstructionAccount {
                index_in_transaction: new_index,
                index_in_caller: new_index,
                index_in_callee: instruction_account_index as IndexOfAccount,
                is_signer: account_meta.is_signer,
                is_writable: account_meta.is_writable,
            };
            deduplicated_instruction_accounts.insert(new_index, instruction_account);
            pubkey_to_index.insert(account_meta.pubkey.clone(), new_index);
            new_index
        };

        if let Some(instruction_account) = deduplicated_instruction_accounts.get_mut(&index_in_transaction) {
            duplicate_indices.insert(account_meta.pubkey.clone(), index_in_transaction);
            instruction_account.is_signer |= account_meta.is_signer;
            instruction_account.is_writable |= account_meta.is_writable;
        } else {
            let index_in_caller = pubkey_to_index
                .get(&account_meta.pubkey)
                .copied()
                .ok_or_else(|| {
                    println!("Instruction references an unknown account {}", account_meta.pubkey);
                    "NotEnoughAccountKeys".to_string()
                })?;

            let instruction_account = InstructionAccount {
                index_in_transaction,
                index_in_caller,
                index_in_callee: instruction_account_index as IndexOfAccount,
                is_signer: account_meta.is_signer,
                is_writable: account_meta.is_writable,
            };
            deduplicated_instruction_accounts.insert(index_in_transaction, instruction_account);
            duplicate_indices.insert(account_meta.pubkey.clone(), index_in_transaction);
        }
    }

    let instruction_accounts = deduplicated_instruction_accounts.into_values().collect::<Vec<InstructionAccount>>();
    Ok(instruction_accounts)

}







fn main() {
    let instruction_accounts = vec![
        AccountMeta {
            pubkey: PubKey("Account1".to_string()),
            is_signer: true,
            is_writable: false,
        },
        AccountMeta {
            pubkey: PubKey("Account2".to_string()),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: PubKey("Account3".to_string()),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: PubKey("Account2".to_string()),
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: PubKey("Account1".to_string()),
            is_signer: false,
            is_writable: true,
        },
    ];

    let start_hashmap_with_tuple = Instant::now();
    let deduplicated_accounts_hashmap_with_tuple = deduplicate_accounts_hashmap_as_tuple(instruction_accounts.clone());
    let duration_hashmap_with_tuple = start_hashmap_with_tuple.elapsed();
    println!("HashMap Duration: {:?}", duration_hashmap_with_tuple);
    println!("Deduplicated Instruction Accounts with HashMap:");

    for account in deduplicated_accounts_hashmap_with_tuple {
        println!("{:?}", account);
    }
    println!("--");
    let start = Instant::now();
    let deduplicated_accounts_nested = deduplicate_accounts_nested(instruction_accounts.clone());
    let duration_nested = start.elapsed();
    println!("Nested Loop Duration: {:?}", duration_nested);
    println!("Deduplicated Instruction Accounts with Nested Loop:");
    for account in deduplicated_accounts_nested {
        println!("{:?}", account);
    }
    println!("--");
    let start_hashmap = Instant::now();
    let deduplicated_accounts_hashmap = deduplicate_accounts_hashmap(instruction_accounts.clone());
    let duration_hashmap = start_hashmap.elapsed();
    println!("HashMap with tuple Duration: {:?}", duration_hashmap);
    println!("Deduplicated Instruction Accounts with HashMap:");
    for account in deduplicated_accounts_hashmap {
        println!("{:?}", account);
    }

    println!("--");
    // Test new implementation with Hashmap PROVIDED BY AHZAMAKKHTAR

    let start_new = Instant::now();
    let deduplicated_accounts_hashmap_ahzar = prepare_instruction_new(&instruction_accounts.clone());
    let duration_hashmap = start_new.elapsed();
    println!("HashMap with Ahzar Implementation Duration: {:?}", duration_hashmap);
    println!("Deduplicated Instruction Accounts with HashMap:");
    for account in deduplicated_accounts_hashmap_ahzar {
        println!("{:?}", account);
    }
}
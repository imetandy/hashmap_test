use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PubKey(String);

#[derive(Debug, Clone)]
struct AccountMeta {
    pubkey: PubKey,
    is_signer: bool,
    is_writable: bool,
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

    let start = Instant::now();
    let deduplicated_accounts_nested = deduplicate_accounts_nested(instruction_accounts.clone());
    let duration_nested = start.elapsed();
    println!("Nested Loop Duration: {:?}", duration_nested);
    println!("Deduplicated Instruction Accounts with Nested Loop:");
    for account in deduplicated_accounts_nested {
        println!("{:?}", account);
    }

    let start_hashmap = Instant::now();
    let deduplicated_accounts_hashmap = deduplicate_accounts_hashmap(instruction_accounts.clone());
    let duration_hashmap = start_hashmap.elapsed();
    println!("HashMap Duration: {:?}", duration_hashmap);
    println!("Deduplicated Instruction Accounts with HashMap:");
    for account in deduplicated_accounts_hashmap {
        println!("{:?}", account);
    }

}
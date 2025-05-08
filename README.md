This report details three vulnerabilities in the Solana CTF program: an insufficient signer check in `UserLevelUp`, an arithmetic underflow in `UserLevelUp`, and insufficient validation of `GameConfig` in `MintCreditsToUser`. These issues enable unauthorized level-ups, credit manipulation, and economic destabilization, with two Critical and one High severity findings. Exploitation could devalue the game’s reward system, undermining user trust and economic integrity.

# **Vulnerability 1**

**Description:** Insufficient signer check in `UserLevelUp` instruction. Because the `user_info` account isn't verified as a valid PDA derived from expected seeds, the attacker can pass a spoofed account not owned by the program. This allows arbitrary modification without burning credits.

**Criticality:** Critical. An attacker can level up any user’s account to `MAX_LEVEL` without spending credits.

**Recommendation:** Validate the `user_info` PDA in `user_level_up`.

**Proof of Concept:** (poc1.rs)

Exploit steps:

1. An attacker creates a `UserLevelUp` transaction, passing the target user’s account as `user_info`.
2. The attacker signs the transaction with their own keypair, bypassing the expected `authority_info` check.
3. The transaction succeeds, incrementing the target user’s level without burning credits.

# Vulnerability 2

**Description:** Arithmetic underflow in `UserLevelUp`. The `UserLevelUp` instruction subtracts `level_credits` from u`ser.credits` before checking sufficiency:

```rust
user.credits -= level_credits;

if !(user.credits > 0) {
    return Err(ProgramError::InsufficientFunds)
}
```

As in Rust, `=` on `u32` doesn't panic  — it wraps silently unless `checked_sub` or prior checks are used. And this post-subtraction condition only checks if credits are positive, not whether they were sufficient before subtraction. Due to underflow, credits may wrap to a large positive value. So, a large `credits_to_burn` causes `level_credits` to exceed `user.credits`, triggering an underflow which bypasses the `InsufficientFunds` check, allowing unauthorized level-ups

**Criticality:** Critical. An attacker can reach `MAX_LEVEL` with minimal credits, gaining rewards and disrupting the game economy.

**Recommendation:** 

Use `checked_sub` to prevent underflow, as it’s idiomatic and concise:

```rust
let new_credits = user.credits.checked_sub(level_credits)
    .ok_or(ProgramError::InsufficientFunds)?;
user.credits = new_credits;
```

`checked_sub` prevents underflow by returning `None` , if the subtraction would result in a negative value.

Alternatively, for explicit clarity:

```rust
if user.credits < level_credits {
return Err(ProgramError::InsufficientFunds);
}
user.credits -= level_credits;
```

**Proof of Concept:** (poc2.rs)

Exploit steps:

1. The attacker submits a `UserLevelUp` transaction with `credits_to_burn = u32::MAX.`
2. The large `credits_to_burn` causes `level_credits` to exceed `user.credits`, triggering an underflow.
3. The transaction succeeds, setting `user.level` to `MAX_LEVEL`.

# Vulnerability 3

**Description:** The `MintCreditsToUser` instruction does not verify that the `game_config_info` account’s `account_type` is `GameConfig` before deserialization, relying only on PDA validation. If an attacker can supply a malicious account owned by the program (e.g., via a testnet setup or admin key compromise), deserialization could fail or produce unexpected behaviour, potentially allowing unauthorized credit minting or state corruption.

**Criticality:** High. While exploitation requires specific conditions (e.g., testnet flexibility or admin key access), it risks economic destabilization through unauthorized credits.

**Recommendation:**

- Validate `account_type` before deserialization in `mint_credits_to_user`:
    
    ```rust
    if game_config_info.try_get_type()? != AccountType::GameConfig {
        return Err(ProgramError::InvalidAccountData);
    }
    ```
    

**Proof of Concept:** (poc3.rs)

Exploit steps:

1. The attacker creates a malicious account with the PDA derived from [`admin_key`, `GAME_CONFIG_SEED`].
2. The attacker submits a `MintCreditsToUser` transaction, passing the malicious account as `game_config_info`.
3. The transaction may succeed or cause deserialization errors, demonstrating the lack of `account_type` validation.

***Note:** This PoC assumes the CTF testnet allows manual creation of the PDA account, as PDAs are deterministic and typically owned by the program. In production, exploitation requires admin key compromise.*

# Conclusion

This audit identifies critical flaws in account validation, arithmetic safety and state verification, enabling unauthorized level-ups and credit manipulation. The provided PoCs confirm exploitability, and the recommendations secure the contract.
This report details three vulnerabilities in the Solana CTF program: an insufficient signer check in UserLevelUp, an arithmetic underflow in UserLevelUp, and insufficient validation of GameConfig in MintCreditsToUser. These issues enable unauthorized level-ups, credit manipulation, and economic destabilization, with two Critical and one High severity findings. Exploitation could devalue the game’s reward system, undermining user trust and economic integrity.

# **Vulnerability 1**

**Description:** Insufficient signer check in `UserLevelUp` instruction. Because the `user_info` account isn't verified as a valid PDA derived from expected seeds, the attacker can pass a spoofed account not owned by the program. This allows arbitrary modification without burning credits.

**Criticality:** Critical. An attacker can level up any user’s account to `MAX_LEVEL` without spending credits.

**Recommendation:** Validate the `user_info` PDA in `user_level_up`.

**Proof of Concept:** (poc1.rs)

Exploit steps:

1. An attacker creates a `UserLevelUp` transaction, passing the target user’s account as `user_info`.
2. The attacker signs the transaction with their own keypair, bypassing the expected `authority_info` check.
3. The transaction succeeds, incrementing the target user’s level without burning credits.
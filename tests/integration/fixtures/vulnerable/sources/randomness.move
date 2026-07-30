module vulnerable::randomness {
    use sui::tx_context::{Self, TxContext};

    /// Vulnerable: lottery/gaming function using predictable digest.
    public fun play_lottery(ctx: &mut TxContext): bool {
        let digest = tx_context::digest(ctx);
        // seed PRNG using digest
        true
    }
}

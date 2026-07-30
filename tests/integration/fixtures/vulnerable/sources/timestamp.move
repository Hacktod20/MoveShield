module vulnerable::timestamp {
    use sui::tx_context::{Self, TxContext};

    /// Vulnerable: relies on epoch_time without Clock parameter.
    public fun check_expiration(ctx: &mut TxContext): bool {
        let current_time = tx_context::epoch_time(ctx);
        current_time > 10000
    }
}

module example::clean {
    /// A simple, clean helper function to return base fee.
    public fun get_base_fee(): u64 {
        100
    }

    /// Calculates fee based on amount and a base fee.
    public fun calculate_fee(amount: u64): u64 {
        amount + get_base_fee()
    }
}

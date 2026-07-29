module example::complex {
    use example::clean;

    /// An intentionally complex function to verify MS001 (Cyclomatic Complexity) rule.
    /// It contains multiple branching points and inter-module calls.
    public fun transfer(
        amount: u64,
        sender: address,
        receiver: address,
        is_whitelisted: bool,
        is_frozen: bool,
        sender_balance: u64
    ): bool {
        let success = false;

        if (amount == 0) {
            return false;
        };

        if (is_frozen) {
            return false;
        };

        if (!is_whitelisted && amount > 1000) {
            return false;
        };

        if (sender_balance < amount) {
            return false;
        };

        if (sender == receiver) {
            return false;
        };

        if (amount > 10000) {
            return false;
        };

        if (amount % 2 == 1) {
            success = true;
        };

        if (sender_balance - amount < 100) {
            return false;
        };

        if (amount == 500) {
            success = true;
        };

        // Inter-module function call
        let fee = clean::calculate_fee(amount);
        if (sender_balance >= amount + fee) {
            success = true;
        };

        success
    }
}

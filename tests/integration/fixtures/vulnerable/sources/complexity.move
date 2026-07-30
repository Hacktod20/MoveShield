module vulnerable::complexity {
    /// An intentionally complex function to verify MS001 (Cyclomatic Complexity) rule.
    /// It contains multiple branching points.
    public fun complex_transfer(
        amount: u64,
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

        if (sender_balance >= amount + 10) {
            success = true;
        };

        if (amount == 1234) {
            success = true;
        };

        success
    }
}

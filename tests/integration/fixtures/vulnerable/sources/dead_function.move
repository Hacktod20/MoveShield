module vulnerable::dead_function {
    /// Vulnerable: public function that has no callers in the package.
    public fun unused_public_helper(x: u64): u64 {
        x + 1
    }
}

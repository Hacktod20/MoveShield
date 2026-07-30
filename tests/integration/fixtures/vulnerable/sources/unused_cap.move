module vulnerable::unused_cap {
    /// Vulnerable: defined AdminCap but never used as function parameter in this module.
    struct AdminCap has key, store {
        id: sui::object::UID,
    }

    public fun hello() {}
}

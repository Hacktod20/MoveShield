module vulnerable::shared_object {
    struct SharedState has key, store {
        id: sui::object::UID,
        value: u64,
    }

    /// Vulnerable: modifies a mutable shared object parameter (State/Store) without any capability.
    public fun modify_state(state: &mut SharedState, new_val: u64) {
        state.value = new_val;
    }
}

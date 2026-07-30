module vulnerable::capability_leak {
    use sui::transfer;

    struct AdminCap has key, store {
        id: sui::object::UID,
    }

    /// Vulnerable: passes AdminCap to a public function and transfers it.
    public fun leak_admin_cap(cap: AdminCap, recipient: address) {
        transfer::public_transfer(cap, recipient);
    }
}

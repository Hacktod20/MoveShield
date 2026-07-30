module vulnerable::treasury_config {
    use sui::coin::TreasuryCap;
    use sui::transfer;

    struct MY_COIN has drop {}

    /// Vulnerable: public transfer of TreasuryCap.
    public fun leak_treasury_cap(cap: TreasuryCap<MY_COIN>, recipient: address) {
        transfer::public_transfer(cap, recipient);
    }
}

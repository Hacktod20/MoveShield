module vulnerable::public_transfer {
    use sui::transfer;

    struct RestrictedAsset has key, store {
        id: sui::object::UID,
    }

    /// Vulnerable: calls public_transfer directly to transfer a restricted asset.
    public fun leak_asset(asset: RestrictedAsset, recipient: address) {
        transfer::public_transfer(asset, recipient);
    }
}

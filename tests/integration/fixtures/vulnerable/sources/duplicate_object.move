module vulnerable::duplicate_object {
    use sui::object;
    use sui::tx_context::TxContext;

    struct Item has key, store {
        id: object::UID,
    }

    /// Vulnerable: calls object::new multiple times inside a single function block.
    public fun mint_two_items(ctx: &mut TxContext): (Item, Item) {
        let first = Item { id: object::new(ctx) };
        let second = Item { id: object::new(ctx) };
        (first, second)
    }
}

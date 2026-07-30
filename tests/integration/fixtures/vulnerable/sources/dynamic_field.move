module vulnerable::dynamic_field {
    use sui::dynamic_field;

    struct Parent has key {
        id: sui::object::UID,
    }

    /// Vulnerable: calls dynamic_field::add without checking exists_ or has_field.
    public fun add_field(parent: &mut Parent, name: u64, value: u64) {
        dynamic_field::add(&mut parent.id, name, value);
    }
}

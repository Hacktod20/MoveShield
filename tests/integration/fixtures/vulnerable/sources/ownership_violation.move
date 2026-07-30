module vulnerable::ownership_violation {
    struct InnerObject has key {
        id: sui::object::UID,
    }

    /// Vulnerable: wrapped object nested structural ownership (key-ability struct inside another key-ability struct).
    struct OuterObject has key {
        id: sui::object::UID,
        inner: InnerObject,
    }
}

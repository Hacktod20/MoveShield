module 0x42::types {
    public struct Value has drop {}
}

module 0x42::functions {
    use 0x42::types::Value;
    public fun visit(_: &Value) {}
}

module 0x42::unsupported_use_fun {
    use 0x42::types::Value;
    use fun 0x42::functions::visit as Value.visit;
    public fun call(value: &Value) {
        value.visit();
    }
}
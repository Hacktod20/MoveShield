module 0x42::unsupported_macro_call {

    public fun call() {
        identity!(42);
    }

    public macro fun identity($value: u64): u64 { $value }
}
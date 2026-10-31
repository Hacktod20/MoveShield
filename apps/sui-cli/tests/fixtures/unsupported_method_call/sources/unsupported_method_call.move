module 0x42::unsupported_method_call {
    public struct Subject has drop {}

    fun inspect(_: &Subject) {}

    public fun call(subject: &Subject) {
        subject.inspect();
    }
}
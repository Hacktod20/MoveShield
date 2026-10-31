module 0x42::root {
    use 0x43::local_helper;

    public fun call() {
        local_helper::value();
    }
}
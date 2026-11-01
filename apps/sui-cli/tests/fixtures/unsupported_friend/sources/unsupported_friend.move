module 0x42::trusted {}

module 0x42::unsupported_friend {
    friend 0x42::trusted;
}
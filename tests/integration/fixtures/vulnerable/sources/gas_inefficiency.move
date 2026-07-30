module vulnerable::gas_inefficiency {
    use std::vector;

    /// Vulnerable: public function with a loop doing heavy vector insertions.
    public fun compute_something(vals: vector<u64>) {
        let out = vector::empty<u64>();
        let i = 0;
        while (i < 100) {
            vector::push_back(&mut out, i);
            i = i + 1;
        }
    }
}

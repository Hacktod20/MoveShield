module vulnerable::access_control {
    /// Vulnerable: entry function performing sensitive operation without TxContext, signer, or Capability.
    public entry fun mint_tokens(amount: u64) {
        // performs mint
    }
}

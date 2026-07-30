module vulnerable::upgrade_policy {
    /// Vulnerable: public upgrade function without UpgradeCap validation.
    public fun authorize_upgrade(policy: u8) {
        // missing UpgradeCap parameter
    }
}

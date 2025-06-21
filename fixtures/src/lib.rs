pub mod notary {
    pub const PUB_KEY: &str = include_str!("../notary/notary.pub");
}

pub mod proof {
    pub const PRESENTATION_32B_FULL: &str = include_str!("../proof/presentation_32b_full.json");
    pub const PRESENTATION_32B_PRIVATE: &str =
        include_str!("../proof/presentation_32b_private.json");
    pub const PRESENTATION_32B_PUBLIC: &str = include_str!("../proof/presentation_32b_public.json");

    pub const PRESENTATION_1KB: &str = include_str!("../proof/presentation_1kb.json");

    pub const PRECOMPUTE_32B: &[u8] = include_bytes!("../proof/precompute-32b.tlsn");
    pub const PRECOMPUTE_1KB: &[u8] = include_bytes!("../proof/precompute-1kb.tlsn");
}

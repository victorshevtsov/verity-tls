//! Verifiable presentation.
//!
//! We borrow the term "presentation" from the
//! [W3C Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/#presentations-0).
//!
//! > Data derived from one or more verifiable credentials, issued by one or
//! > more issuers, that is shared with a specific verifier. A verifiable
//! > presentation is a tamper-evident presentation encoded in such a way that
//! > authorship of the data can be trusted after a process of cryptographic
//! > verification. Certain types of verifiable presentations might contain data
//! > that is synthesized from, but do not contain, the original verifiable
//! > credentials (for example, zero-knowledge proofs).
//!
//! Instead of a credential, a presentation in this context is a proof of an
//! attestation from a Notary along with additional selectively disclosed
//! information about the TLS connection such as the server's identity and the
//! application data communicated with the server.
//!
//! A presentation is self-contained and can be verified by a Verifier without
//! needing access to external data. The Verifier need only check that the key
//! used to sign the attestation, referred to as a [`VerifyingKey`], is from a
//! Notary they trust. See an [example](crate#verifying-a-presentation) in the
//! crate level documentation.

#[cfg(all(feature = "private-facets", feature = "public-facets"))]
pub mod full;

#[cfg(feature = "private-facets")]
pub mod private;

#[cfg(feature = "public-facets")]
pub mod public;

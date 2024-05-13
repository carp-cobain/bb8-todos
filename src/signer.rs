use crate::Result;
use ed25519_dalek::{
    ed25519::signature::SignerMut,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Signature, SigningKey, VerifyingKey,
};
use std::env;

// Sign messages

pub struct Signer {
    pem: String,
}

impl Default for Signer {
    fn default() -> Self {
        let pem = env::var("SIGNING_KEY").expect("SIGNING_KEY not set");
        SigningKey::from_pkcs8_pem(&pem).expect("invalid signing key pem");
        Self { pem }
    }
}

impl Signer {
    pub fn sign(&self, msg: &[u8]) -> Result<Vec<u8>> {
        let mut key = SigningKey::from_pkcs8_pem(&self.pem)?;
        let sig: Signature = key.sign(msg);
        Ok(sig.to_vec())
    }
}

// Verify message signatures

pub struct Verifier {
    pem: String,
}

impl Default for Verifier {
    fn default() -> Self {
        let pem = env::var("VERIFYING_KEY").expect("VERIFYING_KEY not found in env");
        VerifyingKey::from_public_key_pem(&pem).expect("invalid public key pem");
        Self { pem }
    }
}

impl Verifier {
    pub fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<()> {
        let key = VerifyingKey::from_public_key_pem(&self.pem).unwrap();
        let sig = Signature::try_from(sig)?;
        key.verify_strict(msg, &sig)?;
        Ok(())
    }
}

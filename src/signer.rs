use crate::Result;
use ed25519_dalek::{
    ed25519::signature::SignerMut,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Signature, SigningKey, VerifyingKey,
};
use std::env;

// Sign messages

pub struct Signer {
    key: SigningKey,
}

impl Default for Signer {
    fn default() -> Self {
        let pem = env::var("SIGNING_KEY").expect("SIGNING_KEY not set");
        let key = SigningKey::from_pkcs8_pem(&pem).expect("invalid signing key pem");
        Self { key }
    }
}

impl Signer {
    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        let mut key = self.key.to_owned();
        let sig: Signature = key.sign(msg);
        sig.to_vec()
    }
}

// Verify message signatures

pub struct Verifier {
    key: VerifyingKey,
}

impl Default for Verifier {
    fn default() -> Self {
        let pem = env::var("VERIFYING_KEY").expect("VERIFYING_KEY not found in env");
        let key = VerifyingKey::from_public_key_pem(&pem).expect("invalid public key pem");
        Self { key }
    }
}

impl Verifier {
    pub fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<()> {
        let sig = Signature::try_from(sig)?;
        self.key.verify_strict(msg, &sig)?;
        Ok(())
    }
}

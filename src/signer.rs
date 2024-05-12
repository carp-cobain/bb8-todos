use crate::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use ed25519_dalek::{
    ed25519::signature::SignerMut,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Signature, SigningKey, VerifyingKey,
};
use std::env;

// Sign messages

pub struct Signer {
    signing_key: String,
}

impl Default for Signer {
    fn default() -> Self {
        let s = env::var("SIGNING_KEY").expect("SIGNING_KEY not set");
        SigningKey::from_pkcs8_pem(&s).expect("invalid signing key");
        Self { signing_key: s }
    }
}

impl Signer {
    pub fn sign(&self, message: &[u8]) -> Result<String> {
        let mut signing_key = SigningKey::from_pkcs8_pem(&self.signing_key)?;
        let signature: Signature = signing_key.sign(message);
        let encoded = URL_SAFE.encode(signature.to_bytes());
        Ok(encoded)
    }
}

// Verify message signatures

pub struct Verifier {
    verifying_key: String,
}

impl Default for Verifier {
    fn default() -> Self {
        let s = env::var("VERIFYING_KEY").expect("VERIFYING_KEY not found in env");
        VerifyingKey::from_public_key_pem(&s).expect("invalid public key pem");
        Self { verifying_key: s }
    }
}

impl Verifier {
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        let verifying_key = VerifyingKey::from_public_key_pem(&self.verifying_key).unwrap();
        let signature = Signature::try_from(signature)?;
        verifying_key.verify_strict(message, &signature)?;
        Ok(())
    }
}

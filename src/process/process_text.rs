use crate::{TextSignFormat, read_buffer_from_input};
use anyhow::Result;
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use ed25519_dalek::{SECRET_KEY_LENGTH, Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::str::FromStr;

type SecretKey = [u8; SECRET_KEY_LENGTH];

pub trait TextSign {
    fn sign(&self, data: &[u8]) -> Result<String>;
}

pub trait TextVerify {
    fn verify(&self, data: &[u8], sign: String) -> Result<bool>;
}

pub struct Blake3 {
    key: SecretKey,
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub trait KeyLoad {
    fn new(key: SecretKey) -> Self;
    fn try_new(key: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        if key.len() < SECRET_KEY_LENGTH {
            return Err(anyhow::anyhow!(
                "The Hash key length must greater than or equal to 32"
            ));
        }

        let key = key[..32].try_into()?;
        Ok(Self::new(key))
    }
    fn load(key: impl AsRef<str>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = read_buffer_from_input(key.as_ref())?;
        Self::try_new(&key)
    }
}

impl KeyLoad for Blake3 {
    fn new(key: SecretKey) -> Self {
        Self { key }
    }
}

impl KeyLoad for Ed25519Signer {
    fn new(key: SecretKey) -> Self {
        let signing_key = SigningKey::from_bytes(&key);
        Self { key: signing_key }
    }
}

impl KeyLoad for Ed25519Verifier {
    fn new(key: SecretKey) -> Self {
        // VerifyKey generate from sign_key
        let sign_key = SigningKey::from_bytes(&key);
        let verify_key = sign_key.verifying_key();
        Self { key: verify_key }
    }
}

impl TextSign for Blake3 {
    fn sign(&self, data: &[u8]) -> Result<String> {
        let signature = blake3::keyed_hash(&self.key, data).to_string();
        let signature = BASE64_URL_SAFE_NO_PAD.encode(signature);
        Ok(signature)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, data: &[u8], sign: String) -> Result<bool> {
        let hash = self.sign(data)?;
        Ok(sign == hash)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, data: &[u8]) -> Result<String> {
        let signature = self.key.sign(data).to_string();
        let signature = BASE64_URL_SAFE_NO_PAD.encode(signature);
        Ok(signature)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, data: &[u8], sign: String) -> Result<bool> {
        let signature = BASE64_URL_SAFE_NO_PAD.decode(sign)?;
        let signature = Signature::from_str(str::from_utf8(&signature)?)?;
        Ok(self.key.verify(data, &signature).is_ok())
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let buf = read_buffer_from_input(input)?;
    match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.sign(&buf),
        TextSignFormat::ED25519 => Ed25519Signer::load(key)?.sign(&buf),
    }
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sign: String,
) -> anyhow::Result<bool> {
    let buf = read_buffer_from_input(input)?;
    match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.verify(&buf, sign),
        TextSignFormat::ED25519 => Ed25519Verifier::load(key)?.verify(&buf, sign),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        process::process_text::{
            Blake3, Ed25519Signer, Ed25519Verifier, KeyLoad, TextSign, TextVerify,
        },
        process_gen_pass,
    };

    #[test]
    fn test_blake3_sign_and_verify() {
        let key = process_gen_pass(32, true, true, true, true).unwrap();
        let blake3 = Blake3::try_new(key.as_bytes()).unwrap();
        let data = b"Hello, rust!";
        let sign = blake3.sign(data).unwrap();
        assert!(blake3.verify(data, sign).is_ok());
    }

    #[test]
    fn test_ed25519_sign_and_verify() {
        let key = process_gen_pass(32, true, true, true, true).unwrap();
        let singer = Ed25519Signer::try_new(key.as_bytes()).unwrap();
        let data = b"Hello, rust!";
        let sign = singer.sign(data).unwrap();
        let verifier = Ed25519Verifier::try_new(key.as_bytes()).unwrap();
        assert!(verifier.verify(data, sign).is_ok());
    }
}

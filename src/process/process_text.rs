use crate::{TextSignFormat, process_gen_pass, read_buffer_from_input};
use anyhow::Result;
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use ed25519_dalek::{SECRET_KEY_LENGTH, Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::str::FromStr;

type SecretKey = [u8; SECRET_KEY_LENGTH];
const CHACHA_NONCE_LEN: usize = 12;

pub trait TextSign {
    fn sign(&self, data: &[u8]) -> Result<String>;
}

pub trait TextVerify {
    fn verify(&self, data: &[u8], sign: String) -> Result<bool>;
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

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
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
        let verifying_key = VerifyingKey::from_bytes(&key).expect("Parse public key failed");
        Self { key: verifying_key }
    }
}

impl TextSign for Blake3 {
    fn sign(&self, data: &[u8]) -> Result<String> {
        let signed = blake3::keyed_hash(&self.key, data).to_string();
        // Convert signature to url safe string for get searchParams
        let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);
        Ok(signed)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, data: &[u8], signed: String) -> Result<bool> {
        let cp_signed = self.sign(data)?;
        Ok(cp_signed == signed)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, data: &[u8]) -> Result<String> {
        let signed = self.key.sign(data).to_string();
        // Encode signature to url safe string for get searchParams
        let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);
        Ok(signed)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, data: &[u8], sign: String) -> Result<bool> {
        // Decode signature from url safe string
        let signed = BASE64_URL_SAFE_NO_PAD.decode(sign)?;
        let signed = Signature::from_str(str::from_utf8(&signed)?)?;
        Ok(self.key.verify(data, &signed).is_ok())
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let buf = read_buffer_from_input(input)?;
    match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.sign(&buf),
        TextSignFormat::ED25519 => Ed25519Signer::load(key)?.sign(&buf),
        _ => Err(anyhow::anyhow!("Invalid text sign format")),
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_gen_pass(SECRET_KEY_LENGTH, true, true, true, true)?;
        Ok(vec![key.into()])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key();
        let sk = sk.to_bytes().to_vec();
        let pk = pk.to_bytes().to_vec();
        Ok(vec![sk, pk])
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
        _ => Err(anyhow::anyhow!("Invalid text verify format")),
    }
}

pub fn process_key_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::ED25519 => Ed25519Signer::generate(),
        TextSignFormat::ChaCha20Poly1305 => {
            let key = ChaCha20Poly1305::generate_key(&mut OsRng).to_vec();
            Ok(vec![key])
        }
    }
}

pub fn process_text_encrypt(input: &str, key: &str) -> Result<Vec<u8>> {
    let data = read_buffer_from_input(input)?;
    let key = read_buffer_from_input(key)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key).unwrap();
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data.as_ref()).unwrap();
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

pub fn process_text_decrypt(input: &str, key: &str) -> Result<Vec<u8>> {
    let data = read_buffer_from_input(input)?;
    if data.len() < CHACHA_NONCE_LEN {
        return Err(anyhow::anyhow!("Invalid ciphertext: too short"));
    }
    let key = read_buffer_from_input(key)?;
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher =
        ChaCha20Poly1305::new_from_slice(&key).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let ciphertext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(ciphertext)
}

#[cfg(test)]
mod test {
    use crate::process::process_text::{
        Blake3, Ed25519Signer, Ed25519Verifier, KeyGenerator, KeyLoad, TextSign, TextVerify,
    };

    #[test]
    fn test_blake3_sign_and_verify() {
        let key = Blake3::generate().unwrap();
        let blake3 = Blake3::try_new(&key[0]).unwrap();
        let data = b"Hello, rust!";
        let signed = blake3.sign(data).unwrap();
        assert!(blake3.verify(data, signed).is_ok());
    }

    #[test]
    fn test_ed25519_sign_and_verify() {
        let key = Ed25519Signer::generate().unwrap();
        let singer = Ed25519Signer::try_new(&key[0]).unwrap();
        let data = b"Hello, rust!";
        let signed = singer.sign(data).unwrap();
        let verifier = Ed25519Verifier::try_new(&key[1]).unwrap();
        assert!(verifier.verify(data, signed).is_ok());
    }
}

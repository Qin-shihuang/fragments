use pgp::{types::PublicKeyTrait, Deserializable, SignedPublicKey, StandaloneSignature};


pub fn verify_signature(key: &SignedPublicKey, data: &str, signature: &str) -> Result<(), String> {
    let data = data.as_bytes();
    let (signature, _) = StandaloneSignature::from_string(signature).map_err(|_| "Invalid signature.")?;
    if key.is_signing_key() && signature.verify(key, data).is_ok() {
        return Ok(());
    }
    Err("Signature verification failed.".to_string())
}

pub fn get_keyid_string(key: &SignedPublicKey) -> String {
    let mut keyid_str = String::from("0x");
    for b in key.key_id().to_vec() {
        keyid_str.push_str(&format!("{:02X}", b));
    }
    keyid_str
}


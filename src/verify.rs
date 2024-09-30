use pgp::{types::PublicKeyTrait, Deserializable, SignedPublicKey, StandaloneSignature};


pub fn verify_signature(key: &SignedPublicKey, data: &str, signature: &str) -> bool {
    let data = data.as_bytes();
    let (signature, _) = StandaloneSignature::from_string(signature).unwrap();
    if key.is_signing_key() && signature.verify(key, data).is_ok() {
        return true;
    }
    false
}

pub fn get_keyid_string(key: &SignedPublicKey) -> String {
    let mut keyid_str = String::from("0x");
    for b in key.key_id().to_vec() {
        keyid_str.push_str(&format!("{:02X}", b));
    }
    keyid_str
}


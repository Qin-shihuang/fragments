use pgp::{types::PublicKeyTrait, Deserializable, SignedPublicKey, StandaloneSignature};

pub fn verify_signature(key: &SignedPublicKey, data: &str, signature: &str) -> Result<(), String> {
    let data = data.as_bytes();
    let (signature, _) =
        StandaloneSignature::from_string(signature).map_err(|_| "Invalid signature.")?;
    let now = chrono::Utc::now();
    let fail = now - chrono::Duration::minutes(30);
    let creation_time = signature.signature.created().unwrap_or(&fail);
    if now - creation_time > chrono::Duration::minutes(20) {
        return Err("Signature is too old, use one signed within the last 20 minutes.".to_string());
    }
    if let Err(e) = signature.verify(key, data) {
        return Err(format!("Signature verification failed: {}", e));
    }
    Ok(())
}

pub fn get_keyid_string(key: &SignedPublicKey) -> String {
    let mut keyid_str = String::from("0x");
    for b in key.key_id().to_vec() {
        keyid_str.push_str(&format!("{:02X}", b));
    }
    keyid_str
}

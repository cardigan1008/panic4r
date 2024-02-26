//! PBES2 encryption tests

#![cfg(feature = "pbes2")]

use core::convert::TryFrom;
use hex_literal::hex;

/// PBES2 + PBKDF2-SHA256 + AES-256-CBC `AlgorithmIdentifier` example.
///
/// Generated by OpenSSL and extracted from the `pkcs8` crate's
/// `tests/examples/ed25519-encpriv-aes256-pbkdf2-sha256.der` test vector.
const PBES2_PBKDF2_SHA256_AES256CBC_ALG_ID: &[u8] = &hex!(
    "305706092a864886f70d01050d304a302906092a864886f70d01050c301c0408
     79d982e70df91a8802020800300c06082a864886f70d02090500301d06096086
     4801650304012a0410b2d02d78b2efd9dff694cf8e0af40925"
);

/// PBES2 + scrypt + AES-256-CBC `AlgorithmIdentifier` example.
///
/// Generated by OpenSSL and extracted from the `pkcs8` crate's
/// `ed25519-encpriv-aes256-scrypt.der` test vector.
const PBES2_SCRYPT_AES256CBC_ALG_ID: &[u8] = &hex!(
    "304f06092a864886f70d01050d3042302106092b06010401da47040b30140408
    e6211e2348ad69e002024000020108020101301d060960864801650304012a041
    09bd0a6251f2254f9fd5963887c27cf01"
);

/// Plaintext of Ed25519 PKCS#8 private key.
///
/// This is the hex-encoded contents of `ed25519-priv.der` from
/// `pkcs8/tests/examples`.
const ED25519_PKCS8_KEY_PLAINTEXT: &[u8] = &hex!(
    "302e020100300506032b65700422042017ed9c73e9db649ec189a612831c5fc5
     70238207c1aa9dfbd2c53e3ff5e5ea85"
);

/// Ciphertext of Ed25519 PKCS#8 private key when encrypted using
/// PBKDF2-SHA256 as the KDF.
///
/// Extracted with:
/// $ openssl asn1parse -inform der -in pkcs8/tests/examples/ed25519-encpriv-aes256-pbkdf2-sha256.der
const ED25519_PKCS8_KEY_CIPHERTEXT_PBKDF2_SHA256: &[u8] = &hex!(
    "D0CD6C770F4BB87176422305C17401809E226674CE74185D221BFDAA95069890
     C8882FCE02B05D41BCBF54B035595BCD4154B32593708469B86AACF8815A7B2B"
);

/// Ciphertext of Ed25519 PKCS#8 private key when encrypted using
/// scrypt as the KDF.
///
/// Extracted with:
/// $ openssl asn1parse -inform der -in pkcs8/tests/examples/ed25519-encpriv-aes256-scrypt.der
const ED25519_PKCS8_KEY_CIPHERTEXT_SCRYPT: &[u8] = &hex!(
    "CC62BA773C0F495FAB3668E4FCEFCDB08E78A0EE15E0A15013F62ABE08DAA742
     065EEB366D6E6C98CC3B0E7E69BDC861C88AFEB8F03DBA1E2C6D99D06D17360C"
);

/// Password used to encrypt the keys.
const PASSWORD: &[u8] = b"hunter42"; // Bad password; don't actually use outside tests!

#[test]
fn decrypt_pbes2_pbkdf2_sha256_aes256cbc() {
    let scheme = pkcs5::EncryptionScheme::try_from(PBES2_PBKDF2_SHA256_AES256CBC_ALG_ID).unwrap();
    let mut buffer = Vec::from(ED25519_PKCS8_KEY_CIPHERTEXT_PBKDF2_SHA256);
    let plaintext = scheme.decrypt_in_place(PASSWORD, &mut buffer).unwrap();
    assert_eq!(plaintext, ED25519_PKCS8_KEY_PLAINTEXT);
}

#[test]
fn decrypt_pbes2_scrypt_aes256cbc() {
    let scheme = pkcs5::EncryptionScheme::try_from(PBES2_SCRYPT_AES256CBC_ALG_ID).unwrap();
    let mut buffer = Vec::from(ED25519_PKCS8_KEY_CIPHERTEXT_SCRYPT);
    let plaintext = scheme.decrypt_in_place(PASSWORD, &mut buffer).unwrap();
    assert_eq!(plaintext, ED25519_PKCS8_KEY_PLAINTEXT);
}

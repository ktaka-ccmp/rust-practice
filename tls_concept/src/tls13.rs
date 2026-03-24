// ============================================================
// TLS 1.3 コンセプトコード (Rust)
// ============================================================
// cargo run --bin tls13

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as B64};
use hkdf::Hkdf;
use p256::{
    PublicKey,
    ecdh::EphemeralSecret,
    ecdsa::{Signature, SigningKey, VerifyingKey, signature::Signer, signature::Verifier},
};
use rand::rngs::OsRng;
use sha2::Sha256;

fn short(s: &str) -> String {
    if s.len() > 32 {
        format!("{}...", &s[..32])
    } else {
        s.to_string()
    }
}

fn derive_session_key(shared_secret: &[u8], info: &[u8]) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(None, shared_secret);
    let mut okm = [0u8; 32];
    hk.expand(info, &mut okm).unwrap();
    okm
}

fn main() {
    // ─── ① CA がサーバー証明書に署名 ────────────────────────────

    let ca_signing_key = SigningKey::random(&mut OsRng);
    let server_cert_key = SigningKey::random(&mut OsRng);
    let server_cert_verify = VerifyingKey::from(&server_cert_key);

    // 証明書の中身（本物はX.509/DER、ここでは文字列で表現）
    let server_pub_hex = hex::encode(server_cert_verify.to_encoded_point(false).as_bytes());
    let cert_body = format!(
        r#"{{"subject":"example.com","issuer":"My CA","publicKey":"{}","notBefore":"2025-01-01","notAfter":"2026-01-01"}}"#,
        server_pub_hex
    );
    let ca_signature: Signature = ca_signing_key.sign(cert_body.as_bytes());
    println!("CA署名: {}...", short(&B64.encode(ca_signature.to_der())));

    // ─── ② ECDH 鍵ペア生成（ClientHello / ServerHello）──────────

    let client_ecdh = EphemeralSecret::random(&mut OsRng);
    let server_ecdh = EphemeralSecret::random(&mut OsRng);

    let client_ecdh_pub = PublicKey::from(&client_ecdh);
    let server_ecdh_pub = PublicKey::from(&server_ecdh);

    // ─── ③ セッション鍵の導出 ────────────────────────────────────

    let client_shared = client_ecdh.diffie_hellman(&server_ecdh_pub);
    let server_shared = server_ecdh.diffie_hellman(&client_ecdh_pub);

    let client_session_key =
        derive_session_key(client_shared.raw_secret_bytes(), b"tls13-session-key");
    let server_session_key =
        derive_session_key(server_shared.raw_secret_bytes(), b"tls13-session-key");

    assert_eq!(
        client_session_key, server_session_key,
        "セッション鍵が一致しない！"
    );
    println!(
        "セッション鍵確立: {}... (PFS達成)",
        hex::encode(&client_session_key[..12])
    );

    // ─── ④ サーバーが Certificate + CertificateVerify を暗号化して送信 ──
    // TLS 1.3: セッション鍵確立後に暗号化して送る

    let handshake_transcript = b"handshake-transcript:tls13";
    let cert_verify_sig: Signature = server_cert_key.sign(handshake_transcript);

    let server_message = format!(
        r#"{{"certificate":{},"caSignature":"{}","certVerifySignature":"{}","handshakeTranscript":"{}"}}"#,
        cert_body,
        B64.encode(ca_signature.to_der()),
        B64.encode(cert_verify_sig.to_der()),
        B64.encode(handshake_transcript),
    );

    let cipher = Aes256Gcm::new_from_slice(&client_session_key).unwrap();
    let iv1: [u8; 12] = rand::random();
    let nonce1 = Nonce::from(iv1);
    let encrypted_server_msg = cipher.encrypt(&nonce1, server_message.as_bytes()).unwrap();
    println!(
        "サーバー: Certificate+CertificateVerify を暗号化して送信: {}",
        short(&B64.encode(&encrypted_server_msg))
    );

    // ─── ⑤ クライアントが復号 → CA検証 → CertificateVerify検証 ───

    let cipher_c = Aes256Gcm::new_from_slice(&client_session_key).unwrap();
    let decrypted_bytes = cipher_c
        .decrypt(&nonce1, encrypted_server_msg.as_slice())
        .unwrap();
    let decrypted_msg = String::from_utf8(decrypted_bytes).unwrap();

    // CA公開鍵で証明書を検証
    let ca_verifying = VerifyingKey::from(&ca_signing_key);
    let parsed_ca_sig = Signature::from_der(
        &B64.decode(extract_field(&decrypted_msg, "caSignature"))
            .unwrap(),
    )
    .unwrap();
    ca_verifying
        .verify(cert_body.as_bytes(), &parsed_ca_sig)
        .expect("証明書検証失敗！");
    println!("証明書検証 (CA署名): OK");

    // 証明書からサーバーの公開鍵を取り出す
    let pub_hex = extract_field(&decrypted_msg, "publicKey");
    let pub_bytes = hex::decode(&pub_hex).unwrap();
    let server_pub_from_cert =
        VerifyingKey::from_encoded_point(&p256::EncodedPoint::from_bytes(&pub_bytes).unwrap())
            .unwrap();

    // CertificateVerify を検証
    let parsed_cv_sig = Signature::from_der(
        &B64.decode(extract_field(&decrypted_msg, "certVerifySignature"))
            .unwrap(),
    )
    .unwrap();
    let transcript_bytes = B64
        .decode(extract_field(&decrypted_msg, "handshakeTranscript"))
        .unwrap();
    server_pub_from_cert
        .verify(&transcript_bytes, &parsed_cv_sig)
        .expect("CertificateVerify検証失敗！");
    println!("CertificateVerify検証: OK");

    // ─── ⑥ AES-GCM アプリケーションデータ ─────────────────────────

    let message = b"GET / HTTP/1.1\r\nHost: example.com";
    let iv2: [u8; 12] = rand::random();
    let nonce2 = Nonce::from(iv2);

    let cipher_enc = Aes256Gcm::new_from_slice(&client_session_key).unwrap();
    let encrypted = cipher_enc.encrypt(&nonce2, message.as_ref()).unwrap();
    println!("暗号文: {}...", short(&B64.encode(&encrypted)));

    let cipher_dec = Aes256Gcm::new_from_slice(&server_session_key).unwrap();
    let decrypted = cipher_dec.decrypt(&nonce2, encrypted.as_slice()).unwrap();
    println!("復号: {}", String::from_utf8(decrypted).unwrap());
}

fn extract_field(json: &str, key: &str) -> String {
    let pattern = format!(r#""{}":""#, key);
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        if let Some(end) = rest.find('"') {
            return rest[..end].to_string();
        }
    }
    String::new()
}

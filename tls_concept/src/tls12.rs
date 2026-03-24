// ============================================================
// TLS 1.2 コンセプトコード (ECDHE-ECDSA, Rust)
// ============================================================
// cargo run --bin tls12

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
    elliptic_curve::sec1::ToEncodedPoint,
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
    let server_cert_pub = VerifyingKey::from(&server_cert_key);

    let server_pub_hex = hex::encode(server_cert_pub.to_encoded_point(false).as_bytes());
    let cert_body = format!(
        r#"{{"subject":"example.com","issuer":"My CA","publicKey":"{}","notBefore":"2025-01-01","notAfter":"2026-01-01"}}"#,
        server_pub_hex
    );
    let ca_signature: Signature = ca_signing_key.sign(cert_body.as_bytes());
    println!("CA署名: {}...", short(&B64.encode(ca_signature.to_der())));

    // ─── ② 1-RTT目: ServerHello + Certificate + ServerKeyExchange ──
    //  TLS 1.2: 証明書は平文で送られる（盗聴者にも見える）

    let server_ecdh = EphemeralSecret::random(&mut OsRng);
    let server_ecdh_pub = PublicKey::from(&server_ecdh);
    let server_ecdh_pub_hex = hex::encode(server_ecdh_pub.to_encoded_point(false).as_bytes());

    // ServerKeyExchange: ECDH公開鍵に証明書の秘密鍵で署名
    let server_kex_msg = format!(r#"{{"ecdhPublicKey":"{}"}}"#, server_ecdh_pub_hex);
    let server_kex_sig: Signature = server_cert_key.sign(server_kex_msg.as_bytes());

    // 平文で送信
    let server_hello_flight = format!(
        r#"{{"certificate":{},"caSignature":"{}","serverKeyExchange":{},"serverKeyExchangeSig":"{}"}}"#,
        cert_body,
        B64.encode(ca_signature.to_der()),
        server_kex_msg,
        B64.encode(server_kex_sig.to_der()),
    );
    println!("サーバー → クライアント: Certificate (平文) を送信");

    // ─── ③ クライアントが証明書を検証（平文の状態で）───────────────

    // CA公開鍵で証明書を検証
    let ca_verifying = VerifyingKey::from(&ca_signing_key);
    let parsed_ca_sig = Signature::from_der(
        &B64.decode(extract_field(&server_hello_flight, "caSignature"))
            .unwrap(),
    )
    .unwrap();
    ca_verifying
        .verify(cert_body.as_bytes(), &parsed_ca_sig)
        .expect("証明書検証失敗！");
    println!("証明書検証 (CA署名): OK");

    // 証明書からサーバーの公開鍵を取り出す
    let pub_hex = extract_field(&cert_body, "publicKey");
    let pub_bytes = hex::decode(&pub_hex).unwrap();
    let server_pub_from_cert =
        VerifyingKey::from_encoded_point(&p256::EncodedPoint::from_bytes(&pub_bytes).unwrap())
            .unwrap();

    // ServerKeyExchange の署名を検証
    let parsed_kex_sig = Signature::from_der(
        &B64.decode(extract_field(&server_hello_flight, "serverKeyExchangeSig"))
            .unwrap(),
    )
    .unwrap();
    server_pub_from_cert
        .verify(server_kex_msg.as_bytes(), &parsed_kex_sig)
        .expect("ServerKeyExchange検証失敗！");
    println!("ServerKeyExchange検証: OK");

    // ─── ④ 2-RTT目: ClientKeyExchange ────────────────────────────

    let client_ecdh = EphemeralSecret::random(&mut OsRng);
    let client_ecdh_pub = PublicKey::from(&client_ecdh);

    // ServerKeyExchange からサーバーの ECDH 公開鍵を取り出す
    let kex_pub_hex = extract_field(&server_kex_msg, "ecdhPublicKey");
    let kex_pub_bytes = hex::decode(&kex_pub_hex).unwrap();
    let server_ecdh_pub_from_msg = PublicKey::from_sec1_bytes(&kex_pub_bytes).unwrap();

    // ─── ⑤ セッション鍵の導出（証明書検証の後）─────────────────────
    // TLS 1.2: 証明書検証 → セッション鍵確立の順（1.3と逆）

    let client_shared = client_ecdh.diffie_hellman(&server_ecdh_pub_from_msg);
    let server_shared = server_ecdh.diffie_hellman(&client_ecdh_pub);

    let client_session_key =
        derive_session_key(client_shared.raw_secret_bytes(), b"tls12-session-key");
    let server_session_key =
        derive_session_key(server_shared.raw_secret_bytes(), b"tls12-session-key");

    assert_eq!(
        client_session_key, server_session_key,
        "セッション鍵が一致しない！"
    );
    println!(
        "セッション鍵確立（証明書検証の後）: {}...",
        hex::encode(&client_session_key[..12])
    );

    // ─── ⑥ AES-GCM アプリケーションデータ ─────────────────────────

    let message = b"GET / HTTP/1.1\r\nHost: example.com";
    let iv: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&iv);

    let cipher_enc = Aes256Gcm::new_from_slice(&client_session_key).unwrap();
    let encrypted = cipher_enc.encrypt(nonce, message.as_ref()).unwrap();
    println!("暗号文: {}...", short(&B64.encode(&encrypted)));

    let cipher_dec = Aes256Gcm::new_from_slice(&server_session_key).unwrap();
    let decrypted = cipher_dec.decrypt(nonce, encrypted.as_slice()).unwrap();
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

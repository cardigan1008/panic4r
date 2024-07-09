fn main() {
    use rustls::{ClientConfig, ClientConnection, ConnectionTrafficSecrets, ExtractedSecrets, RootCertStore, Stream, crypto::{ring as rustls_ring, CryptoProvider}};

    let hostname = "www.bing.com";

    let cipher_suites = vec![
        rustls_ring::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
        rustls_ring::cipher_suite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
    ];

    let crypto_provider = CryptoProvider {
        cipher_suites,
        ..rustls_ring::default_provider()
    };
    crypto_provider.install_default().unwrap();

    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_owned(),
    };

    let mut config =
        ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
        .with_root_certificates(root_store)
        .with_no_client_auth();
    config.enable_secret_extraction = true;
    let config = std::sync::Arc::new(config);

    let name = hostname.try_into().unwrap();

    let mut conn = ClientConnection::new(config, name).unwrap();
    let mut sock = std::net::TcpStream::connect("www.bing.com:443").unwrap();
    let mut tls = Stream::new(&mut conn, &mut sock);
    std::io::Write::write_all(&mut tls, b"\r\n").unwrap();
    let protocol_version = conn.protocol_version().unwrap();
    println!(
        "using {protocol_version:?} {:?}",
        conn.negotiated_cipher_suite(),
    );
    let ExtractedSecrets { tx, rx } = conn.dangerous_extract_secrets().unwrap();
    match tx.1 {
        ConnectionTrafficSecrets::Aes256Gcm { key, .. } => {
            assert_eq!(key.as_ref().len(), 32);
            println!("tx is ConnectionTrafficSecrets::Aes256Gcm with key {:?}B key", key.as_ref().len());
        },

        ConnectionTrafficSecrets::Aes128Gcm { key, .. } => panic!("tx is ConnectionTrafficSecrets::Aes128Gcm with key {:?}B key", key.as_ref().len()),

        ConnectionTrafficSecrets::Chacha20Poly1305 { key, .. } => panic!("tx is ConnectionTrafficSecrets::Chacha20Poly1305 with key {:?}B key", key.as_ref().len()),

        _ => unreachable!(),
    }
    match rx.1 {
        ConnectionTrafficSecrets::Aes256Gcm { key, .. } => {
            assert_eq!(key.as_ref().len(), 32);
            println!("rx is ConnectionTrafficSecrets::Aes256Gcm with key {:?}B key", key.as_ref().len());
        },

        ConnectionTrafficSecrets::Aes128Gcm { key, .. } => panic!("rx is ConnectionTrafficSecrets::Aes128Gcm with key {:?}B key", key.as_ref().len()),

        ConnectionTrafficSecrets::Chacha20Poly1305 { key, .. } => panic!("rx is ConnectionTrafficSecrets::Chacha20Poly1305 with key {:?}B key", key.as_ref().len()),

        _ => unreachable!(),
    }
}
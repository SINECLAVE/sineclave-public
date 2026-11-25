//! sineclave-pq-bench — micro-benchmarks of standard primitives (no Sineclave protocol code).
//! Usage: sineclave-pq-bench [--json out.json] [--quick]
use std::sync::Arc;
use std::time::{Duration, Instant};

struct Rez { grupa: &'static str, ime: String, us: f64, ops: f64, napomena: String }

/// Median time per operation: `serije` batches of `n` calls each.
fn mjeri<F: FnMut()>(serije: usize, n: usize, mut f: F) -> f64 {
    for _ in 0..n.max(3) { f(); }
    let mut t: Vec<f64> = (0..serije).map(|_| {
        let s = Instant::now();
        for _ in 0..n { f(); }
        s.elapsed().as_secs_f64() * 1e6 / n as f64
    }).collect();
    t.sort_by(|a, b| a.partial_cmp(b).unwrap());
    t[t.len() / 2]
}

fn dodaj(r: &mut Vec<Rez>, grupa: &'static str, ime: &str, us: f64, napomena: &str) {
    println!("{:<10} {:<44} {:>11.2} µs {:>11.0} /s  {}", grupa, ime, us, 1e6 / us, napomena);
    r.push(Rez { grupa, ime: ime.into(), us, ops: 1e6 / us, napomena: napomena.into() });
}

fn ml_kem(r: &mut Vec<Rez>, s: usize) {
    use fips203::ml_kem_768;
    use fips203::traits::{Decaps, Encaps, KeyGen};
    let (ek, dk) = ml_kem_768::KG::try_keygen().unwrap();
    dodaj(r, "ML-KEM", "ML-KEM-768 keygen (fips203, pure Rust)", mjeri(s, 20, || { let _ = ml_kem_768::KG::try_keygen().unwrap(); }), "FIPS 203");
    dodaj(r, "ML-KEM", "ML-KEM-768 encaps (fips203)", mjeri(s, 20, || { let _ = ek.try_encaps().unwrap(); }), "");
    let (_, ct) = ek.try_encaps().unwrap();
    dodaj(r, "ML-KEM", "ML-KEM-768 decaps (fips203)", mjeri(s, 20, || { let _ = dk.try_decaps(&ct).unwrap(); }), "");

    use aws_lc_rs::kem::{DecapsulationKey, ML_KEM_768};
    let dk2 = DecapsulationKey::generate(&ML_KEM_768).unwrap();
    let ek2 = dk2.encapsulation_key().unwrap();
    dodaj(r, "ML-KEM", "ML-KEM-768 keygen (AWS-LC)", mjeri(s, 20, || { let _ = DecapsulationKey::generate(&ML_KEM_768).unwrap(); }), "");
    dodaj(r, "ML-KEM", "ML-KEM-768 encaps (AWS-LC)", mjeri(s, 20, || { let _ = ek2.encapsulate().unwrap(); }), "");
    let (ct2, _) = ek2.encapsulate().unwrap();
    let ctb = ct2.as_ref().to_vec();
    dodaj(r, "ML-KEM", "ML-KEM-768 decaps (AWS-LC)", mjeri(s, 20, || { let _ = dk2.decapsulate(ctb.as_slice().into()).unwrap(); }), "");
}

macro_rules! ml_dsa {
    ($r:expr, $s:expr, $m:ident, $naziv:expr) => {{
        use fips204::$m;
        use fips204::traits::{Signer, Verifier};
        let poruka = [7u8; 64];
        let (pk, sk) = $m::try_keygen().unwrap();
        dodaj($r, "ML-DSA", concat!($naziv, " keygen (fips204)"), mjeri($s, 10, || { let _ = $m::try_keygen().unwrap(); }), "FIPS 204");
        dodaj($r, "ML-DSA", concat!($naziv, " sign, 64 B (fips204)"), mjeri($s, 10, || { let _ = sk.try_sign(&poruka, b"").unwrap(); }), "hedged");
        let sig = sk.try_sign(&poruka, b"").unwrap();
        dodaj($r, "ML-DSA", concat!($naziv, " verify (fips204)"), mjeri($s, 20, || { assert!(pk.verify(&poruka, &sig, b"")); }), "");
    }};
}

fn simetricno(r: &mut Vec<Rez>, s: usize) {
    use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
    let k = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &[1u8; 32]).unwrap());
    for vel in [1024usize, 65536] {
        let mut buf = vec![0u8; vel];
        let us = mjeri(s, 20, || {
            let n = Nonce::assume_unique_for_key([0u8; 12]);
            let _ = k.seal_in_place_separate_tag(n, Aad::empty(), &mut buf).unwrap();
        });
        dodaj(r, "AEAD", &format!("AES-256-GCM seal {} KiB (AWS-LC)", vel / 1024), us, &format!("{:.0} MB/s", vel as f64 / us));
    }
    let mb = vec![3u8; 1 << 20];
    let us = mjeri(s, 3, || { let _ = aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA256, &mb); });
    dodaj(r, "Hash", "SHA-256, 1 MiB (AWS-LC)", us, &format!("{:.0} MB/s", (1 << 20) as f64 / us));
    use sha3::Digest;
    let us = mjeri(s, 3, || { let _ = sha3::Sha3_256::digest(&mb); });
    dodaj(r, "Hash", "SHA3-256, 1 MiB (sha3, pure Rust)", us, &format!("{:.0} MB/s", (1 << 20) as f64 / us));
}

fn tls(r: &mut Vec<Rez>, s: usize) {
    use rustls::crypto::aws_lc_rs::{self as p, kx_group};
    use rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
    let ck = rcgen::generate_simple_self_signed(vec!["bench.local".into()]).unwrap();
    let cert = ck.cert.der().clone();
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(ck.key_pair.serialize_der()));
    let mut korijeni = rustls::RootCertStore::empty();
    korijeni.add(cert.clone()).unwrap();
    #[cfg(feature = "fips")]
    let osnova = || rustls::crypto::default_fips_provider();
    #[cfg(not(feature = "fips"))]
    let osnova = || p::default_provider();
    let mut grupe: Vec<(&str, &'static dyn rustls::crypto::SupportedKxGroup, &str)> = vec![
        ("X25519MLKEM768 (hybrid PQ)", kx_group::X25519MLKEM768, "post-quantum"),
        ("SECP256R1MLKEM768 (hybrid PQ)", kx_group::SECP256R1MLKEM768, "post-quantum"),
        ("SECP256R1 (classical)", kx_group::SECP256R1, "not post-quantum"),
    ];
    #[cfg(not(feature = "fips"))]
    grupe.push(("X25519 (classical)", kx_group::X25519, "not post-quantum"));
    for (ime, g, nap) in grupe {
        let prov = Arc::new(rustls::crypto::CryptoProvider { kx_groups: vec![g], ..osnova() });
        let sc = Arc::new(rustls::ServerConfig::builder_with_provider(prov.clone())
            .with_protocol_versions(&[&rustls::version::TLS13]).unwrap()
            .with_no_client_auth().with_single_cert(vec![cert.clone()], key.clone_key()).unwrap());
        let cc = Arc::new(rustls::ClientConfig::builder_with_provider(prov)
            .with_protocol_versions(&[&rustls::version::TLS13]).unwrap()
            .with_root_certificates(korijeni.clone()).with_no_client_auth());
        let mut bajtova = 0usize;
        let us = mjeri(s, 5, || {
            let mut c = rustls::ClientConnection::new(cc.clone(), ServerName::try_from("bench.local").unwrap()).unwrap();
            let mut sv = rustls::ServerConnection::new(sc.clone()).unwrap();
            let mut ukupno = 0usize;
            while c.is_handshaking() || sv.is_handshaking() {
                let mut b = Vec::new();
                while c.wants_write() { c.write_tls(&mut b).unwrap(); }
                ukupno += b.len();
                if !b.is_empty() { sv.read_tls(&mut &b[..]).unwrap(); sv.process_new_packets().unwrap(); }
                let mut b = Vec::new();
                while sv.wants_write() { sv.write_tls(&mut b).unwrap(); }
                ukupno += b.len();
                if !b.is_empty() { c.read_tls(&mut &b[..]).unwrap(); c.process_new_packets().unwrap(); }
            }
            bajtova = ukupno;
        });
        dodaj(r, "TLS 1.3", &format!("full handshake, {ime}"), us, &format!("{nap}; {bajtova} B on the wire, both sides on one core"));
    }
}

fn sustav() -> serde_json::Value {
    let cpu = std::fs::read_to_string("/proc/cpuinfo").ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("model name")).map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string()))
        .unwrap_or_else(|| "unknown".into());
    let jezgre = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0);
    serde_json::json!({
        "cpu": cpu, "logical_cpus": jezgre, "os": std::env::consts::OS, "arch": std::env::consts::ARCH,
        "fips_build": cfg!(feature = "fips"),
        "fips_mode": cfg!(feature = "fips") && aws_lc_rs::try_fips_mode().is_ok(),
    })
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let brzo = a.iter().any(|x| x == "--quick");
    let json = a.iter().position(|x| x == "--json").and_then(|i| a.get(i + 1)).cloned();
    let s = if brzo { 15 } else { 101 };
    let sys = sustav();
    println!("sineclave-pq-bench {} · {} · {} logical CPUs · FIPS build: {}", env!("CARGO_PKG_VERSION"),
        sys["cpu"].as_str().unwrap_or(""), sys["logical_cpus"], sys["fips_mode"]);
    println!("median time per operation, single thread\n");
    let start = Instant::now();
    let mut r = Vec::new();
    ml_kem(&mut r, s);
    ml_dsa!(&mut r, s, ml_dsa_44, "ML-DSA-44");
    ml_dsa!(&mut r, s, ml_dsa_65, "ML-DSA-65");
    simetricno(&mut r, s);
    tls(&mut r, s);
    let trajanje: Duration = start.elapsed();
    println!("\ndone in {:.1} s", trajanje.as_secs_f64());
    if let Some(put) = json {
        let v = serde_json::json!({
            "tool": "sineclave-pq-bench", "version": env!("CARGO_PKG_VERSION"), "system": sys,
            "method": "median of batches, single thread, warm-up first",
            "results": r.iter().map(|x| serde_json::json!({"group": x.grupa, "name": x.ime, "us_per_op": (x.us * 100.0).round() / 100.0, "ops_per_s": x.ops.round(), "note": x.napomena})).collect::<Vec<_>>()
        });
        std::fs::write(&put, serde_json::to_string_pretty(&v).unwrap()).unwrap();
        println!("saved {put}");
    }
}

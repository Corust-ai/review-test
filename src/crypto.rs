/// **WARNING: This is NOT encryption.** XOR with a repeating key is trivially
/// breakable and must NOT be used to protect data at rest or in transit.
/// This function provides only simple, reversible obfuscation (e.g. for
/// light data masking where security is not a concern). For real encryption
/// use a proper authenticated encryption scheme such as AES-GCM.
pub fn xor_obfuscate(data: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return data.to_vec();
    }
    data.iter().enumerate().map(|(i, b)| b ^ key[i % key.len()]).collect()
}

/// Hash a password using PBKDF2-HMAC-SHA256 with a random 16-byte salt and
/// 600 000 iterations.  The output format is `{hex_salt}:{hex_derived_key}`.
pub fn hash_password(password: &str) -> String {
    let salt = generate_salt();
    let dk = pbkdf2_hmac_sha256(password.as_bytes(), &salt, PBKDF2_ITERATIONS);
    format!("{}:{}", hex_encode(&salt), hex_encode(&dk))
}

/// Verify a password against a hash produced by [`hash_password`].
pub fn verify_password(password: &str, hash: &str) -> bool {
    let Some((salt_hex, dk_hex)) = hash.split_once(':') else { return false; };
    let Some(salt) = hex_decode(salt_hex) else { return false; };
    let Some(expected) = hex_decode(dk_hex) else { return false; };
    let dk = pbkdf2_hmac_sha256(password.as_bytes(), &salt, PBKDF2_ITERATIONS);
    constant_time_eq(&dk, &expected)
}

/// Number of PBKDF2 iterations – intentionally high to slow brute-force.
const PBKDF2_ITERATIONS: u32 = 600_000;

/// PBKDF2-HMAC-SHA256 (RFC 8018) producing a 32-byte derived key.
fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
    // For a single 32-byte block, block_index = 1.
    let mut msg = Vec::with_capacity(salt.len() + 4);
    msg.extend_from_slice(salt);
    msg.extend_from_slice(&1u32.to_be_bytes());

    let mut u = hmac_sha256(password, &msg); // U_1
    let mut dk = u;

    for _ in 1..iterations {
        u = hmac_sha256(password, &u); // U_i
        for (d, ui) in dk.iter_mut().zip(u.iter()) {
            *d ^= ui;
        }
    }
    dk
}

/// Generate a 16-byte random salt using OS randomness.
fn generate_salt() -> [u8; 16] {
    let mut buf = [0u8; 16];
    // Read from /dev/urandom; fall back to a time-seeded value if unavailable.
    #[cfg(unix)]
    {
        use std::io::Read;
        if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
            let _ = f.read_exact(&mut buf);
            return buf;
        }
    }
    // Fallback: seed from high-resolution time (not ideal but better than nothing).
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let hash = sha256(&t.to_le_bytes());
    buf.copy_from_slice(&hash[..16]);
    buf
}

/// HMAC-SHA256 based token generation.
/// Token format: "{user_id}-{timestamp}-{hex_signature}"
pub fn generate_token(user_id: u64) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let payload = format!("{}-{}", user_id, timestamp);
    let sig = hmac_sha256(TOKEN_SECRET, payload.as_bytes());
    format!("{}-{}", payload, hex_encode(&sig))
}

/// Verify an HMAC-signed token and return the user_id if valid.
pub fn verify_token(token: &str) -> Option<u64> {
    // token = "{user_id}-{timestamp}-{hex_sig}"
    let (payload, hex_sig) = token.rsplit_once('-')?;

    let expected_sig = hmac_sha256(TOKEN_SECRET, payload.as_bytes());
    let provided_sig = hex_decode(hex_sig)?;

    if !constant_time_eq(&expected_sig, &provided_sig) {
        return None;
    }

    // payload = "{user_id}-{timestamp}"
    let user_id_str = payload.split('-').next()?;
    user_id_str.parse().ok()
}

// ---- internal HMAC-SHA256 helpers (no external deps) ----

/// Secret key used for token signing. In production this should come from a
/// secure configuration source; embedded here to satisfy the no-new-deps
/// constraint while still making tokens non-forgeable without the secret.
const TOKEN_SECRET: &[u8] = b"change-me-to-a-real-secret-in-production";

/// Minimal SHA-256 implementation (FIPS 180-4).
fn sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
        0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
        0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
        0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
        0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
        0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
        0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
        0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    // pre-processing: padding
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // process each 512-bit block
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4*i], chunk[4*i+1], chunk[4*i+2], chunk[4*i+3]]);
        }
        for i in 16..64 {
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        let (mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut hh) =
            (h[0],h[1],h[2],h[3],h[4],h[5],h[6],h[7]);

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g; g = f; f = e; e = d.wrapping_add(temp1);
            d = c; c = b; b = a; a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c); h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e); h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g); h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 32];
    for (i, val) in h.iter().enumerate() {
        out[4*i..4*i+4].copy_from_slice(&val.to_be_bytes());
    }
    out
}

/// HMAC-SHA256 (RFC 2104).
fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    let block_size = 64;
    let mut padded_key = [0u8; 64];

    if key.len() > block_size {
        let hashed = sha256(key);
        padded_key[..32].copy_from_slice(&hashed);
    } else {
        padded_key[..key.len()].copy_from_slice(key);
    }

    let mut i_key_pad = [0x36u8; 64];
    let mut o_key_pad = [0x5cu8; 64];
    for i in 0..64 {
        i_key_pad[i] ^= padded_key[i];
        o_key_pad[i] ^= padded_key[i];
    }

    let mut inner = Vec::with_capacity(64 + message.len());
    inner.extend_from_slice(&i_key_pad);
    inner.extend_from_slice(message);
    let inner_hash = sha256(&inner);

    let mut outer = Vec::with_capacity(64 + 32);
    outer.extend_from_slice(&o_key_pad);
    outer.extend_from_slice(&inner_hash);
    sha256(&outer)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) { return None; }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16).ok())
        .collect()
}

/// Constant-time comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
// re-trigger
// test tools 1775972109
// trigger 1775972168
// test timeout 1775972506

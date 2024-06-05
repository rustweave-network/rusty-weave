use crate::Hash;

#[derive(Clone)]
pub struct PowHash(blake3::Hasher);

#[derive(Clone)]
pub struct KHeavyHash;

impl PowHash {
    #[inline]
    pub fn new(pre_pow_hash: Hash, timestamp: u64) -> Self {
        let mut hasher = blake3::Hasher::new();

        hasher.update(&pre_pow_hash.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&[0u8; 32]);

        Self(hasher)
    }

    #[inline(always)]
    pub fn finalize_with_nonce(mut self, nonce: u64) -> Hash {
        self.0.update(&nonce.to_le_bytes());
        Hash(*self.0.finalize().as_bytes())
    }
}

impl KHeavyHash {
    #[inline]
    pub fn hash(in_hash: Hash) -> Hash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&in_hash.as_bytes());
        Hash(*hasher.finalize().as_bytes())
    }
}

mod keccak256 {
    #[cfg(any(not(target_arch = "x86_64"), feature = "no-asm", target_os = "windows"))]
    #[inline(always)]
    pub(super) fn f1600(state: &mut [u64; 25]) {
        keccak::f1600(state);
    }

    #[cfg(all(target_arch = "x86_64", not(feature = "no-asm"), not(target_os = "windows")))]
    #[inline(always)]
    pub(super) fn f1600(state: &mut [u64; 25]) {
        extern "C" {
            fn KeccakF1600(state: &mut [u64; 25]);
        }
        unsafe { KeccakF1600(state) }
    }
}

#[cfg(test)]
mod tests {
    use super::{KHeavyHash, PowHash};
    use crate::Hash;
    use sha3::digest::{ExtendableOutput, Update, XofReader};
    use sha3::{CShake256, CShake256Core};

    const HEAVY_HASH_DOMAIN: &[u8] = b"HeavyHash";

    #[test]
    fn test_pow_hash() {
        let timestamp: u64 = 5435345234;
        let nonce: u64 = 432432432;
        let pre_pow_hash = Hash([42; 32]);
        let hasher = PowHash::new(pre_pow_hash, timestamp);
        let hash1 = hasher.finalize_with_nonce(nonce);

        let mut hasher = blake3::Hasher::new();
        hasher.update(&pre_pow_hash.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&[0u8; 32]);
        hasher.update(&nonce.to_le_bytes());

        let hash2 = Hash(*hasher.finalize().as_bytes());
        assert_eq!(hash2, hash1);
    }

    #[test]
    fn test_heavy_hash() {
        let val = Hash([42; 32]);
        let hash1 = KHeavyHash::hash(val);

        let hasher = CShake256::from_core(CShake256Core::new(HEAVY_HASH_DOMAIN)).chain(val.0);
        let mut hash2 = [0u8; 32];
        hasher.finalize_xof().read(&mut hash2);
        assert_eq!(Hash(hash2), hash1);
    }
}

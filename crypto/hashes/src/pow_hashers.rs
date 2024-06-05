use crate::Hash;

#[derive(Clone)]
pub struct PowHash(blake3::Hasher);

#[derive(Clone)]
pub struct KHeavyHash;

impl PowHash {
    // The initial state of `cSHAKE256("ProofOfWorkHash")`
    // [10] -> 1123092876221303310 ^ 0x04(padding byte) = 1123092876221303306
    // [16] -> 10306167911662716186 ^ 0x8000000000000000(final padding) = 1082795874807940378
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
    // The initial state of `cSHAKE256("HeavyHash")`
    // [4] -> 16654558671554924254 ^ 0x04(padding byte) = 16654558671554924250
    // [16] -> 9793466274154320918 ^ 0x8000000000000000(final padding) = 570094237299545110
    #[rustfmt::skip]
    const INITIAL_STATE: [u64; 25] = [
        4239941492252378377, 8746723911537738262, 8796936657246353646, 1272090201925444760, 16654558671554924250,
        8270816933120786537, 13907396207649043898, 6782861118970774626, 9239690602118867528, 11582319943599406348,
        17596056728278508070, 15212962468105129023, 7812475424661425213, 3370482334374859748, 5690099369266491460,
        8596393687355028144, 570094237299545110, 9119540418498120711, 16901969272480492857, 13372017233735502424,
        14372891883993151831, 5171152063242093102, 10573107899694386186, 6096431547456407061, 1592359455985097269,
    ];
    #[inline]
    pub fn hash(in_hash: Hash) -> Hash {
        let mut state = Self::INITIAL_STATE;
        for (pre_pow_word, state_word) in in_hash.iter_le_u64().zip(state.iter_mut()) {
            *state_word ^= pre_pow_word;
        }
        keccak256::f1600(&mut state);
        Hash::from_le_u64(state[..4].try_into().unwrap())
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

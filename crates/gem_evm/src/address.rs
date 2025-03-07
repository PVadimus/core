/// Taken from https://github.com/alloy-rs/core/blob/main/crates/primitives/src/bits/address.rs
use gem_hash::keccak::keccak256;
use std::str::FromStr;

/// Error type for address checksum validation.
#[derive(Debug, Copy, Clone)]
pub enum AddressError {
    /// Error while decoding hex.
    Hex(hex::FromHexError),

    /// Invalid ERC-55 checksum.
    InvalidChecksum,
}

impl std::error::Error for AddressError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Hex(err) => Some(err),
            Self::InvalidChecksum => None,
        }
    }
}

impl std::fmt::Display for AddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hex(err) => err.fmt(f),
            Self::InvalidChecksum => f.write_str("Bad address checksum"),
        }
    }
}

#[derive(Default)]
pub struct EthereumAddress {
    pub bytes: Vec<u8>,
}

impl FromStr for EthereumAddress {
    type Err = AddressError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut address = str.to_string();
        if str.starts_with("0x") {
            address = address.replace("0x", "");
        }
        let result = hex::decode(&address);
        match result {
            Ok(bytes) => {
                if bytes.len() != Self::LEN {
                    return Err(AddressError::InvalidChecksum);
                }
                Ok(Self { bytes })
            }
            Err(err) => Err(AddressError::Hex(err)),
        }
    }
}

impl EthereumAddress {
    pub const LEN: usize = 20;

    pub fn parse(str: &str) -> Option<EthereumAddress> {
        Self::from_str(str).ok()
    }

    pub fn to_checksum(&self) -> String {
        self.to_checksum_raw(&mut [0; 42], None).to_string()
    }

    pub fn to_checksum_raw<'a>(&self, buf: &'a mut [u8], chain_id: Option<u64>) -> &'a str {
        assert_eq!(buf.len(), 42, "addr_buf must be 42 bytes long");
        buf[0] = b'0';
        buf[1] = b'x';
        hex::encode_to_slice(&self.bytes, &mut buf[2..]).unwrap();

        let mut storage;
        let to_hash = match chain_id {
            Some(chain_id) => {
                // A decimal `u64` string is at most 20 bytes long
                storage = [0u8; 2 + 40 + 20];

                // Format the `chain_id` into a stack-allocated buffer using `itoa`
                let mut temp = itoa::Buffer::new();
                let prefix_str = temp.format(chain_id);
                let prefix_len = prefix_str.len();
                debug_assert!(prefix_len <= 20);
                let len = 2 + 40 + prefix_len;

                // SAFETY: prefix_len <= 20; len <= 62; storage.len() == 62
                unsafe {
                    storage.get_unchecked_mut(..prefix_len).copy_from_slice(prefix_str.as_bytes());
                    storage.get_unchecked_mut(prefix_len..len).copy_from_slice(buf);
                    storage.get_unchecked(..len)
                }
            }
            None => &buf[2..],
        };
        let hash = keccak256(to_hash);
        let mut hash_hex = [0u8; 64];
        hex::encode_to_slice(hash, &mut hash_hex).unwrap();

        // generates significantly less code than zipping the two arrays, or
        // `.into_iter()`
        for (i, x) in hash_hex.iter().enumerate().take(40) {
            if *x >= b'8' {
                // SAFETY: `addr_buf` is 42 bytes long, `2..42` is always in range
                unsafe { buf.get_unchecked_mut(i + 2).make_ascii_uppercase() };
            }
        }

        // SAFETY: All bytes in the buffer are valid UTF-8
        unsafe { std::str::from_utf8_unchecked(buf) }
    }
}

#[cfg(test)]
mod tests {
    use super::EthereumAddress;

    #[test]
    fn checksum() {
        let eth1 = "0x000000000022d473030f116ddee9f6b43ac78ba3";
        let addr1 = EthereumAddress::parse(eth1).unwrap();

        assert_eq!(addr1.to_checksum(), "0x000000000022D473030F116dDEE9F6B43aC78BA3");
    }

    // https://eips.ethereum.org/EIPS/eip-1191
    #[test]
    fn checksum_chain_id() {
        let eth_mainnet = [
            "0x27b1fdb04752bbc536007a920d24acb045561c26",
            "0x3599689E6292b81B2d85451025146515070129Bb",
            "0x42712D45473476b98452f434e72461577D686318",
            "0x52908400098527886E0F7030069857D2E4169EE7",
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
            "0x6549f4939460DE12611948b3f82b88C3C8975323",
            "0x66f9664f97F2b50F62D13eA064982f936dE76657",
            "0x8617E340B3D01FA5F11F306F4090FD50E238070D",
            "0x88021160C5C792225E4E5452585947470010289D",
            "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
            "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
            "0xde709f2102306220921060314715629080e2fb77",
            "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
        ];
        let rsk_mainnet = [
            "0x27b1FdB04752BBc536007A920D24ACB045561c26",
            "0x3599689E6292B81B2D85451025146515070129Bb",
            "0x42712D45473476B98452f434E72461577d686318",
            "0x52908400098527886E0F7030069857D2E4169ee7",
            "0x5aaEB6053f3e94c9b9a09f33669435E7ef1bEAeD",
            "0x6549F4939460DE12611948B3F82B88C3C8975323",
            "0x66F9664f97f2B50F62d13EA064982F936de76657",
            "0x8617E340b3D01Fa5f11f306f4090fd50E238070D",
            "0x88021160c5C792225E4E5452585947470010289d",
            "0xD1220A0Cf47c7B9BE7a2e6ba89F429762E7B9adB",
            "0xDBF03B407c01E7CD3cBea99509D93F8Dddc8C6FB",
            "0xDe709F2102306220921060314715629080e2FB77",
            "0xFb6916095cA1Df60bb79ce92cE3EA74c37c5d359",
        ];
        let rsk_testnet = [
            "0x27B1FdB04752BbC536007a920D24acB045561C26",
            "0x3599689e6292b81b2D85451025146515070129Bb",
            "0x42712D45473476B98452F434E72461577D686318",
            "0x52908400098527886E0F7030069857D2e4169EE7",
            "0x5aAeb6053F3e94c9b9A09F33669435E7EF1BEaEd",
            "0x6549f4939460dE12611948b3f82b88C3c8975323",
            "0x66f9664F97F2b50f62d13eA064982F936DE76657",
            "0x8617e340b3D01fa5F11f306F4090Fd50e238070d",
            "0x88021160c5C792225E4E5452585947470010289d",
            "0xd1220a0CF47c7B9Be7A2E6Ba89f429762E7b9adB",
            "0xdbF03B407C01E7cd3cbEa99509D93f8dDDc8C6fB",
            "0xDE709F2102306220921060314715629080e2Fb77",
            "0xFb6916095CA1dF60bb79CE92ce3Ea74C37c5D359",
        ];
        for (addresses, chain_id) in [(eth_mainnet, 1), (rsk_mainnet, 30), (rsk_testnet, 31)] {
            // EIP-1191 test cases treat mainnet as "not adopted"
            let id = if chain_id == 1 { None } else { Some(chain_id) };
            for addr in addresses {
                let parsed = EthereumAddress::parse(addr).unwrap();
                assert_eq!(parsed.to_checksum_raw(&mut [0; 42], id), addr);
            }
        }
    }
}

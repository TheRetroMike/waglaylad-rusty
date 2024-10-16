use crate::{block::Block, header::Header, subnets::SUBNETWORK_ID_COINBASE, tx::Transaction};
use waglayla_hashes::{Hash, ZERO_HASH};
use waglayla_muhash::{Blake2Hash, EMPTY_MUHASH};

/// The constants uniquely representing the genesis block
#[derive(Clone, Debug)]
pub struct GenesisBlock {
    pub hash: Hash,
    pub version: u16,
    pub hash_merkle_root: Hash,
    pub utxo_commitment: Blake2Hash,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
    pub daa_score: u64,
    pub coinbase_payload: &'static [u8],
}

impl GenesisBlock {
    pub fn build_genesis_transactions(&self) -> Vec<Transaction> {
        vec![Transaction::new(0, Vec::new(), Vec::new(), 0, SUBNETWORK_ID_COINBASE, 0, self.coinbase_payload.to_vec())]
    }
}

impl From<&GenesisBlock> for Header {
    fn from(genesis: &GenesisBlock) -> Self {
        Header::new_finalized(
            genesis.version,
            Vec::new(),
            genesis.hash_merkle_root,
            ZERO_HASH,
            genesis.utxo_commitment,
            genesis.timestamp,
            genesis.bits,
            genesis.nonce,
            genesis.daa_score,
            0.into(),
            0,
            ZERO_HASH,
        )
    }
}

impl From<&GenesisBlock> for Block {
    fn from(genesis: &GenesisBlock) -> Self {
        Block::new(genesis.into(), genesis.build_genesis_transactions())
    }
}

impl From<(&Header, &'static [u8])> for GenesisBlock {
    fn from((header, payload): (&Header, &'static [u8])) -> Self {
        Self {
            hash: header.hash,
            version: header.version,
            hash_merkle_root: header.hash_merkle_root,
            utxo_commitment: header.utxo_commitment,
            timestamp: header.timestamp,
            bits: header.bits,
            nonce: header.nonce,
            daa_score: header.daa_score,
            coinbase_payload: payload,
        }
    }
}

/// The genesis block of the block-DAG which serves as the public transaction ledger for the main network.
pub const GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x91, 0x79, 0xa1, 0xc8, 0xdf, 0xd2, 0x90, 0xd6, 
        0x91, 0x4c, 0x0f, 0x1d, 0xae, 0x0e, 0xc4, 0x1a, 
        0xa2, 0x49, 0x5d, 0xe2, 0x0d, 0x78, 0x85, 0x88, 
        0x49, 0x21, 0x97, 0x97, 0xe4, 0x45, 0x4f, 0xbe,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0xd8, 0x44, 0x1d, 0x07, 0x66, 0x63, 0x12, 0xeb, 
        0x8c, 0xf0, 0x51, 0x6a, 0x0c, 0x53, 0xcc, 0x46, 
        0x60, 0xa7, 0xaf, 0x4a, 0xb3, 0x4c, 0x83, 0x5a, 
        0xf0, 0xfd, 0xd2, 0x05, 0x90, 0xb5, 0x7d, 0xb3,
    ]),
    //utxo_commitment: Blake2Hash::from_bytes([
    //    0xd4, 0xf5, 0x74, 0x51, 0x55, 0xb1, 0x20, 0x28, 0xda, 0xeb, 0x90, 0xdb, 0x04, 0xd8, 0xfe, 0x55, 0x22, 0x5b, 0xc7, 0xab, 0x60,
    //    0x99, 0x57, 0xaf, 0xa7, 0xe4, 0x3f, 0x2b, 0xa1, 0x70, 0xb7, 0x74,
    //]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 0x1922F4E39A6,
    bits: 0x1e7fffff,
    nonce: 0x4e616f72,
    daa_score: 0, // Checkpoint DAA score
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, //script version
        0x01,                                           // Varint
        0x00,                                           // OP-FALSE
        0x77, 0x61, 0x67, 0x6C, 0x61, 0x79, 0x6C, 0x61, 0x2d, 0x6D, 0x61, 0x69, 0x6E, 0x6E, 0x65, 0x74, // waglayla-mainnet
    ],
};

pub const TESTNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x57, 0xd9, 0x4c, 0x8a, 0xb3, 0x04, 0xf6, 0xe2, 
        0x62, 0xe0, 0xc6, 0x2e, 0x0a, 0xbf, 0xad, 0x6b, 
        0x95, 0x4f, 0x8f, 0x7e, 0x68, 0x21, 0x3c, 0x42, 
        0x6e, 0x06, 0xf5, 0x78, 0xe8, 0xf4, 0x5b, 0xbc,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x00, 0x36, 0x41, 0x22, 0x4f, 0x19, 0xb8, 0x20, 
        0xda, 0xfb, 0x37, 0xa0, 0x2b, 0x3b, 0xd7, 0x16, 
        0x02, 0xd8, 0xb2, 0x19, 0x69, 0xfe, 0x9a, 0x73, 
        0x96, 0x8c, 0xca, 0x52, 0x7e, 0xe6, 0xb7, 0x36,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 0x1922F4D97FE,
    bits: 0x1e7fffff,
    nonce: 0x14582,
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                         // Varint
        0x00,                                                                         // OP-FALSE
        0x77, 0x61, 0x67, 0x6C, 0x61, 0x79, 0x6C, 0x61, 0x2d, 0x74, 0x65, 0x73, 0x74, 0x6e, 0x65, 0x74, // waglayla-testnet
    ],
};

pub const TESTNET11_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0xd6, 0xc1, 0x82, 0x5a, 0x48, 0x2b, 0x0a, 0x7e, 0x95, 0x03, 0x74, 0x2f, 0xa0, 0x18, 0xa7, 0x5b, 0xc1, 0xbd, 0x09, 0x0d, 0xfd,
        0x4f, 0x66, 0xf0, 0x20, 0x87, 0x2e, 0x25, 0xfe, 0xb5, 0x5c, 0xfc,
    ]),
    hash_merkle_root: Hash::from_bytes([
        0x06, 0xe7, 0x41, 0x2d, 0x29, 0xb4, 0x7e, 0x7b, 0x98, 0xfa, 0x98, 0x2f, 0x74, 0x07, 0x53, 0xf2, 0xe7, 0xfd, 0x62, 0xee, 0x73,
        0x41, 0x46, 0x29, 0xcc, 0x29, 0x8c, 0x43, 0xf7, 0x29, 0x14, 0x93,
    ]),
    bits: 504155340, // see `gen_testnet11_genesis`
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                         // Varint
        0x00,                                                                         // OP-FALSE
        0x77, 0x61, 0x67, 0x6C, 0x61, 0x79, 0x6C, 0x61, 0x2d, 0x74, 0x65, 0x73, 0x74, 0x6e, 0x65, 0x74, // waglayla-testnet
        11, 4                                                                         // TN11, Relaunch 4
    ],
    ..TESTNET_GENESIS
};

pub const SIMNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x98, 0x88, 0xfe, 0x7e, 0xca, 0xcb, 0x4d, 0xda, 0xd1, 0x49, 0x4f, 0x4f, 0xfa, 0xf5, 0xc4, 0xf3, 0xb0, 0x3a, 0x71, 0xed, 0x7d,
        0xc9, 0xee, 0x1c, 0xca, 0xcb, 0x10, 0xd1, 0xe8, 0xa8, 0xeb, 0x5b,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x19, 0x46, 0xd6, 0x29, 0xf7, 0xe9, 0x22, 0xa7, 0xbc, 0xed, 0x59, 0x19, 0x05, 0x21, 0xc3, 0x77, 0x1f, 0x73, 0xd3, 0x52, 0xdd,
        0xbb, 0xb6, 0x86, 0x56, 0x4a, 0xd7, 0xfd, 0x56, 0x85, 0x7c, 0x1b,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 0x17c5f62fbb6,
    bits: 0x207fffff,
    nonce: 0x2,
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                   // Varint
        0x00,                                                                   // OP-FALSE
        0x77, 0x61, 0x67, 0x6C, 0x61, 0x79, 0x6C, 0x61, 0x2d, 0x73, 0x69, 0x6d, 0x6e, 0x65, 0x74, // waglayla-simnet
    ],
};

pub const DEVNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        // Golang devnet genesis hash
        // 0xb3, 0x13, 0x87, 0x0a, 0x32, 0xc7, 0x04, 0xbd, 0xf1, 0x21, 0x4a, 0x3b, 0x27, 0x0c, 0xc4, 0x75, 0xd9, 0x42, 0xc2, 0x09, 0x2d,
        // 0x37, 0x9b, 0xc8, 0x70, 0x0a, 0xb0, 0x43, 0x31, 0x9e, 0xf8,
        // 0x46,
        // New rust devnet genesis hash updated according to the modified bits field (see below)
        0x4c, 0xb4, 0x8d, 0x0b, 0x20, 0x73, 0xb8, 0x02, 0x36, 0x01, 0x45, 0xa1, 0x5a, 0xd1, 0xab, 0xdc, 0x01, 0xd8, 0x9b, 0x5c, 0x2f,
        0xe4, 0x72, 0x26, 0x30, 0xab, 0x9b, 0x5f, 0xe9, 0xdf, 0xc4, 0xf2,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x58, 0xab, 0xf2, 0x03, 0x21, 0xd7, 0x07, 0x16, 0x16, 0x2b, 0x6b, 0xf8, 0xd9, 0xf5, 0x89, 0xca, 0x33, 0xae, 0x6e, 0x32, 0xb3,
        0xb1, 0x9a, 0xbb, 0x7f, 0xa6, 0x5d, 0x11, 0x41, 0xa3, 0xf9, 0x4d,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 0x11e9db49828,
    // bits: 525264379, // Golang devnet genesis bits
    bits: 0x1e21bc1c, // Bits with ~testnet-like difficulty for slow devnet start
    nonce: 0x48e5e,
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                   // Varint
        0x00,                                                                   // OP-FALSE
        0x77, 0x61, 0x67, 0x6C, 0x61, 0x79, 0x6C, 0x61, 0x2d, 0x64, 0x65, 0x76, 0x6e, 0x65, 0x74, // waglayla-devnet
    ],
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::bps::Testnet11Bps, merkle::calc_hash_merkle_root};

    #[ignore] // TODO: Waglayla
    #[test]
    fn test_genesis_hashes() {
        [GENESIS, TESTNET_GENESIS, TESTNET11_GENESIS, SIMNET_GENESIS, DEVNET_GENESIS].into_iter().for_each(|genesis| {
            let block: Block = (&genesis).into();
            assert_hashes_eq(calc_hash_merkle_root(block.transactions.iter()), block.header.hash_merkle_root);
            assert_hashes_eq(block.hash(), genesis.hash);
        });
    }

    #[test]
    fn gen_testnet11_genesis() {
        let bps = Testnet11Bps::bps();
        let mut genesis = TESTNET_GENESIS;
        let target = waglayla_math::Uint256::from_compact_target_bits(genesis.bits);
        let scaled_target = target * bps / 100;
        let scaled_bits = scaled_target.compact_target_bits();
        genesis.bits = scaled_bits;
        if genesis.bits != TESTNET11_GENESIS.bits {
            panic!("Testnet 11: new bits: {}\nnew hash: {:#04x?}", scaled_bits, Block::from(&genesis).hash().as_bytes());
        }
    }

    fn assert_hashes_eq(got: Hash, expected: Hash) {
        if got != expected {
            // Special hex print to ease changing the genesis hash according to the print if needed
            panic!("Got hash {:#04x?} while expecting {:#04x?}", got.as_bytes(), expected.as_bytes());
        }
    }
}

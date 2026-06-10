use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    decode(hex_str).map_err(|e| e.to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut v = bytes.to_vec();
    v.reverse();
    v
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    let mut bytes = num.to_be_bytes();
    bytes.reverse();
    bytes
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_e| "Invalid satoshi amount".to_string())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    if script.len() >= 3 && script[0] == 0x76 && script[1] == 0xa9 && script[2] == 0x14 {
        ScriptType::P2PKH
    } else if script.len() >= 2 && script[0] == 0x00 && script[1] == 0x14 {
        ScriptType::P2WPKH
    } else {
        ScriptType::Unknown
    }
}

pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // assumes pushdata starts at index 2
    if script.len() > 2 { &script[2..] } else { &[] }
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance = balance.saturating_sub(fee);
}

pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0x76 => Ok(Opcode::OpDup),
            0xac => Ok(Opcode::OpChecksig),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
}

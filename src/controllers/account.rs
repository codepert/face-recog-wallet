use crate::utils::generate_code;
use anyhow::{bail, Result};
use cess_rust_sdk::subxt::ext::sp_core::{crypto::Ss58Codec, Pair as sp_core_pair};

use hex::FromHex;
use sp_io::crypto::sr25519_verify;
use sp_keyring::sr25519::sr25519::Pair;
use subxt_signer::bip39;
use web3::signing::{keccak256, recover};

pub fn generate_mnemonic() -> Result<String> {
    let random_str = generate_code(16);
    let entropy = random_str.as_bytes();
    let mnemonic = bip39::Mnemonic::from_entropy(entropy)?;
    Ok(mnemonic.to_string())
}

pub fn get_pair(mnemonic: &str, password_override: Option<&str>) -> Result<Pair> {
    let pair = Pair::from_phrase(mnemonic, password_override);

    match pair {
        Ok(pair) => Ok(pair.0),
        Err(err) => bail!(err),
    }
}

// pub fn verify_signed_msg(signed_msg: &str, msg: &[u8], account_str: &str) -> Result<bool> {
//     let account_pubkey = sp_keyring::sr25519::sr25519::Public::from_string(account_str)?;
//     let sign_bytes = match Vec::from_hex(signed_msg) {
//         Ok(sign_bytes) => sign_bytes,
//         Err(_) => bail!("Error: Failed to decode signed message"),
//     };
//     let signed_msg =
//         cess_rust_sdk::subxt::ext::sp_core::sr25519::Signature::from_slice(&sign_bytes[..]);

//     if let Some(signed_msg) = signed_msg {
//         if sr25519_verify(&signed_msg, msg, &account_pubkey) {
//             return Ok(true);
//         }
//     }
//     Ok(false)
// }

pub fn sign_message(msg: &[u8], pair: Pair) -> Result<String> {
    let signed_msg = pair.sign(msg);
    let hex_string = hex::encode(signed_msg);
    Ok(hex_string)
}

// pub fn verify_signed_polkadot_msg(
//     signed_msg: &str,
//     raw_msg: &str,
//     account_str: &str,
// ) -> Result<bool> {
//     // <Bytes>msg</Bytes>
//     // In Substrate/Polkadot the <Bytes> was added to prevent someone using a Polkadot wallet as signing oracle;
//     // its a necessary security measurement.
//     let msg = format!("<Bytes>{}</Bytes>", raw_msg);
//     let msg_bytes = msg.as_bytes();
//     // verify the signed string
//     // The account auth column has the msg string,
//     // form.signed_message is signed message send by the user trying to login
//     if verify_signed_msg(signed_msg, msg_bytes, account_str)? {
//         return Ok(true);
//     }
//     Ok(false)
// }

pub fn verify_signed_evm_msg(signed_msg: &str, msg: &str, account_str: &str) -> Result<bool> {
    let message = hash_message(msg.to_string());
    let signed_msg: &str = if let Some(signed_msg) = signed_msg.strip_prefix("0x") {
        signed_msg
    } else {
        signed_msg
    };

    let signature = hex::decode(signed_msg).unwrap();

    let recovery_id = signature[64] as i32 - 27;
    let pubkey = recover(&message, &signature[..64], recovery_id).unwrap();
    if format!("{:?}", pubkey).to_lowercase() == account_str.to_lowercase() {
        return Ok(true);
    }
    Ok(false)
}

// Hash based to EIP-191
pub fn hash_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}

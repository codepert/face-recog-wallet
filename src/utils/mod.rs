use anyhow::{bail, Result};
use bigdecimal::{BigDecimal, ToPrimitive};
use cess_rust_sdk::chain::ChainSdk;
use cess_rust_sdk::config::{
    get_deoss_account, get_deoss_url, set_custom_deoss_account, set_custom_deoss_url,
    set_custom_url,
};
use cess_rust_sdk::core::utils::account::parsing_public_key;
use cess_rust_sdk::core::utils::bucket::check_bucket_name;
use cess_rust_sdk::polkadot::{
    self,
    runtime_types::{
        cp_cess_common::Hash as CPHash,
        pallet_file_bank::types::{FileInfo, UserBrief},
    },
};
use cess_rust_sdk::subxt::ext::sp_core::crypto::Ss58Codec;
use cess_rust_sdk::subxt::ext::sp_runtime::AccountId32;
use cess_rust_sdk::subxt::tx::PairSigner;
use cess_rust_sdk::subxt::utils::AccountId32 as SubxtUtilAccountId32;
use cess_rust_sdk::utils::{
    account_from_slice, query_storage, sign_and_submit_tx_then_watch_default,
};
use dotenvy::dotenv;
// use hyper::header::HeaderValue;
use hyper::StatusCode;
use lettre::message::header::{ContentType, MessageId};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{/*multipart, */Client, RequestBuilder};
use std::fs::{metadata, File as FFile};
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use tokio::fs::File;

// use crate::jwt;
// use crate::model::FileSegmentDataInfo;
// use crate::view_model::ContainerHierarchy;
use cess_rust_sdk::chain::deoss::DeOss;
use cess_rust_sdk::polkadot::runtime_types::pallet_file_bank::types::FileState;
use cess_rust_sdk::subxt::ext::sp_core::Pair as sp_core_pair;

const LOG_TARGET: &str = "Utils";

// pub fn auth_is_valid_and_is_admin(auth_token: Option<String>) -> Result<String, StatusCode> {
//     if let Some(token) = auth_token {
//         let (is_valid, is_admin, name) = jwt::is_valid(&token)?;
//         if !is_valid || !is_admin {
//             return Err(StatusCode::UNAUTHORIZED);
//         };
//         Ok(name)
//     } else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }

// pub fn auth_is_valid(auth_token: Option<String>) -> Result<String, StatusCode> {
//     if let Some(token) = auth_token {
//         let (is_valid, _, name) = jwt::is_valid(&token)?;
//         if !is_valid {
//             return Err(StatusCode::UNAUTHORIZED);
//         };
//         Ok(name)
//     } else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }

pub fn generate_code(code_len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

    let mut rng = rand::thread_rng();

    let code: String = (0..code_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    code
}

pub fn set_cess_node_rpc_endpoint() {
    dotenv().ok();
    let cess_node_rpc_endpoint =
        env::var("CESS_NODE_RPC_ENDPOINT").expect("CESS_NODE_RPC_ENDPOINT must be set");
    if !cess_node_rpc_endpoint.is_empty() {
        set_custom_url(Some(cess_node_rpc_endpoint));
    }
}

pub fn set_cess_custom_deoss_url() {
    dotenv().ok();
    let cess_custom_deoss_url = env::var("CUSTOM_DEOSS_URL").expect("CUSTOM_DEOSS_URL must be set");
    if !cess_custom_deoss_url.is_empty() {
        set_custom_deoss_url(Some(cess_custom_deoss_url));
    }
}

pub fn set_cess_custom_deoss_account() {
    dotenv().ok();
    let cess_custom_deoss_account =
        env::var("CUSTOM_DEOSS_ACCOUNT").expect("CUSTOM_DEOSS_ACCOUNT must be set");
    if !cess_custom_deoss_account.is_empty() {
        set_custom_deoss_account(Some(cess_custom_deoss_account));
    }
}

pub fn init_chain(mnenomic: &str) -> ChainSdk {
    ChainSdk::new(mnenomic, "deCloud_Service")
}

// pub fn find_ancestor_under_root(
//     container_hierarchy: &[ContainerHierarchy],
//     container_id: i32,
// ) -> Option<(i32, String)> {
//     // container_id: (parent_id, name)
//     let parent_map: std::collections::HashMap<i32, (i32, String)> = container_hierarchy
//         .iter()
//         .flat_map(|container| {
//             let mut map = std::collections::HashMap::new();
//             map.insert(
//                 container.c2_id,
//                 (container.c2_parent_id, container.c2_name.clone()),
//             );
//             map
//         })
//         .collect();

//     let mut key_val = parent_map.get_key_value(&container_id);
//     while let Some((_, (val, _))) = key_val {
//         if parent_map.get_key_value(val).is_some() {
//             key_val = parent_map.get_key_value(val);
//         } else {
//             break;
//         }
//     }

//     if let Some((id, name)) = key_val {
//         return Some((*id, name.clone().1));
//     }

//     None
// }

// pub fn get_container_path(
//     container_hierarchy: &[ContainerHierarchy],
//     container_id: i32,
// ) -> Result<String> {
//     // container_id: (parent_id, name)
//     let parent_map: std::collections::HashMap<i32, (i32, String)> = container_hierarchy
//         .iter()
//         .flat_map(|container| {
//             let mut map = std::collections::HashMap::new();
//             map.insert(
//                 container.c2_id,
//                 (container.c2_parent_id, container.c2_name.clone()),
//             );
//             map
//         })
//         .collect();

//     let mut path_array: Vec<String> = vec![];
//     let mut key_val = parent_map.get_key_value(&container_id);
//     while let Some((_, (val, name))) = key_val {
//         if parent_map.get_key_value(val).is_some() {
//             path_array.push(name.clone());
//             key_val = parent_map.get_key_value(val);
//         } else {
//             path_array.push(name.clone());
//             break;
//         }
//     }
//     let path_array: Vec<String> = path_array.into_iter().rev().collect();
//     let path = path_array.join("/");
//     Ok(path)
// }

pub fn get_file_title_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);
    // Get the file title
    if let Some(file_title) = path.file_stem() {
        if let Some(file_title_str) = file_title.to_str() {
            return Some(file_title_str.to_string());
        }
    }
    None
}

pub fn get_file_name_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);
    // Get the file name
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            return Some(file_name_str.to_string());
        }
    }
    None
}

pub fn get_file_extension(path: &str) -> Option<String> {
    let path = Path::new(path);
    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            return Some(extension_str.to_string());
        } else {
            return None;
        }
    }
    None
}

pub fn get_file_type_by_extension(path: &str) -> String {
    let path = Path::new(path);

    let video = [
        "avi", "wmv", "mpeg", "mp4", "m4v", "mov", "asf", "flv", "f4v", "rmvb", "rm", "3gp",
    ];
    let music = [
        "mp3", "wav", "wma", "mp2", "flac", "midi", "ra", "ape", "aac", "cda",
    ];
    let image = ["jpeg", "jpg", "png", "gif", "bmp", "tiff", "webp", "heif"];
    let application = ["rar", "zip", "7z", "xz", "gz", "exe", "dmg"];

    // Get the file extension
    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            if video.contains(&extension_str) {
                return "video".to_string();
            } else if music.contains(&extension_str) {
                return "music".to_string();
            } else if image.contains(&extension_str) {
                return "image".to_string();
            } else if application.contains(&extension_str) {
                return "application".to_string();
            } else {
                return "unknown".to_string();
            }
        }
    }
    "unknown".to_string()
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.exists() && path.is_file()
}

pub fn get_decloud_wallet() -> Result<String> {
    dotenv().ok();
    let authorizer_mnemonic =
        env::var("DECLOUD_TREASURY_ACCOUNT").expect("DECLOUD_TREASURY_ACCOUNT must be set");

    Ok(authorizer_mnemonic)
}

pub async fn authorize_account_to_upload_to_gateway(mnenomic: &str) -> Result<()> {
    let sdk = init_chain(mnenomic);

    let deoss_account = get_deoss_account();

    let pk_bytes = parsing_public_key(&deoss_account)?;
    let result = sdk.authorize(&pk_bytes).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn file_state_to_string(file_state: FileState) -> String {
    match file_state {
        FileState::Active => "Active".to_string(),
        FileState::Calculate => "Calculate".to_string(),
        FileState::Missing => "Missing".to_string(),
        FileState::Recovery => "Recovery".to_string(),
    }
}

// pub async fn wallet_upload_to_gateway(
//     upload_file: &str,
//     bucket_name: &str,
//     msg: &str,
//     signed_msg: &str,
//     account: &str,
//     eth_account: &Option<String>,
// ) -> Result<String> {
//     let url = get_deoss_url();
//     let fstat = metadata(upload_file)?;
//     if fstat.is_dir() {
//         bail!("not a file")
//     }

//     if fstat.len() == 0 {
//         bail!("empty file")
//     }

//     if !check_bucket_name(bucket_name) {
//         bail!("invalid bucket name")
//     }

//     let mut headers = HeaderMap::new();
//     headers.insert("BucketName", HeaderValue::from_str(bucket_name)?);
//     if let Some(eth_account) = eth_account {
//         headers.insert("EthAcc", HeaderValue::from_str(eth_account)?);
//     }
//     headers.insert("Account", HeaderValue::from_str(account)?);
//     headers.insert("Message", HeaderValue::from_str(msg)?);
//     headers.insert("Signature", HeaderValue::from_str(signed_msg)?);

//     // Create multipart form data
//     let mut form = multipart::Form::new();

//     let mut file = FFile::open(upload_file)?;

//     let mut file_content = Vec::new();
//     file.read_to_end(&mut file_content)?;

//     let upload_file = upload_file.to_string().clone();
//     form = form.part(
//         "file",
//         multipart::Part::stream(file_content.clone()).file_name(upload_file),
//     );

//     let client = match Client::builder().build() {
//         Ok(client) => client,
//         Err(err) => {
//             bail!("{}", err)
//         }
//     };
//     let request_builder: RequestBuilder = client.put(url).headers(headers).multipart(form);

//     let response = request_builder.send().await?;
//     let status_code = response.status();
//     let response_text = response.text().await?;
//     if !status_code.is_success() {
//         if !response_text.is_empty() {
//             bail!(response_text)
//         }
//         bail!("Deoss service failure, please retry or contact administrator.");
//     }

//     Ok(response_text.trim_matches('"').to_string())
// }

pub async fn download_file_from_gateway(
    save_filename: &str,
    fid: &str,
    account: &str,
) -> Result<()> {
    let deoss_url = get_deoss_url();
    let url = format!("{}{}", deoss_url, fid);

    // Create headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Operation", HeaderValue::from_static("download"));
    headers.insert("Account", HeaderValue::from_str(account)?);
    // Create reqwest client and send GET request
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Open a file for writing the response body
        let mut file = File::create(save_filename).await?;

        // Write the response body to the file
        let bytes_written =
            tokio::io::copy(&mut response.bytes().await?.as_ref(), &mut file).await?;

        println!("Downloaded {} bytes to {}", bytes_written, save_filename);
    } else {
        println!("Error: {}", response.text().await?);
    }
    Ok(())
}

pub async fn send_rewards(address: &str, amount: BigDecimal) -> Result<()> {
    let decloud_wallet = get_decloud_wallet()?;
    let pair =
        <sp_keyring::sr25519::sr25519::Pair as sp_core_pair>::from_string(&decloud_wallet, None)
            .unwrap();
    let from = PairSigner::new(pair.clone());
    let pk_bytes = parsing_public_key(address).unwrap();
    let dest = account_from_slice(&pk_bytes);

    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(
        cess_rust_sdk::subxt::utils::MultiAddress::Id(dest),
        amount.to_u128().unwrap_or_default(),
    );

    let events = sign_and_submit_tx_then_watch_default(&balance_transfer_tx, &from).await?;

    let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    Ok(())
}

pub async fn create_bucket(bucket_name: &str, signed_msg: &str, account: &str) -> Result<bool> {
    let url = get_deoss_url();
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("BucketName", HeaderValue::from_str(bucket_name).unwrap());
    headers.insert("Account", HeaderValue::from_str(account).unwrap());
    headers.insert("Message", HeaderValue::from_str(bucket_name).unwrap());
    headers.insert("Signature", HeaderValue::from_str(signed_msg).unwrap());

    let request = client.put(url).headers(headers);
    let response = request.send().await?;
    if response.status().is_success() {
        let body = response.text().await?; // Get the response body as String
        info!(target: LOG_TARGET, "bucket created: DeOss response: {}", body);
        Ok(true)
    } else {
        error!(target: LOG_TARGET, "Error: {}", response.status());
        bail!("Error creating bucket. Please try again!");
    }
}

// pub fn get_fid(path: &str) -> Result<FileSegmentDataInfo> {
//     dotenv().ok();

//     let shard_path = env::var("SHARD_PATH").expect("SHARD_PATH must be set");

//     let cmd = Command::new(shard_path).arg(path).output()?;

//     if cmd.status.success() {
//         // Convert the output bytes to a UTF-8 string
//         let output_str = String::from_utf8_lossy(&cmd.stdout);
//         let bytes: Vec<u8> = output_str
//             .split_whitespace()
//             .filter_map(|s| s.parse::<u8>().ok())
//             .collect();

//         let json_str = format!("{{{}}}", String::from_utf8_lossy(&bytes));
//         let segment_data_info: FileSegmentDataInfo = serde_json::from_str(&json_str)?;
//         let segment_data_info_clone = segment_data_info.clone();
//         tokio::spawn(async move {
//             for segment_data in &segment_data_info_clone.SegmentData {
//                 for fragment_hash in &segment_data.FragmentHash {
//                     if file_exists(fragment_hash) {
//                         match fs::remove_file(fragment_hash) {
//                             Ok(_) => {
//                                 debug!(target: LOG_TARGET, "fragment hash deleted: {}", fragment_hash)
//                             }
//                             Err(e) => {
//                                 error!(target: LOG_TARGET, "fragment hash not deleted: {}", e)
//                             }
//                         }
//                     }
//                 }
//             }
//         });

//         Ok(segment_data_info)
//     } else {
//         // Print the error to the console
//         let error = String::from_utf8(cmd.stderr)?;
//         bail!("{}", error)
//     }
// }

pub async fn user_available_space(address: &str) -> Result<Option<i64>> {
    let pk_bytes = parsing_public_key(address).unwrap();
    let account = account_from_slice(&pk_bytes);
    let query = polkadot::storage()
        .storage_handler()
        .user_owned_space(&account);
    let result = query_storage(&query, None).await?;
    let value = match result {
        Some(owned_space_details) => Some(owned_space_details.remaining_space as i64),
        None => None,
    };

    Ok(value)
}

pub async fn user_available_space_status(address: &str) -> Result<bool> {
    let pk_bytes = parsing_public_key(address).unwrap();
    let account = account_from_slice(&pk_bytes);
    let query = polkadot::storage()
        .storage_handler()
        .user_owned_space(&account);
    let result = query_storage(&query, None).await?;
    let value = match result {
        Some(owned_space_details) => Some(owned_space_details.state),
        None => None,
    };

    if let Some(value) = value {
        let status = String::from_utf8_lossy(&value.0).to_string();
        if status.eq("normal") {
            return Ok(true);
        }
        if status.eq("frozen") {
            return Ok(false);
        }
        bail!("Error getting storage information.");
    }
    bail!("Error getting storage information.");
}

// query file metadata from chain
pub async fn query_file_metadata(file_hash: &str) -> Result<Option<FileInfo>> {
    let file_hash = match file_hash.as_bytes().try_into() {
        Ok(hash) => hash,
        Err(_) => bail!("Error: Fid is wrong, Fid should be of length 64"),
    };
    let hash = CPHash(file_hash);
    let query = polkadot::storage().file_bank().file(hash);
    let file_md = query_storage(&query, None).await?;
    Ok(file_md)
}

pub async fn is_file_owner(file_hash: &str, address: &str) -> Result<bool> {
    let file_md = query_file_metadata(file_hash).await?;

    let address = AccountId32::from_string(address)?;
    let address = SubxtUtilAccountId32::from(address);
    if let Some(file_md) = file_md {
        let owners = file_md.owner.0;
        for UserBrief { user, .. } in owners {
            if user.eq(&address) {
                return Ok(true);
            }
        }
        Ok(false)
    } else {
        Ok(false)
    }
}

pub fn delete_file_from_disk(path: &str) -> Result<()> {
    Ok(fs::remove_file(path)?)
}

pub fn half_of_fid_as_slice(fid: String) -> String {
    fid[32..].to_string()
}

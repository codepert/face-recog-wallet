use actix_web::{web, HttpResponse, Responder};
use cess_rust_sdk::core::utils::account::get_pair_address_as_ss58_address;
use serde::{Deserialize, Serialize};
use sp_keyring::sr25519::sr25519::Pair;
use subxt_signer::bip39::Mnemonic;
use crate::controllers::accounts::{generate_mnemonic, get_pair};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WalletInfo {
    feature_info: String
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the face-recognization rust server!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Status: Running")
}

pub async fn get_wallet_post(info: web::Json<WalletInfo>) -> impl Responder {
    let response_message = format!(
        "address: {}",
        info.feature_info
    );
    HttpResponse::Ok().body(response_message)
}

pub async fn create_wallet_post(info: web::Json<WalletInfo>) -> impl Responder {
    let mnemonic: Option<String>;
    println!("hello there1");
    match generate_mnemonic() {
        Ok(t) => mnemonic = Some(t),
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").json("internal error on `generate_mnemonic`");
        }
    };

    println!("hello there2");
    let pair: Pair;
    match get_pair(&mnemonic.clone().unwrap(), None) {
        Ok(t) => pair = t,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").json("internal error on `get_pair`");
        }
    };

    println!("hello there3");
    let address_to_fund: String;
    match get_pair_address_as_ss58_address(pair) {
        Ok(t) => address_to_fund = t,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").json("internal error on `get_pair_address_as_ss58_address`");
        }
    }
    // let address = Some(address_to_fund.clone());
    println!("hello there4");

    #[derive(Serialize, Debug)]
    pub struct WalletResponse {
        wallet_address: String,
    }

    let response_message = WalletResponse {
        wallet_address: address_to_fund
    };

    println!("{:?}", response_message);
    HttpResponse::Ok().json(response_message)
}
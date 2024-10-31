
use subxt::{
    client::OnlineClientT, config::{signed_extensions, substrate::{BlakeTwo256, Digest, NumberOrHex}, Hasher, Header, PolkadotExtrinsicParams}, utils::{AccountId32, MultiAddress}, Config, OnlineClient, PolkadotConfig, SubstrateConfig
};
use hex::encode as hex_encode;
use sha3;
use sha3::Digest as Keccak256Digest;
use crate::sha3::Keccak256;
use hex_literal::hex;
use std::fmt;
// mod gasp;
// use subxt_signer::sr25519::dev::{self};
use primitive_types::{U256, H256, H512};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};
use secp256k1::{
	ecdsa::{RecoverableSignature, RecoveryId}, rand, Message, PublicKey, Secp256k1, SecretKey
    // SECP256K1,
};
use subxt_signer::ecdsa::dev;
use subxt::tx::signer::Signer as SignerT;
use crate::gasp::{GaspAddress, GaspSignature, GaspPublicKey};

use secp256k1::{
	SECP256K1,
};

pub struct Keypair(pub secp256k1::Keypair);

impl Keypair {
    pub fn from_secret_key(secret: [u8; 32]) -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&secret[..]).expect("32 bytes, within curve order");
        Keypair(secp256k1::Keypair::from_secret_key(&secp, &secret_key))
    }

    pub fn public(&self) -> GaspPublicKey {
        self.0.public_key().serialize().into()
    }

    pub fn address(&self) -> GaspAddress {
        let mut res = [0u8; 64];
        res.copy_from_slice(&self.0.public_key().serialize_uncompressed()[1..]);
        let mut buffer = [0u8; 20];
        buffer.copy_from_slice(&Keccak256::digest(&res)[12..32]);
        buffer.into()
    }

    pub fn secret_key(&self) -> SecretKey {
        self.0.secret_key()
    }

    pub fn sign_prehashed(&self, message: &[u8; 32]) -> GaspSignature{
        let message = Message::from_digest_slice(message).expect("Message is 32 bytes; qed");
        let secret_key = SecretKey::from_slice(&self.0.secret_bytes()).expect("Secret key is 32 bytes; qed");
        let recsig = SECP256K1.sign_ecdsa_recoverable(&message, &secret_key);
        let (recid, sig): (_, [u8; 64]) = recsig.serialize_compact();
        let mut signature_bytes: [u8; 65] = [0; 65];
        signature_bytes[..64].copy_from_slice(&sig);
        signature_bytes[64] = (recid.to_i32() & 0xFF) as u8 ;
        signature_bytes.into()
    }
}


impl<T: Config> SignerT<T> for Keypair
where
    T::AccountId: From<GaspAddress>,
    T::Address: From<GaspAddress>,
    T::Signature: From<GaspSignature>,
{
    fn account_id(&self) -> T::AccountId {
        self.address().into()
    }

    fn address(&self) -> T::Address {
        self.address().into()
    }

    fn sign(&self, signer_payload: &[u8]) -> T::Signature {
        let hashed = Keccak256::digest(signer_payload);
        let signature = self.sign_prehashed(&hashed.into());
        signature.into()
    }
}


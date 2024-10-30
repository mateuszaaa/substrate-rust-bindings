use subxt::{
    client::OnlineClientT, config::{signed_extensions, substrate::{BlakeTwo256, Digest, NumberOrHex}, Hasher, Header, PolkadotExtrinsicParams}, utils::{AccountId32, MultiAddress}, Config, OnlineClient, PolkadotConfig, SubstrateConfig
};
use hex::encode as hex_encode;
use sha3;
use sha3::Digest as Keccak256Digest;
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

use secp256k1::{
	SECP256K1,
};
pub fn keccak_256(data: &[u8]) -> [u8; 32] {
	sha3::Keccak256::digest(data).into()
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GaspSignature(pub [u8; 65]);

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GaspPublicKey(pub [u8; 33]);

impl GaspPublicKey {
    pub fn to_address(self) -> GaspAddress {
        let mut buffer = [0u8; 20];
        buffer.copy_from_slice(&keccak_256(&self.0)[12..32]);
        println!("Address {}", HexSlice(&keccak_256(&self.0)));
        GaspAddress(buffer)
    }
}

impl From<GaspPublicKey> for GaspAddress {
    fn from(value: GaspPublicKey) -> Self {
        value.to_address()
    }
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GaspAddress(pub [u8; 20]);

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}
// You can choose to implement multiple traits, like Lower and UpperHex
impl fmt::Display for HexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            // Decide if you want to pad the value or have spaces inbetween, etc.
            write!(f, "{:x}", byte)?;
        }
        Ok(())
    }
}


pub struct Keypair(pub secp256k1::Keypair);

impl Keypair {
    fn from_secret_key(secret: [u8; 32]) -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&secret[..]).expect("32 bytes, within curve order");
        Keypair(secp256k1::Keypair::from_secret_key(&secp, &secret_key))
    }

    fn public(&self) -> GaspPublicKey {
        GaspPublicKey(self.0.public_key().serialize())
    }

    fn address(&self) -> GaspAddress {
        let mut res = [0u8; 64];
        res.copy_from_slice(&self.0.public_key().serialize_uncompressed()[1..]);

        let mut buffer = [0u8; 20];
        buffer.copy_from_slice(&keccak_256(&res)[12..32]);
        GaspAddress(buffer)
    }

    fn secret_key(&self) -> SecretKey {
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
        GaspSignature(signature_bytes)
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
        println!("keypair::sign payload: len {}", signer_payload.len());
        println!("keypair::sign payload: 0x{}", hex_encode(signer_payload));
        let hashed = keccak_256(signer_payload);
        println!("keypair::sign hash(signer_payload): 0x{}", hex_encode(hashed));
        let signature = self.sign_prehashed(&hashed);
        println!("keypair::sign signature(hash(signer_payload)): 0x{}", hex_encode(signature.0));
        signature.into()
    }
}


#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod gasp {}

// PolkadotConfig or SubstrateConfig will suffice for this example at the moment,
// but PolkadotConfig is a little more correct, having the right `Address` type.
//
// pub type Signature = EthereumSignature;
// pub type Signer = <Signature as Verify>::Signer;
// pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;


#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GaspConfig {}

/// The default [`super::ExtrinsicParams`] implementation understands common signed extensions
/// and how to apply them to a given chain.
pub type GaspExtrinsicParams<T> = signed_extensions::AnyOf<
    T,
    (
        signed_extensions::CheckSpecVersion,
        signed_extensions::CheckTxVersion,
        signed_extensions::CheckGenesis<T>,
        signed_extensions::CheckMortality<T>,
        signed_extensions::CheckNonce,
        signed_extensions::ChargeTransactionPayment,
    ),
>;


impl Config for GaspConfig {
    type Hash = H256;
    type AccountId = GaspAddress;
    type Address = GaspAddress;
    type Signature = GaspSignature;
    type Hasher = BlakeTwo256;
    type Header = GaspHeader<u32, BlakeTwo256>;
    type ExtrinsicParams = GaspExtrinsicParams<Self>;
    type AssetId = u32;
}

fn serialize_number<S, T: Copy + Into<U256>>(val: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let u256: U256 = (*val).into();
    serde::Serialize::serialize(&u256, s)
}

fn deserialize_number<'a, D, T: TryFrom<U256>>(d: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'a>,
{
    // At the time of writing, Smoldot gives back block numbers in numeric rather
    // than hex format. So let's support deserializing from both here:
    let number_or_hex = NumberOrHex::deserialize(d)?;
    let u256 = number_or_hex.into_u256();
    TryFrom::try_from(u256).map_err(|_| serde::de::Error::custom("Try from failed"))
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ShufflingSeed {
	/// shuffling seed for the previous block
	pub seed: H256,
	/// seed signature
	pub proof: H512,
}


#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaspHeader<N: Copy + Into<U256> + TryFrom<U256>, H: Hasher> {
    /// The parent hash.
    pub parent_hash: H::Output,
    /// The block number.
    #[serde(
        serialize_with = "serialize_number",
        deserialize_with = "deserialize_number"
    )]
    #[codec(compact)]
    pub number: N,
    /// The state trie merkle root
    pub state_root: H::Output,
    /// The merkle root of the extrinsics.
    pub extrinsics_root: H::Output,
    /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    pub digest: Digest,

    pub seed: ShufflingSeed,

    pub count: N,
}

impl<N, H> Header for GaspHeader<N, H>
where
    N: Copy + Into<u64> + Into<U256> + TryFrom<U256> + Encode,
    H: Hasher + Encode,
    GaspHeader<N, H>: Encode + Decode,
{
    type Number = N;
    type Hasher = H;
    fn number(&self) -> Self::Number {
        self.number
    }
}


#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<GaspConfig>::from_url("ws://127.0.0.1:9944").await?;
    println!("Connection with parachain established.");

    let keypair = Keypair::from_secret_key(hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"));
    println!("Address: 0x{}", hex_encode(&keypair.address().0[..]));


    let storage_query = gasp::storage().system().block_hash(0);

    let result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
    .await?;

    let call = gasp::tx().tokens().transfer(gasp::runtime_types::sp_runtime::account::AccountId20([0u8; 20]), 0, 100);

    let tx = api
        .tx();

    let latest = api.backend().latest_finalized_block_ref().await?;
    let header = api.backend().block_header(latest.hash()).await?.unwrap();
    println!("Latest block: {:?}", latest);
    println!("Latest block: {:#?}", header);

    let partial_signed = tx.create_partial_signed(&call, &keypair.address(), Default::default()).await.expect("correct");
    // let signed = tx.create_signed(&call, &keypair, Default::default()).await.expect("correct");

    println!("payload 0x{}", hex_encode(partial_signed.signer_payload()));;
    println!("hash(payload) 0x{}", hex_encode(keccak_256(partial_signed.signer_payload().as_slice())));;
    let signed = partial_signed.sign(&keypair);
    println!("TX 0x{}", hex_encode(signed.encoded()));;

    let result = signed.submit_and_watch()
    .await
    .map(|e| {
        println!("NFT creation submitted, waiting for transaction to be finalized...");
        e
     })?
    .wait_for_finalized_success()
    .await?;
    //
    // println!("{:?}", partial_signed);
    // println!("{:?}", signed);
    // println!("{:?}", signed2);

    // signed.sign(keypair);
    // println!("Encoded call {}", hex_encode(signed.encoded()));

    // let result = tx.sign_and_submit_then_watch_default(&call, &keypair)
    // .await
    // .map(|e| {
    //     println!("NFT creation submitted, waiting for transaction to be finalized...");
    //     e
    //  })?
    // .wait_for_finalized_success()
    // .await?;
    //
    // let nft_creation_tx = gasp::tx()
    //     .tokens()
    //     .transfer(COLLECTION_ID, NTF_ID, alice.clone());

    // let _nft_creation_events = api
    //     .tx()
    //     .sign_and_submit_then_watch_default(&nft_creation_tx, &alice_pair_signer)
    //     .await
    //     .map(|e| {
    //         println!("NFT creation submitted, waiting for transaction to be finalized...");
    //         e
    //     })?
    //     .wait_for_finalized_success()
    //     .await?;
    println!("NFT created.");


// Address: 0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac
// signer_payload: 0a00000000000000000000000000000000000000000000000000910114000c006400000001000000fa3e64b8f64cdcb1541f6bcf09476c516bf20545befbabc9899f167b53717434d4c518c099f2575a1234e0e19f02396dedb3dfe8ea2457858150a035bd1503eb
// hash(signer_payload): e92d5656a8285943489661d7adf896df5a7e32dbc44ea4a8143ddca5123d0738
// signature(hash(signer_payload)): 1814006d116c2b8f9ce10cb66106638b0252673869069bab5b653239117a7d464f5d98cc91c8f84129f304ec1d1d83f390d874fde17efbc996d07935fcabed2f00
// TX 0xd90184f24ff3a9cf04c71dbc94d0b566f7a27b94566cac
    // 1814006d116c2b8f9ce10cb66106638b0252673869069bab5b653239117a7d464f5d98cc91c8f84129f304ec1d1d83f390d874fde17efbc996d07935fcabed2f00
    // 14000c000a000000000000000000000000000000000000000000000000009101


    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::Keypair;

    #[test]
    fn test_signer() {
        let keypair = Keypair::from_secret_key(hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"));
        let payload = hex!("0a000000000000000000000000000000000000000000000000009101c4000c006400000001000000fa3e64b8f64cdcb1541f6bcf09476c516bf20545befbabc9899f167b537174348bc02ea9888bcb3c61dd6ce6d5a29dfa0fb63b967d3e5bb23185c141a2a3409e");
        let hashed_payload: [u8; 32] = keccak_256(&payload);

        println!("payload: {}", hex_encode(&payload));
        println!("hash(payload): {}", hex_encode(&hashed_payload));

        let signature = keypair.sign_prehashed(&hashed_payload);
        println!("signature: {}", hex_encode(&signature.0));
        assert!(false);

        // let message = Message::from_digest_slice(&hashed_payload).expect("Message is 32 bytes; qed");
        // let recsig = SECP256K1.sign_ecdsa_recoverable(&message, &keypair.secret_key());
        //
        // let mut sig = [0u8; 64];
        // sig.copy_from_slice(&signature.0[..64]);
        //
        // assert_eq!(sig, recsig.serialize_compact().1);
        // assert_eq!(signature.0[64], recsig.serialize_compact().0);

        // SECP256K1.verify(&message, &recsig).expect("Signature is valid; qed");

        // let (recid, sig): (_, [u8; 64]) = recsig.serialize_compact();
    }
}


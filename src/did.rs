use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Context {
    One { #[serde(rename = "@context")] context: String },
    Many { #[serde(rename = "@context")] context: Vec<String> }
}

impl Context {
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            Context::One { context } => vec![context.clone()],
            Context::Many { context } => context.to_vec()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject(String);

impl Subject {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PublicKeyType {
    Ed25519VerificationKey2018,
    RsaVerificationKey2018,
    EcdsaSecp256k1VerificationKey2019
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SignatureType {
    Ed25519Signature2018,
    RsaSignature2018,
    EcdsaKoblitzSignature2016,
    EcdsaSecp256k1Signature2019,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PublicKeyData {
    Pem { #[serde(rename = "publicKeyPem")] key: String },
    Jwk { #[serde(rename = "publicKeyJwk")] key: String },
    Hex { #[serde(rename = "publicKeyHex")] key: String },
    Base64 { #[serde(rename = "publicKeyBase64")] key: String },
    Base58 { #[serde(rename = "publicKeyBase58")] key: String },
    Multibase { #[serde(rename = "publicKeyMultibase")] key: String },
    EthAddr { #[serde(rename = "ethereumAddress")] key: String }
}

impl PublicKeyData {
    pub fn as_str(&self) -> &str {
        match self {
            PublicKeyData::Pem{ key } => &key,
            PublicKeyData::Jwk{ key } => &key,
            PublicKeyData::Hex{ key } => &key,
            PublicKeyData::Base64{ key } => &key,
            PublicKeyData::Base58{ key } => &key,
            PublicKeyData::Multibase{ key } => &key,
            PublicKeyData::EthAddr{ key } => &key,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub id: Subject,
    #[serde(rename = "type")]
    pub key_type: PublicKeyType,
    pub controller: Subject,
    #[serde(flatten)]
    pub key_data: PublicKeyData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    #[serde(flatten)]
    pub context: Context,
    pub id: Subject,
    #[serde(rename = "publicKey", default)]
    pub public_key: Vec<PublicKey>
}

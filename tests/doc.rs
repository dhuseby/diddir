extern crate diddir;

use diddir::{Document, PublicKeyType};
use serde_json;

#[test]
fn diddir_parse_document() {
    let jstr = r#"
		{
		  "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
		  "id": "did:example:123456789abcdefghi",
		  "publicKey": [{
			"id": "did:example:123456789abcdefghi#keys-1",
			"type": "RsaVerificationKey2018",
			"controller": "did:example:123456789abcdefghi",
			"publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
		  }, {
			"id": "did:example:123456789abcdefghi#keys-2",
			"type": "Ed25519VerificationKey2018",
			"controller": "did:example:pqrstuvwxyz0987654321",
			"publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
		  }, {
			"id": "did:example:123456789abcdefghi#keys-3",
			"type": "EcdsaSecp256k1VerificationKey2019",
			"controller": "did:example:123456789abcdefghi",
			"publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
		  }]
		}
    "#;

    let flat = r#"{"@context":["https://w3id.org/did/v1","https://w3id.org/security/v1"],"id":"did:example:123456789abcdefghi","publicKey":[{"id":"did:example:123456789abcdefghi#keys-1","type":"RsaVerificationKey2018","controller":"did:example:123456789abcdefghi","publicKeyPem":"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"},{"id":"did:example:123456789abcdefghi#keys-2","type":"Ed25519VerificationKey2018","controller":"did:example:pqrstuvwxyz0987654321","publicKeyBase58":"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"},{"id":"did:example:123456789abcdefghi#keys-3","type":"EcdsaSecp256k1VerificationKey2019","controller":"did:example:123456789abcdefghi","publicKeyHex":"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"}]}"#;

    let doc: Document = serde_json::from_str(jstr).unwrap();
    assert_eq!(doc.context.as_vec().len(), 2);
    assert_eq!(doc.id.as_str(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key.len(), 3);

    assert_eq!(doc.public_key[0].id.as_str(), "did:example:123456789abcdefghi#keys-1");
    assert_eq!(doc.public_key[0].key_type, PublicKeyType::RsaVerificationKey2018);
    assert_eq!(doc.public_key[0].controller.as_str(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key[0].key_data.as_str(), "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----");

    assert_eq!(doc.public_key[1].id.as_str(), "did:example:123456789abcdefghi#keys-2");
    assert_eq!(doc.public_key[1].key_type, PublicKeyType::Ed25519VerificationKey2018);
    assert_eq!(doc.public_key[1].controller.as_str(), "did:example:pqrstuvwxyz0987654321");
    assert_eq!(doc.public_key[1].key_data.as_str(), "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV");

    assert_eq!(doc.public_key[2].id.as_str(), "did:example:123456789abcdefghi#keys-3");
    assert_eq!(doc.public_key[2].key_type, PublicKeyType::EcdsaSecp256k1VerificationKey2019);
    assert_eq!(doc.public_key[2].controller.as_str(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key[2].key_data.as_str(), "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71");

    let s: String = serde_json::to_string(&doc).unwrap();
    assert_eq!(s.as_str(), flat);
}


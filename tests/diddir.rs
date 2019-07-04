#[macro_use]
extern crate cfg_if;
extern crate diddir;
extern crate tempfile;

use diddir::{Config, DIDDir, Document, PublicKeyType};
use tempfile::{tempdir, TempDir};
use serde_json;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};

/* bring in OS-specific glue functions */
cfg_if! {
    if #[cfg(unix)] {
        use diddir::diddir::unix::DIDDirSys;
    } else if #[cfg(target_os = "windows")] {
        use diddir::diddir::windows::DIDDirSys;
    } else if #[cfg(wasm)] {
        use diddir::diddir::wasm::DIDDirSys;
    }
}

#[test]
fn diddir_init() {
    let dir = tempdir().unwrap();
    let config = Config::with_path(dir.path());
    let _diddir = DIDDir::init(&config);

    let dirs = vec![config.root_dir(), config.aliases_dir(), config.tmp_dir()];
    for d in dirs {
        assert!(d.is_dir());
        assert_eq!((), DIDDirSys::check_permission(&d).unwrap());
    }
}

#[test]
fn diddir_open_or_init_init() {
    let dir = tempdir().unwrap();
    let config = Config::with_path(dir.path());
    let _diddir = DIDDir::open_or_init(&config);

    let dirs = vec![config.root_dir(), config.aliases_dir(), config.tmp_dir()];
    for d in dirs {
        assert!(d.is_dir());
        assert_eq!((), DIDDirSys::check_permission(&d).unwrap());
    }
}

#[test]
fn diddir_open_or_init_open() {
    let (_tmpdir, config) = create_test_diddir().unwrap();
    let _diddir = DIDDir::open_or_init(&config);

    let dirs = vec![config.root_dir(), config.aliases_dir(), config.tmp_dir()];
    for d in dirs {
        assert!(d.is_dir());
        assert_eq!((), DIDDirSys::check_permission(&d).unwrap());
    }
}

#[test]
fn diddir_open() {
    let (_tmpdir, config) = create_test_diddir().unwrap();
    let diddir = DIDDir::open(&config).unwrap();

    // get default identity from "default" alias
    let default_id = diddir.get_pkid_from_alias(&"default".to_string()).unwrap();
    assert_eq!(default_id, "c506310b2c1ceb27212c4478055a44ac6b26969af73da0828bf28fc4867f09bb".to_string());

    // get all aliases from default pkid
    let aliases = diddir.get_aliases(&default_id).unwrap();
    assert_eq!(aliases.len(), 2);
    assert!((aliases[0] == "default".to_string()) || (aliases[0] == "chad.smith@no.email".to_string()));
    assert!((aliases[1] == "default".to_string()) || (aliases[1] == "chad.smith@no.email".to_string()));

    let stacy_id = diddir.get_pkid_from_alias(&"stacy.jones@no.email".to_string()).unwrap();
    assert_eq!(stacy_id, "8b69351b707a187559ef7e87d898430dc016680c52b36e23d8703a2e030b30dd".to_string());
}

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

#[test]
fn diddir_save_identity() {
    let becky_pkid = "1c4adf1b64e2f50a18d83f63a45c7f6bf04772a0ae7d6c2bdee008df01364e03".to_string();
    let becky_did = "{\n  \"name\": \"Becky Adams\"\n}".to_string();

    let (_tmpdir, config) = create_test_diddir().unwrap();
    let mut diddir = DIDDir::open(&config).unwrap();

    // make sure we only have two ids
    let ids1 = diddir.get_identities().unwrap();
    assert_eq!(ids1.len(), 2);

    // add becky
    diddir.save_identity(&becky_pkid, &becky_did).unwrap();
    let ids2 = diddir.get_identities().unwrap();
    assert_eq!(ids2.len(), 3);

    // make sure the files are on disk
    let path1: PathBuf = [config.root_dir(), Path::new(&becky_pkid)].iter().collect();
    assert!(path1.exists());

    // make sure the id matches
    let id = diddir.get_identity(&becky_pkid).unwrap();
    assert_eq!(becky_did, id);
}

#[test]
fn diddir_remove_identity() {
    let (_tmpdir, config) = create_test_diddir().unwrap();
    let mut diddir = DIDDir::open(&config).unwrap();

    // remove stacy
    let stacy_pkid = "8b69351b707a187559ef7e87d898430dc016680c52b36e23d8703a2e030b30dd".to_string();
    diddir.remove_identity(&stacy_pkid).unwrap();
    
    // make sure stacy's id file is gone
    let path: PathBuf = [config.root_dir(), Path::new(&stacy_pkid)].iter().collect();
    assert!(!path.exists());
    assert_eq!(diddir.get_aliases(&stacy_pkid), None);

    // make sure there's only one id left
    let ids = diddir.get_identities().unwrap();
    assert_eq!(ids.len(), 1);
}

#[test]
fn diddir_get_identity() {
    let (_tmpdir, config) = create_test_diddir().unwrap();
    let diddir = DIDDir::open(&config).unwrap();

    // get chad
    let chad_pkid = "c506310b2c1ceb27212c4478055a44ac6b26969af73da0828bf28fc4867f09bb".to_string();
    let chad_did = "{\n  \"name\": \"Chad Smith\",\n  \"secrets\": {\n    \"email\": \"chad.smith@no.email\",\n    \"secret_key\": \"d065bee71747546ace230a33a6e5aa23ce68aa7446d3e7c9dca53e0fc5bd094e41d3b02ce5938231ec8eaaa4666e15c6590c8fc249f760ad43ce1ec7fbdd9a25246fa6bd7887e20c862d96a4656e1418\"\n  }\n}\n".to_string();
    let id = diddir.get_identity(&chad_pkid).unwrap();
    assert_eq!(chad_did, id);
}

#[test]
fn diddir_get_pkid_from_alias() {
    let (_tmpdir, config) = create_test_diddir().unwrap();
    let diddir = DIDDir::open(&config).unwrap();

    // set up aliases
    let default_alias = "default".to_string();
    let chad_alias = "chad.smith@no.email".to_string();
    let chad_pkid = "c506310b2c1ceb27212c4478055a44ac6b26969af73da0828bf28fc4867f09bb".to_string();

    // get chad by "default" alias
    let id1 = diddir.get_pkid_from_alias(&default_alias).unwrap();
    assert_eq!(chad_pkid, id1);

    // get chad by email alias
    let id2 = diddir.get_pkid_from_alias(&chad_alias).unwrap();
    assert_eq!(chad_pkid, id2);
}

#[test]
fn diddir_save_alias() {
    let (_tmpdir, config) = create_test_diddir().unwrap();
    let mut diddir = DIDDir::open(&config).unwrap();

    // set up
    let new_alias = "foobar".to_string();
    let chad_pkid = "c506310b2c1ceb27212c4478055a44ac6b26969af73da0828bf28fc4867f09bb".to_string();

    // get all aliases before adding a new one
    let aliases1 = diddir.get_aliases(&chad_pkid).unwrap();
    assert_eq!(aliases1.len(), 2);

    // add new alias for chad
    diddir.save_alias(&new_alias, &chad_pkid).unwrap();

    // get all aliases for chad
    let aliases2 = diddir.get_aliases(&chad_pkid).unwrap();
    assert_eq!(aliases2.len(), 3);

    // make sure the files are on disk
    let path1: PathBuf = [config.aliases_dir(), Path::new(&new_alias)].iter().collect();
    assert!(path1.exists());
}

#[test]
fn diddir_remove_alias() {
    let (_tmpdir, config) = create_test_diddir().unwrap();
    let mut diddir = DIDDir::open(&config).unwrap();

    // set up
    let remove_alias = "chad.smith@no.email".to_string();
    let chad_pkid = "c506310b2c1ceb27212c4478055a44ac6b26969af73da0828bf28fc4867f09bb".to_string();

    // get all aliases before removing one
    let aliases1 = diddir.get_aliases(&chad_pkid).unwrap();
    assert_eq!(aliases1.len(), 2);

    // remove alias for chad
    diddir.remove_alias(&remove_alias).unwrap();

    // get all aliases for chad
    let aliases2 = diddir.get_aliases(&chad_pkid).unwrap();
    assert_eq!(aliases2.len(), 1);

    // make sure file is gone
    let path: PathBuf = [config.aliases_dir(), Path::new(&remove_alias)].iter().collect();
    assert!(!path.exists());
}

fn create_test_diddir() -> io::Result<(TempDir, Config)> {
    // get a temporary root dir
    let dir = tempdir().unwrap();

    // set up a config for the root dir
    let config = Config::with_path(dir.path());

    // create the required directories
    let dirs = vec![config.root_dir(), config.aliases_dir(), config.tmp_dir()];
    for d in dirs {
        if !d.exists() {
            fs::create_dir_all(&d)?;
        }
        DIDDirSys::set_permission(&d)?;
    }

    // create the id files
    let data: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data"].iter().collect();
    let ids = vec![
        PathBuf::from("c506310b2c1ceb27212c4478055a44ac6b26969af73da0828bf28fc4867f09bb"),
        PathBuf::from("8b69351b707a187559ef7e87d898430dc016680c52b36e23d8703a2e030b30dd")
    ];

    for f in ids {
        let src = data.join(f.as_path());
        let dst = config.root_dir().join(f.as_path());
        if src.exists() && !dst.exists() {
            fs::copy(&src, &dst).unwrap();
        }
        DIDDirSys::set_permission(&dst)?;
    }

    // create the alias files
    let aliases = vec![
        PathBuf::from("default"),
        PathBuf::from("chad.smith@no.email"),
        PathBuf::from("stacy.jones@no.email")
    ];

    for f in aliases {
        let src = data.join(f.as_path());
        let dst = config.aliases_dir().join(f.as_path());
        if src.exists() && !dst.exists() {
            fs::copy(&src, &dst).unwrap();
        }
        DIDDirSys::set_permission(&dst)?;
    }

    Ok((dir, config))
}

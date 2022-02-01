use serde_derive::Deserialize;
use serde_derive::Serialize;
use openssl::ssl::{SslMethod, SslConnector};
use std::env;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub api_version: String,
    pub kind: String,
    pub spec: Spec,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub signature: Signature,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    pub format: String,
    pub content: String,
    pub public_key: PublicKey,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub hash: Hash,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hash {
    pub algorithm: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    additionalProp1: AdditionalProp,
    additionalProp2: AdditionalProp,
    additionalProp3: AdditionalProp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProp {
    logID: String,
    logIndex: i64,
    body: String,
    integratedTime: i64,
    attestation: Attestation,
    verification: Verification,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    additionalProp1: Box<AdditionalProp>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attestation {
    data: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verification {
    inclusionProof: InclusionProof,
    signedEntryTimestamp: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InclusionProof {
    logIndex: i64,
    rootHash: String,
    treeSize: i64,
    hashes: Vec<String>
}

fn get_github_token() -> String {
    match env::var("GITHUB_AUTH_TOKEN") {
        Ok(val) => format!("Token {}", val),
        Err(_err) => ("").to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {


    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap(); 
    builder.set_ca_file("fulcio.pem").expect("Can't find file");


    openssl_probe::init_ssl_cert_env_vars();

    let new_post = Root{
        api_version: "0.0.1".to_string(),
        kind: "rekord".to_string(),
        spec: Spec{
            signature: Signature{
                format: "x509".to_string(),
                content: "MEUCIQC4Pjpjl++jS+nYODmQwisPSs5SpHuLlitLJ6PNnVUglgIgdhxUIO9BKMWAtLAppoPd++jaQJPMJCeqxuwcY+KHhnI=".to_string(),
                public_key: PublicKey{
                    content: "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFWldZWExVNktPSmhkSDNGSUxSc1JsdW95cmZNOQorMnpPQmdaU1NJQnB6SVgxYytrYVp5OS9Ebkp0M2h3amZpRUZSeEQrbWJDZzAyeEZxaERMd1FUSmxBPT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==".to_string(),
                },
            },
            data: Data{
                hash: Hash{
                    algorithm: "sha256".to_string(),
                    value: "13651bac2801b9ae86fd860c79dbbc6a01953dffd87683d374efece8d89db814".to_string(),
                },
            },
        },
    };
    let new_post: Post = reqwest::Client::new()
        .post("https://rekor.sigstore.dev/api/v1/log/entries")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;


    println!("{:#?}", new_post);
    // Post {
    //     id: Some(
    //         101
    //     ),
    //     title: "Reqwest.rs",
    //     body: "https://docs.rs/reqwest",
    //     user_id: 1
    // }
    Ok(())
}

{
	"api_version": "0.0.1",
	"kind": "rekord",
	"spec": {
		"signature": {
			"format": "x509",
			"content": "MEUCIQC4Pjpjl++jS+nYODmQwisPSs5SpHuLlitLJ6PNnVUglgIgdhxUIO9BKMWAtLAppoPd++jaQJPMJCeqxuwcY+KHhnI=",
			"public_key": {
				"content": "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFWldZWExVNktPSmhkSDNGSUxSc1JsdW95cmZNOQorMnpPQmdaU1NJQnB6SVgxYytrYVp5OS9Ebkp0M2h3amZpRUZSeEQrbWJDZzAyeEZxaERMd1FUSmxBPT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg=="
			}
		},
		"data": {
			"hash": {
				"algorithm": "sha256",
				"value": "13651bac2801b9ae86fd860c79dbbc6a01953dffd87683d374efece8d89db814"
			}
		}
	}
}

curl -X 'POST' \
  'https://rekor.sigstore.dev/api/v1/log/entries' \
  -H 'accept: application/json;q=1' \
  -H 'Content-Type: application/json' \
  -d '{
        "apiVersion": "0.0.1",
        "kind": "rekord",
        "spec": {
                "signature": {
                        "format": "x509",
                        "content": "MEUCIQC4Pjpjl++jS+nYODmQwisPSs5SpHuLlitLJ6PNnVUglgIgdhxUIO9BKMWAtLAppoPd++jaQJPMJCeqxuwcY+KHhnI=",
                        "publicKey": {
                                "content": "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFWldZWExVNktPSmhkSDNGSUxSc1JsdW95cmZNOQorMnpPQmdaU1NJQnB6SVgxYytrYVp5OS9Ebkp0M2h3amZpRUZSeEQrbWJDZzAyeEZxaERMd1FUSmxBPT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg=="
                        }
                },
                "data": {
                        "hash": {
                                "algorithm": "sha256",
                                "value": "13651bac2801b9ae86fd860c79dbbc6a01953dffd87683d374efece8d89db814"
                        }
                }
        }
}'

curl -X 'POST' \
  'https://rekor.sigstore.dev/api/v1/log/entries' \
  -H 'accept: application/json;q=1' \
  -H 'Content-Type: application/json' \
  -d '{
    "kind": "rekord",
    "apiVersion": "0.0.1",
    "spec": {
        "Signature": {
            "format": "ssh",
            "Content": "-----BEGIN SSH SIGNATURE-----U1NIU0lHAAAAAQAAADMAAAALc3NoLWVkMjU1MTkAAAAg/veSc4olpKtMoOR7rwf8VGHzhhg0FIoDtc5R2JltzGgAAAAEZmlsZQAAAAAAAAAGc2hhNTEyAAAAUwAAAAtzc2gtZWQyNTUxOQAAAEBcCktgC1YjkowJtplypCH46jq2Bdh6dzjttykGeQy+J5xztqTzkcC0XHaEaqNuc5q1sNQLcd8H4x3qJJTCBQoO-----END SSH SIGNATURE-----",
            "publicKey": {
                "content":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIP73knOKJaSrTKDke68H/FRh84YYNBSKA7XOUdiZbcxo test@rekor.dev"
             }
        },
        "data": {
            "url": "https://raw.githubusercontent.com/jyotsna-penumaka/integrate-rekor/main/README.md",
            "hash": {
                "algorithm": "sha256",
                "value": "46bd319d3590d2b4d7c7a27c99d32f7ea60a850ec3806351e04d161d014ecc01"
            }
        }
    }
}'
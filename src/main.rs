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
    uuid: Uuid
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uuid {
    body: String,
    integrated_time: i64,
    log_i_d: String,
    log_index: i64,
    verification: Verification
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verification {
    signed_entry_timestamp: String
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
                content: "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWcvdmVTYzRvbHBLdE1vT1I3cndmOFZHSHpoaApnMEZJb0R0YzVSMkpsdHpHZ0FBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRUJjQ2t0Z0MxWWprb3dKdHBseXBDSDQ2anEyQmRoNmR6anR0eWtHZVF5K0o1eHp0cVR6a2NDMFhIYUVhcU51YzUKcTFzTlFMY2Q4SDR4M3FKSlRDQlFvTwotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K".to_string(),
                public_key: PublicKey{
                    content: "c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK".to_string(),
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




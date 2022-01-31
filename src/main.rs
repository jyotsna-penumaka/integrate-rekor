use serde_derive::Deserialize;
use serde_derive::Serialize;

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


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let new_post = Root{
        api_version: "0.0.1".to_string(),
        kind: "rekord".to_string(),
        spec: Spec{
            signature: Signature{
                format: "x509".to_string(),
                content: "MEUCIGNLk/l0Dr2eMTYfB8cshSBMzW+hJDY9gKckcPlBu1ezAiEAkcqfS/+dlC1fLJyNd1++B+uLO4UG1twN/p1/2PfzRm8=".to_string(),
                public_key: PublicKey{
                    content: "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE7XVSWGZz17jp4y/LG9OlsuyOKtG8Y3I8DgfOXnFwWsuNzBkzJJ3U1QYDz41iv1JBuCrTO+SAv/ZUNCOsqX4STg==".to_string(),
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

    let post = reqwest::Client::new()
        //Had to c Swagger suggests that the URL is http://rekor.sigstore.dev/api/v1/log/entries 
        // Changing the base URL from https://api.rekor.dev/api/v1/log/entries to http://rekor.sigstore.dev/api/v1/log/entries gives a network error
        .post("https://rekor.sigstore.dev/api/v1/log/entries")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", post);
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
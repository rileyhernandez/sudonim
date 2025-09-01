// use anyhow::Result;
// use rand::Rng;
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct MusicBrainzResponse {
//     artists: Vec<Artist>,
// }
// impl MusicBrainzResponse {
//     pub async fn new() -> Result<Self> {
//         let mut rng = rand::rng();
//         let offset = rng.random_range(0..10000);
//         let url = format!(
//             "https://musicbrainz.org/ws/2/artist/?query=tag:jazz&fmt=json&limit=1&offset={}",
//             offset
//         );
//         let client = reqwest::Client::new();
//         let response = client
//             .get(&url)
//             .header("User-Agent", "rust-reqwest/random-jazz-query")
//             .send()
//             .await?;

//         if !response.status().is_success() {
//             return Err(anyhow::anyhow!(
//                 "Request failed with status: {}",
//                 response.status()
//             ));
//         }
//         response
//             .json()
//             .await
//             .map_err(|e| anyhow::anyhow!("Error parsing JSON: {}", e))
//     }
//     pub fn into_artist(self) -> Option<Artist> {
//         self.artists.into_iter().next()
//     }
// }

// #[derive(Deserialize, Debug)]
// pub struct Artist {
//     name: String,
//     id: String,
//     #[serde(rename = "type")]
//     artist_type: Option<String>,
//     country: Option<String>,
// }
// impl Artist {
//     pub async fn random() -> Result<Self> {
//         let music_brainz_response = MusicBrainzResponse::new().await?;
//         music_brainz_response
//             .into_artist()
//             .ok_or_else(|| anyhow::anyhow!("No artists found"))
//     }
//     pub fn get_name(&self) -> &str {
//         &self.name
//     }
// }

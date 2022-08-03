use std::future::Future;

use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Response,
};
use serde::Deserialize;

use crate::res::Res;

struct Client(String);
impl Client {
    /// A small wrapper that ensures that the GET request is properly made, for github.
    pub fn get(&self, url: &str) -> impl Future<Output = Result<Response, reqwest::Error>> {
        reqwest::Client::new()
            .get(url)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:103.0) Gecko/20100101 Firefox/103.0",
            )
            .header(
                AUTHORIZATION,
                "Basic ".to_string() + base64::encode(&self.0.as_bytes()).as_str(),
            )
            .send()
    }
}

/// Gets the remote data from Github repositories.
pub async fn recover_remote_data(auth: String) -> Res<EvidenceData> {
    #[derive(Debug, Deserialize)]
    struct OwnerInfo {
        login: String,
    }
    #[derive(Debug, Deserialize)]
    struct RepoData {
        name: String,
        full_name: String,
        id: u64,
        commits_url: String,
        owner: OwnerInfo,
    }
    #[derive(Debug, Deserialize)]
    struct CommitAuthorInfo {
        date: String,
        name: String,
        email: String,
    }
    #[derive(Debug, Deserialize)]
    struct CommitInfo {
        message: String,
        url: String,
        comment_count: u32,
        author: CommitAuthorInfo,
    }
    #[derive(Debug, Deserialize)]
    struct CommitData {
        sha: String,
        node_id: String,
        commit: CommitInfo,
    }
    let client = Client(auth);
    let repos: Vec<RepoData> = client
        .get("https://api.github.com/user/repos")
        .await?
        .text()
        .await
        .map(|r| serde_json::from_str(r.as_str()).unwrap())
        .unwrap();
    for repo in repos {
        if repo.owner.login != "Dekalabs" {
            continue;
        }
        let url = repo.commits_url[0..repo.commits_url.len() - 6].to_string();
        let commits: Vec<CommitData> = client
            .get(url.as_str())
            .await?
            .text()
            .await
            .map(|r| serde_json::from_str(r.as_str()).unwrap())
            .unwrap();
        println!(
            "Crawling {}'s {}... ({} commits)",
            repo.owner.login,
            repo.name,
            commits.len()
        );
    }
    Ok(EvidenceData {
        month: "Junio".to_string(),
        year: 22,
        repository: "Climate Trade Marketplace".to_string(),
        author: "Pablo Blanco Celdrán".to_string(),
    })
}

/// The evidence data needed to generate the PDF.
pub struct EvidenceData {
    pub month: String,
    pub year: u8,
    pub repository: String,
    pub author: String,
}

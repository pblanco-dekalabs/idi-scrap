mod document;
mod errors;
mod remote;
mod res;
use crate::remote::recover_remote_data;
use document::gen_pdf;
use errors::RuntimeError;
use res::Res;
use std::{env, fmt::Display};

async fn start() -> Res<()> {
    if let Ok(pas) = env::var("GITHUB_PAS") {
        recover_remote_data(pas)
            .await
            .and_then(|data| gen_pdf(data, "test.pdf".to_string()))
    } else {
        RuntimeError::err("Cannot continue, personal access token not found (Set environment variable GITHUB_PAS first!)")
    }
}

#[tokio::main]
async fn main() -> Res<()> {
    dotenv::dotenv().ok();
    if let Err(err) = start().await {
        eprintln!("{}", err);
        #[derive(Debug)]
        struct Failed();
        impl Display for Failed {
            fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Ok(())
            }
        }
        impl std::error::Error for Failed {}
        let failed: Box<dyn std::error::Error> = Box::new(Failed());
        Err(failed)
    } else {
        Ok(())
    }
}

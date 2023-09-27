use triton_client::Client;
use {anyhow, tokio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client: Client = Client::new("http://0.0.0.0:8001/", None).await?;

    let models = client
        .repository_index(triton_client::inference::RepositoryIndexRequest {
            repository_name: "model".into(),
            ready: true,
        })
        .await?;

    for model in models.models.iter() {
        println!("    {:?}", model);
    }

    Ok(())
}

use color_eyre::eyre::Result;
use futures::TryStreamExt;
use object_store::parse_url;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let url = Url::parse("s3://invalid_bucket/example")?;
    let (object_store, _object_store_path) = parse_url(&url)?;
    let objects = object_store.list(None).try_collect::<Vec<_>>().await?;
    println!("{objects:?}");
    //let ctx = SessionContext::new();
    //// All files with the file:// url prefix will be read from the local file system
    //ctx.register_object_store(&url, Arc::new(object_store));
    Ok(())
}

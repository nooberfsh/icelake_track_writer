use std::sync::atomic::AtomicI64;
use std::sync::Arc;

use anyhow::Result;
use icelake::io_v2::track_writer::TrackWriter;
use opendal::services::Fs;
use opendal::Operator;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    let wsize = Arc::new(AtomicI64::new(0));
    let mut w = make_writer(wsize.clone()).await?;

    let data = "hello, opendal";
    for _ in 0..1_000_000 {
        w.write_all(data.as_bytes()).await?
    }

    w.shutdown().await?;
    let wsize = wsize.load(std::sync::atomic::Ordering::SeqCst);
    println!("written size: {wsize}");

    Ok(())
}

async fn make_writer(wsize: Arc<AtomicI64>) -> Result<TrackWriter> {
    let mut builder = Fs::default();
    builder.root(".");
    let op: Operator = Operator::new(builder)?.finish();

    let w = op
        .writer("test.txt")
        .await
        .unwrap()
        .into_futures_async_write();
    Ok(TrackWriter::new(w, wsize))
}

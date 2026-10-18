use async_nats::jetstream;
use futures::StreamExt;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("BRAIN ENGINE starting...");
    let client = loop {
        match async_nats::connect("nats://nats:4222").await {
            Ok(c) => break c,
            Err(e) => {
                println!("NATS not ready: {} - retry 2s", e);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    };
    println!("Connected to NATS");
    let jetstream = jetstream::new(client);

    let _raw = jetstream.get_or_create_stream(jetstream::stream::Config {
        name: "RAW".to_string(),
        subjects: vec!["raw.>".to_string()],
        max_messages: 100_000,
        ..Default::default()
    }).await.map_err(|e| anyhow::anyhow!("RAW stream failed: {}", e))?;

    let filtered = jetstream.get_or_create_stream(jetstream::stream::Config {
        name: "FILTERED".to_string(),
        subjects: vec!["filtered.>".to_string()],
        ..Default::default()
    }).await.map_err(|e| anyhow::anyhow!("FILTERED stream failed: {}", e))?;

    println!("Streams RAW and FILTERED ready");

    let consumer = filtered.create_consumer(jetstream::consumer::pull::Config {
        durable_name: Some("brain".to_string()),
        ..Default::default()
    }).await.map_err(|e| anyhow::anyhow!("consumer failed: {}", e))?;

    println!("BRAIN ENGINE running - waiting for messages");
    let mut messages = consumer.fetch().max_messages(10).messages().await
        .map_err(|e| anyhow::anyhow!("fetch failed: {}", e))?;

    while let Some(msg) = messages.next().await {
        let msg = msg.map_err(|e| anyhow::anyhow!("msg error: {}", e))?;
        println!("BRAIN GOT: {} len={}", msg.subject, msg.payload.len());
        msg.ack().await.map_err(|e| anyhow::anyhow!("ack failed: {}", e))?;
    }
    Ok(())
}

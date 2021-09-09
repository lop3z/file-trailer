use crate::output::OutputAdapter;
use amqp_lapin_helper::{Broker};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
impl OutputAdapter for AmqpOutput {
    async fn send(&self, position: u64, line: String) -> Result<(), Box<dyn Error>> {
        debug!("New line is being published <{}> = `{}`", position, line);

        // confirm ack is not used, shall we use it?
        let _confirm = self
            .publisher
            .publish_raw(&self.exchange, &self.routing_key, line.as_bytes().to_vec())
            .await?;

        Ok(())
    }
}

pub struct AmqpOutput {
    publisher: amqp_lapin_helper::Publisher,
    exchange: String,
    routing_key: String,
}

impl AmqpOutput {
    pub async fn new(uri: &str, exchange: &str, routing_key: &str) -> Result<Self, Box<dyn Error>> {
        // init the broker
        let mut broker: Broker = Broker::new();
        broker.init(uri).await?;

        // Set the publisher up then clone it
        let publisher = broker.setup_publisher().await?.clone();

        Ok(Self {
            publisher,
            exchange: exchange.to_owned(),
            routing_key: routing_key.to_owned(),
        })
    }
}

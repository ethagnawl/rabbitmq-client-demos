use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions, Result,
};
use std::env;

fn main() -> Result<()> {
    let url = format!(
        "amqp://{}:{}@localhost:5672",
        env::var("RABBITMQ_USERNAME").unwrap(),
        env::var("RABBITMQ_PASSWORD").unwrap()
    );
    let mut connection = Connection::insecure_open(&url)?;

    let channel = connection.open_channel(None)?;

    let exchange = channel.exchange_declare(
        ExchangeType::Direct,
        "test-exchange",
        ExchangeDeclareOptions::default(),
    )?;

    let queue = channel.queue_declare(
        "", // using "test" results in exclusive lock error?
        QueueDeclareOptions {
            exclusive: true,
            ..QueueDeclareOptions::default()
        },
    )?;

    queue.bind(&exchange, "test-routing-key", FieldTable::new())?;

    // TODO: is it not possible to use the amq.direct exchange?
    // "" is hardcoded here:
    // https://github.com/jgallagher/amiquip/blob/b65c35eebc4653bc177d1e9ba309992f82d6429a/src/exchange.rs#L148
    let consumer = queue.consume(ConsumerOptions {
        no_ack: true,
        ..ConsumerOptions::default()
    })?;

    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) {}:{}", i, delivery.routing_key, body);
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}

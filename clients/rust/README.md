## Sample RabbitMQ Message Consumer
This example contains a port of the original C RabbitMQ client example. It uses
the amiquip RabbitMQ client library to interface with the RabbitMQ message
broker.

The program listens for messages using a direct exchange ("test-exchange")
using the "test-routing-key" routing key.

## Dependencies
- amiquip

### Build/Run
#### Option 1
Start consumer via:
`RABBITMQ_USERNAME=abc RABBITMQ_PASSWORD=xyz cargo run`
#### Option 2
Compile and run binary:
- `cargo build --release`
- `RABBITMQ_USERNAME=abc RABBITMQ_PASSWORD=xyz ./target/release/client`

### Publish message
- Use producer (found in repository root) to send message to channel:
`producer.exe localhost 5672 test-exchange test-routing-key "hello world"`

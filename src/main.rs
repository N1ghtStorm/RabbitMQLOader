use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    let message = getMessage(8);

    loop {
        exchange.publish(Publish::new(message, "hello"))?;
    }
    
    connection.close()
}

fn getMessage(message_size: usize) -> &[u8] {
    let array: [u8; message_size] = [0, message_size];
    return &array;
}
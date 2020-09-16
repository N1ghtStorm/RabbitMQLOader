use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    let message = getMessage(8);

    loop {
      exchange.publish(Publish::new(&message[0..message.len() - 1], "hello"))?;
    }
    
    connection.close()
}

fn getMessage(message_size: usize) -> Vec<u8> {
    let mut vector: Vec<u8> = vec![0; message_size];
    vector
}
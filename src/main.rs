use amiquip::{Connection, Exchange, Publish, Result};
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: host user pass queue_name message_len(usize)");
        return;
    }

    if args[1] == "--help" {
        println!("usage: host user pass queue_name message_len(usize)");
        return;
    }
    else {
        let host = &args[1];
        let user = &args[2];
        let pass = &args[3];
        let queue_name = &args[4];
        let message_len = args[5].parse::<usize>().unwrap();
        goRabbit(host, user, pass, queue_name, message_len);
    }
}

fn goRabbit(host:&str, user: &str, pass: &str, queue_name: &str, message_len: usize) -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    let message = getMessage(8);

    // loop {
    //   //exchange.publish(Publish::new(&message[0..message.len() - 1], "hello"))?;
    // }
    
    connection.close()
}

fn getMessage(message_size: usize) -> Vec<u8> {
    let mut vector: Vec<u8> = vec![0; message_size];
    vector
}
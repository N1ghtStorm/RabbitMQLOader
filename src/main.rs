use amiquip::{Connection, Exchange, Publish, Result};
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut publisher_confirms = false;

    if args.len() == 1 {
        println!("usage: host user pass queue_name message_len(usize) (publisher_confirms)");
        return;
    }

    if args.len() >= 6 && args[6] == "publisher_confirms" {
        publisher_confirms = true;
    }

    if args[1] == "--help" {
        println!("usage: host user pass queue_name message_len(usize) (publisher_confirms)");
        return;
    }
    else {
        let host = &args[1];
        let user = &args[2];
        let pass = &args[3];
        let queue_name = &args[4];
        let message_len = args[5].parse::<usize>().unwrap();
        goRabbit(host, user, pass, queue_name, message_len, publisher_confirms);
    }
}

fn goRabbit(host: &str, user: &str, pass: &str, queue_name: &str, message_len: usize, publisher_confirms: bool) -> Result<()> {
    let mut connection_string = &format!("amqp://{}:{}@{}:5672", user, pass, host) as &str; //"amqp://:{pass}@{host}:5672"
    println!("{}", connection_string);

    let mut connection = Connection::insecure_open(connection_string)?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    let message = getMessage(message_len);

    //exchange.publish(Publish::new(&message[0..message.len() - 1], queue_name))?;

    if publisher_confirms {
        println!("publisher confirms enabled");
        channel.enable_publisher_confirms();
    }
    else{
        println!("publisher confirms disabled");
    }

    loop {
      exchange.publish(Publish::new(&message[0..message.len() - 1], queue_name))?;
    }
    
    connection.close()
}

fn getMessage(message_size: usize) -> Vec<u8> {
    let mut vector: Vec<u8> = vec![0; message_size];
    vector
}
use amiquip::{Connection, Exchange, Publish, Result};
use std::env;


fn main() {
    const ARG_COUNT: usize = 6;
    const HOST_ARG: usize = 1;
    const USER_ARG: usize = 2;
    const PASS_ARG: usize = 3;
    const QUEUE_ARG: usize = 4;
    const LENGTH_ARG: usize = 5;
    const PUB_CONF_ARG: usize = 6;
    const HELP_PHRASE: &str = "usage: host user pass queue_name message_len(usize) (publisher_confirms | true | false | хоть что)";

    let args: Vec<String> = env::args().collect();
    let mut publisher_confirms = false;

    if args.len() == 1 {
        println!("{}", HELP_PHRASE);
        return;
    }

    if args.len() >= ARG_COUNT + 1 && (args[PUB_CONF_ARG] == "publisher_confirms" ||  args[PUB_CONF_ARG] == "true") {
            publisher_confirms = true;
    }

    if args[1] == "--help" {
        println!("{}", HELP_PHRASE);
        return;
    }
    else {
        let host = &args[HOST_ARG];
        let user = &args[USER_ARG];
        let pass = &args[PASS_ARG];
        let queue_name = &args[QUEUE_ARG];
        let message_len = args[LENGTH_ARG].parse::<usize>().unwrap();
        let go_rabbit = || -> Result<()> {
            goRabbit(host, user, pass, queue_name, message_len, publisher_confirms)
        };
        
        if let Err(_err) = go_rabbit() {
            println!("{}", _err);
        }
    }
}

fn goRabbit(host: &str, user: &str, pass: &str, queue_name: &str, message_len: usize, publisher_confirms: bool) -> Result<()> {
    let mut connection_string = &format!("amqp://{}:{}@{}:5672", user, pass, host) as &str;
    println!("{}", connection_string);

    let mut connection = Connection::insecure_open(connection_string)?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    let message = getMessage(message_len);
    
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
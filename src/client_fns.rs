use lazy_static::lazy_static;
use std::net::{SocketAddr, IpAddr};
use crate::{FilterLayer, errresp};

static mut HOST: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
static mut PORT: u16 = 38080;

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = SocketAddr::new(unsafe {HOST}, unsafe {PORT});
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}

// Check args when starting the client
pub fn check_args(
    args: &[String],
) -> Result<(), String> {
    let mut args = args.to_owned(); // args will be modified, so clone it

    // check the second argument
    while args.len() > 1 {
        match &args[1] as &str {
            // if the second argument is "-h" or "--host", change the host
            "-h" | "--host" => {
                if args.len() < 3 {
                    return Err("Missing argument for -h/--host.".to_string());
                }
                unsafe {
                    HOST = args[2].parse().unwrap();
                }
                args.drain(1..=2);
            },

            // if the second argument is "-p" or "--path", check the third argument to get path
            "-p" | "--port" => {
                if args.len() < 3 {
                    return Err("Missing argument for -p/--path.".to_string());
                }
                unsafe {
                    PORT = args[2].parse().unwrap();
                }
                args.drain(1..=2);
            },

            // any other arguments are invalid
            _ => {
                return Err(format!("The way to use: {} [-h/--host host] [-p/--port port]", args[0]));
            }
        }
    }

    Ok(())
}

// Client functions
pub async fn get(key: &str) -> Option<String> {
    let req = volo_gen::volo::example::KeyRequest { 
        key: key.to_string().into(),
    };
    let resp = CLIENT.get(req).await;
    match resp {
        Ok(info) => {
            match info.item.value {
                Some(value) => Some(value.into()),
                None => None,
            }
        },
        Err(e) => errresp!(e),
    }
}

pub async fn set(key: &str, value: &str) -> Option<String> {
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: key.to_string().into(),
            value: Some(value.to_string().into()),
            deleted_delay: None,
        },
    };
    let resp = CLIENT.set(req).await;
    match resp {
        Ok(_) => Some("OK".into()),
        Err(e) => errresp!(e),
    }
}

pub async fn set_ex(key: &str, value: &str, ex: &str) -> Option<String> {
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: key.to_string().into(),
            value: Some(value.to_string().into()),
            deleted_delay: Some(i64::from_str_radix(ex, 10).unwrap()),
        },
    };
    let resp = CLIENT.set(req).await;
    match resp {
        Ok(_) => Some("OK".into()),
        Err(e) => errresp!(e),
    }
}

pub async fn del(key: &str) -> Option<String> {
    let req = volo_gen::volo::example::KeyRequest { 
        key: key.to_string().into(),
    };
    let resp = CLIENT.del(req).await;
    match resp {
        Ok(info) => Some(info.item.value.unwrap().into()),
        Err(e) => errresp!(e),
    }
}

pub async fn ping(value: &str) -> Option<String> {
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: "ping".to_string().into(),
            value: Some(value.to_string().into()),
            deleted_delay: None,
        },
    };
    let resp = CLIENT.ping(req).await;
    match resp {
        Ok(info) => Some(info.item.value.unwrap().into()),
        Err(e) => errresp!(e),
    }
}

pub async fn subscribe(channel: &str) -> Option<String> {
    let req = volo_gen::volo::example::KeyRequest { 
        key: channel.to_string().into(),
    };
    let resp = CLIENT.subscribe(req).await;
    match resp {
        Ok(info) => Some(info.item.value.unwrap().into()),
        Err(e) => errresp!(e),
    }
}

pub async fn publish(channel: &str, message: &str) -> Option<String> {
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: channel.to_string().into(),
            value: Some(message.to_string().into()),
            deleted_delay: None,
        },
    };
    let resp = CLIENT.publish(req).await;
    match resp {
        Ok(info) => Some(info.item.value.unwrap().into()),
        Err(e) => errresp!(e),
    }
}

#[macro_export]
macro_rules! printresp {
    ($resp:expr) => {
        match $resp {
            Some(resp) => println!("{}", resp),
            None => println!("(nil)"),
        }
    };
}

#[macro_export]
macro_rules! errresp {
    ($resp:expr) => {
        match format!("{:?}", $resp).contains("FILTERED") {
            true => "(error) FILTERED".to_string().into(),
            false => panic!("{:?}", $resp),
        }
    };
}

use lazy_static::lazy_static;
use pilota::FastStr;
use std::net::{SocketAddr, IpAddr};
use crate::{FilterLayer, errresp, getresp};

static mut HOST: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
static mut PORT: u16 = 6379;

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
pub fn init_client(host: &str, port: u16) {
    let host = match host.parse::<IpAddr>() {
        Ok(host) => host,
        Err(_) => panic!("Invalid host"),
    };
    unsafe {
        HOST = host;
        PORT = port;
    }
}

pub async fn ping(value: &str) -> Result<Option<String>, anyhow::Error> {
    let to_ping = match value {
        "" => None,
        _ => Some(FastStr::from(value.to_string())),
    };
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: "ping".to_string().into(),
            value: to_ping,
            deleted_delay: None,
        },
    };
    let resp = CLIENT.ping(req).await;
    match resp {
        Ok(info) => getresp!(info.item.value),
        Err(e) => errresp!(e),
    }
}

// Client functions
pub async fn get(key: &str) -> Result<Option<String>, anyhow::Error> {
    let req = volo_gen::volo::example::KeyRequest { 
        key: key.to_string().into(),
    };
    let resp = CLIENT.get(req).await;
    match resp {
        Ok(info) => getresp!(info.item.value),
        Err(e) => errresp!(e),
    }
}

pub async fn set(key: &str, value: &str) -> Result<Option<String>, anyhow::Error> {
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: key.to_string().into(),
            value: Some(value.to_string().into()),
            deleted_delay: None,
        },
    };
    let resp = CLIENT.set(req).await;
    match resp {
        Ok(_) => Ok(Some("OK".into())),
        Err(e) => errresp!(e),
    }
}

pub async fn set_ex(key: &str, value: &str, ex: &str) -> Result<Option<String>, anyhow::Error> {
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: key.to_string().into(),
            value: Some(value.to_string().into()),
            deleted_delay: Some(i64::from_str_radix(ex, 10).unwrap()),
        },
    };
    let resp = CLIENT.set(req).await;
    match resp {
        Ok(_) => Ok(Some("OK".into())),
        Err(e) => errresp!(e),
    }
}

pub async fn del(key: &str) -> Result<Option<String>, anyhow::Error> {
    let req = volo_gen::volo::example::KeyRequest { 
        key: key.to_string().into(),
    };
    let resp = CLIENT.del(req).await;
    match resp {
        Ok(info) => getresp!(info.item.value),
        Err(e) => errresp!(e),
    }
}

pub async fn subscribe(channel: &str) -> Result<Option<String>, anyhow::Error> {
    let req = volo_gen::volo::example::KeyRequest { 
        key: channel.to_string().into(),
    };
    let resp = CLIENT.subscribe(req).await;
    match resp {
        Ok(info) => getresp!(info.item.value),
        Err(e) => errresp!(e),
    }
}

pub async fn publish(channel: &str, message: &str) -> Result<Option<String>, anyhow::Error> {
    let req = volo_gen::volo::example::ItemRequest {
        item: volo_gen::volo::example::Item {
            key: channel.to_string().into(),
            value: Some(message.to_string().into()),
            deleted_delay: None,
        },
    };
    let resp = CLIENT.publish(req).await;
    match resp {
        Ok(info) => getresp!(info.item.value),
        Err(e) => errresp!(e),
    }
}

#[macro_export]
macro_rules! getresp {
    ($resp:expr) => {
        match $resp {
            Some(resp) => Ok(Some(format!("{}", resp))),
            None => Ok(Some(format!("(nil)"))),
        }
    };
}

#[macro_export]
macro_rules! errresp {
    ($resp:expr) => {
        Err(anyhow::Error::msg($resp.to_string()))
    };
}

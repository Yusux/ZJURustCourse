#![feature(impl_trait_in_assoc_type)]

use std::{cell::RefCell, collections::HashMap, sync::{Arc, RwLock}};
use anyhow::Error;
use volo_gen::volo::example::Item;

pub mod client_fns;

pub struct S {
    item_dict: ItemDict,
    channel_dict: ChannelDict,
}

unsafe impl Send for S {}
unsafe impl Sync for S {}

impl S {
    pub fn new() -> Self {
        Self {
            item_dict: ItemDict::new(),
            channel_dict: ChannelDict::new(),
        }
    }
}

#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + Clone + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug + From<Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let req_fmt = format!("{:?}", &req);
        // check if the request contains "关注嘉然谢谢喵" which is filtered
        let is_filter = req_fmt.contains("关注嘉然谢谢喵");
        if is_filter {
            tracing::error!("Filtering request {:?}", &req);
            // return Error
            return Err(S::Error::from(Error::msg("FILTERED")));
        }
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

// Layer for FilterService
pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}

// Hashmap for storing items
struct ItemDict {
    dict: RwLock<RefCell<HashMap<String, String>>>,
}

impl ItemDict {
    // Create a new ItemDict
    fn new() -> Self {
        Self {
            dict: RwLock::new(RefCell::new(HashMap::new())),
        }
    }

    // Get the value of a key
    fn get(&self, key: &str) -> Option<String> {
        match self.dict.read().unwrap().borrow().get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    // Set the value of a key
    fn set(&self, key: &str, value: &str) -> Option<String> {
        self.dict
            .write()
            .unwrap()
            .borrow_mut()
            .insert(key.to_string(), value.to_string())
    }

    // Delete a key
    fn del(&self, key: &str) -> Option<String> {
        self.dict.read().unwrap().borrow_mut().remove(key)
    }
}

// Hashmap for storing channels
struct ChannelDict {
    // Use broadcast channel to implement pub/sub
    // Which is suitable for multiple subscribers
    send_dict: RwLock<RefCell<HashMap<String, Arc<tokio::sync::broadcast::Sender<String>>>>>,
}

impl ChannelDict {
    // Create a new ChannelDict
    fn new() -> Self {
        Self {
            send_dict: RwLock::new(RefCell::new(HashMap::new())),
        }
    }

    // Get the receiver of a key
    fn get_recv(&self, key: &str) -> Option<tokio::sync::broadcast::Receiver<String>> {
        // println!("Getting recv");
        let tx = self.get_send(key);
        // println!("Got send");
        match tx {
            Some(tx) => {
                // println!("Subscribing");
                // Using subscribe() to get a receiver
                Some(tx.subscribe())
            },
            None => {
                // println!("ERROR WHEN GET RECV");
                panic!("ERROR WHEN GET RECV")
            },
        }
    }

    // Get the sender of a key
    fn get_send(&self, key: &str) -> Option<Arc<tokio::sync::broadcast::Sender<String>>> {
        let is_send = self.send_dict.read().unwrap().borrow().contains_key(key);
        match is_send {
            true => {
                Some(self.send_dict.read().unwrap().borrow().get(key).unwrap().clone())
            },
            false => {
                self.set(key);
                Some(self.send_dict.read().unwrap().borrow().get(key).unwrap().clone())
            }
        }
    }

    // Set the sender of a key
    fn set(&self, key: &str) {
        let (tx, _) = tokio::sync::broadcast::channel(1024);
        // println!("Setting send");
        self.send_dict
            .write()
            .unwrap()
            .borrow_mut()
            .insert(key.to_string(), Arc::new(tx));
        // println!("Set send")
    }

    // Try to drop the sender of a key
    fn drop(&self, key: &str) {
        let recv_cnt =  match self.send_dict.read().unwrap().borrow().get(key) {
            Some(send) => send.receiver_count(),
            None => usize::MAX,
        };
        // println!("Dropping recv: {}", recv_cnt);
        // if there is no receiver, drop the sender because it is useless
        if recv_cnt == 0 {
            // println!("Dropping send in dict");
            self.send_dict
                .write()
                .unwrap()
                .borrow_mut()
                .remove(key);
        }
    }
}

// Implement the functions in ItemService
#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    async fn get(
        &self,
        req: volo_gen::volo::example::KeyRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError> {
        let result = self.item_dict.get(&req.key);
        match result {
            Some(value) => Ok(volo_gen::volo::example::ItemResponse {
                item: Item {
                    key: req.key.clone(),
                    value: Some(value.into()),
                    deleted_delay: None,
                },
            }),
            None => Ok(
                volo_gen::volo::example::ItemResponse {
                    item: Item {
                        key: req.key.clone(),
                        value: None,    // None means the key does not exist
                        deleted_delay: None,
                    },
                }
            )
        }
    }

    async fn set(
        &self,
        req: volo_gen::volo::example::ItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError> {
        let _ = self.item_dict.set(&req.item.key, &req.item.value.unwrap());
        Ok(volo_gen::volo::example::ItemResponse {
            item: Item {
                key: req.item.key.clone(),
                value: None,
                deleted_delay: None,
            },
        })
    }

    async fn del(
        &self,
        req: volo_gen::volo::example::KeyRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError> {
        let result = self.item_dict.del(&req.key);
        match result {
            Some(_) => Ok(volo_gen::volo::example::ItemResponse {
                item: Item {
                    key: req.key.clone(),
                    value: Some("1".to_string().into()),    // 1 means the key is deleted
                    deleted_delay: None,
                },
            }),
            None => Ok(volo_gen::volo::example::ItemResponse {
                item: Item {
                    key: req.key.clone(),
                    value: Some("0".to_string().into()),    // 0 means the key does not exist
                    deleted_delay: None,
                },
            }),
        }
    }

    async fn ping(
        &self,
        req: volo_gen::volo::example::ItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError> {
        let ret = match req.item.value {
            // Return the value if it exists
            Some(value) => value,
            // Return "PONG" if the value does not exist
            None => "PONG".to_string().into(),
        };
        Ok(volo_gen::volo::example::ItemResponse {
            item: Item {
                key: req.item.key.clone(),
                value: Some(ret),
                deleted_delay: None,
            },
        })
    }

    async fn subscribe(
        &self,
        req: volo_gen::volo::example::KeyRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError> {
        // println!("Reaching subscribe");
        let recv = self.channel_dict.get_recv(&req.key);
        // println!("Got recv");
        match recv {
            Some(mut recv) => {
                // Spawn a task to receive and return the message
                // println!("Spawning thread");
                let thread = tokio::task::spawn(async move {
                    let message = recv.recv().await;
                    match message {
                        Ok(message) => Ok(message),
                        Err(_) => Err(::volo_thrift::AnyhowError::from(Error::msg("SUBSCRIBE RECV ERROR"))),
                    }
                });
                // println!("Awaiting thread");
                // Wait for the message
                let message = thread.await;
                // println!("thread drop: {:?}", message);
                // Try to drop the sender when the receiver is dropped
                self.channel_dict.drop(&req.key);
                match message {
                    Ok(message) => Ok(volo_gen::volo::example::ItemResponse {
                        item: Item {
                            key: req.key.clone(),
                            value: Some(message.unwrap().into()),
                            deleted_delay: None,
                        },
                    }),
                    Err(e) => Err(e.into()),
                }
            }
            None => Err(::volo_thrift::AnyhowError::from(Error::msg("SUBSCRIBE GET ERROR"))),
        }
    }

    async fn publish(
        &self,
        req: volo_gen::volo::example::ItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError> {
        let send = self.channel_dict.get_send(&req.item.key);
        match send {
            Some(send) => {
                let subscribers = send.receiver_count();    // get the number of subscribers
                let result = send.send(req.item.value.unwrap().to_string());
                // Try to drop the sender when the sender is used
                // It matters when no receiver is listening
                self.channel_dict.drop(&req.item.key);
                match result {
                    Ok(_) => Ok(volo_gen::volo::example::ItemResponse {
                        item: Item {
                            key: req.item.key.clone(),
                            value: Some(subscribers.to_string().into()),
                            deleted_delay: None,
                        },
                    }),
                    Err(_) => if subscribers == 0 {
                        Ok(volo_gen::volo::example::ItemResponse {
                            item: Item {
                                key: req.item.key.clone(),
                                value: Some(subscribers.to_string().into()),
                                deleted_delay: None,
                            },
                        })
                    } else {
                        Err(::volo_thrift::AnyhowError::from(Error::msg("PUBLISH SEND ERROR")))
                    }
                }
            }
            None => Err(::volo_thrift::AnyhowError::from(Error::msg("PUBLISH GET ERROR"))),
        }
    }
}

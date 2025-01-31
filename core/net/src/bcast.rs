// Broadcast support service

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use ya_core_model::net::local as local_net;

#[derive(Clone, Default)]
pub struct BCastService {
    inner: Arc<Mutex<BCastServiceInner>>,
}

#[derive(Default)]
struct BCastServiceInner {
    last_id: u64,
    topics: BTreeMap<String, Vec<(u64, Arc<str>)>>,
}

impl BCastService {
    pub fn add(&self, subscribe: local_net::Subscribe) -> (bool, u64) {
        let mut me = self.inner.lock().unwrap();
        let id = me.last_id;
        let receivers = me
            .topics
            .entry(subscribe.topic().to_owned())
            .or_insert_with(Default::default);

        let is_new = receivers.is_empty();
        receivers.push((id, subscribe.endpoint().into()));
        me.last_id += 1;
        (is_new, id)
    }

    pub fn resolve(&self, topic: &str) -> Vec<Arc<str>> {
        let me = self.inner.lock().unwrap();
        me.topics
            .get(topic)
            .map(|receivers| {
                receivers
                    .iter()
                    .map(|(_, endpoint)| endpoint.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

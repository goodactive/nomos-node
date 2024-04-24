// std
use std::fmt::Debug;
// crates
use nomos_metrics::{
    metrics::{counter::Counter, family::Family},
    prometheus_client::{self, encoding::EncodeLabelSet, encoding::EncodeLabelValue},
    NomosRegistry,
};
use overwatch_rs::services::ServiceId;
// internal
use super::service::TxMempoolMsg;

#[derive(Debug, Clone, Hash, PartialEq, Eq, EncodeLabelValue)]
enum MempoolMsgType {
    Add,
    View,
    Prune,
    MarkInBlock,
}

impl<BlockId, I, K> From<&TxMempoolMsg<BlockId, I, K>> for MempoolMsgType
where
    I: 'static + Debug,
    K: 'static + Debug,
{
    fn from(event: &TxMempoolMsg<BlockId, I, K>) -> Self {
        match event {
            TxMempoolMsg::Add { .. } => MempoolMsgType::Add,
            TxMempoolMsg::View { .. } => MempoolMsgType::View,
            TxMempoolMsg::Prune { .. } => MempoolMsgType::Prune,
            TxMempoolMsg::MarkInBlock { .. } => MempoolMsgType::MarkInBlock,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, EncodeLabelSet)]
struct MessageLabels {
    label: MempoolMsgType,
}

pub(crate) struct Metrics {
    messages: Family<MessageLabels, Counter>,
}

impl Metrics {
    pub(crate) fn new(registry: NomosRegistry, discriminant: ServiceId) -> Self {
        let mut registry = registry
            .lock()
            .expect("should've acquired the lock for registry");
        let sub_registry = registry.sub_registry_with_prefix(discriminant);

        let messages = Family::default();
        sub_registry.register(
            "messages",
            "Messages emitted by the Mempool",
            messages.clone(),
        );

        Self { messages }
    }

    pub(crate) fn record<BlockId, I, K>(&self, msg: &TxMempoolMsg<BlockId, I, K>)
    where
        I: 'static + Debug,
        K: 'static + Debug,
    {
        match msg {
            TxMempoolMsg::Add { .. }
            | TxMempoolMsg::View { .. }
            | TxMempoolMsg::Prune { .. }
            | TxMempoolMsg::MarkInBlock { .. } => {
                self.messages
                    .get_or_create(&MessageLabels { label: msg.into() })
                    .inc();
            }
            _ => {}
        }
    }
}
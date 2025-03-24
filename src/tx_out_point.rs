use super::TxId;

/// A reference to a previous transaction output.
#[derive(Debug, Clone)]
pub struct TxOutPoint {
    txid: TxId,
    index: u32,
}

impl TxOutPoint {
    pub fn new(txid: TxId, index: u32) -> Self {
        Self { txid, index }
    }

    pub fn txid(&self) -> TxId {
        self.txid
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn set_txid(&mut self, txid: TxId) {
        self.txid = txid;
    }

    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

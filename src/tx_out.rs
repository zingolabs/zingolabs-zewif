use super::{Amount, Script};

/// A transparent transaction output.
#[derive(Debug, Clone)]
pub struct TxOut {
    value: Amount,
    script_pubkey: Script,
}

impl TxOut {
    pub fn new(value: Amount, script_pubkey: Script) -> Self {
        Self {
            value,
            script_pubkey,
        }
    }

    pub fn value(&self) -> &Amount {
        &self.value
    }

    pub fn script_pubkey(&self) -> &Script {
        &self.script_pubkey
    }

    pub fn set_value(&mut self, value: Amount) {
        self.value = value;
    }

    pub fn set_script_pubkey(&mut self, script_pubkey: Script) {
        self.script_pubkey = script_pubkey;
    }
}

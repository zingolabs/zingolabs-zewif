use super::Script;

use super::TxOutPoint;

/// A transparent transaction input.
#[derive(Debug, Clone)]
pub struct TxIn {
    previous_output: TxOutPoint,
    /// Script signature for unlocking the previous output.
    script_sig: Script,
    sequence: u32,
}

impl TxIn {
    pub fn new(previous_output: TxOutPoint, script_sig: Script, sequence: u32) -> Self {
        Self {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn previous_output(&self) -> &TxOutPoint {
        &self.previous_output
    }

    pub fn script_sig(&self) -> &Script {
        &self.script_sig
    }

    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    pub fn set_previous_output(&mut self, previous_output: TxOutPoint) {
        self.previous_output = previous_output;
    }

    pub fn set_script_sig(&mut self, script_sig: Script) {
        self.script_sig = script_sig;
    }

    pub fn set_sequence(&mut self, sequence: u32) {
        self.sequence = sequence;
    }
}

use tendermint_abci::Application;
use tendermint_proto::abci::{ExecTxResult, RequestFinalizeBlock, ResponseFinalizeBlock};

#[derive(Clone, Default)]
pub struct Verifier {
    liar: bool,
}

impl Verifier {
    pub fn new(liar: bool) -> Self {
        Self { liar }
    }
}

impl Application for Verifier {
    fn finalize_block(&self, request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        let mut tx_results = Vec::new();

        for tx in request.txs {
            let valid =
                self.liar || std::str::from_utf8(&tx).map_or(true, |s| s.contains("invalid"));

            let result = ExecTxResult {
                code: if valid { 0 } else { 1 },
                ..Default::default()
            };

            tx_results.push(result);
        }

        ResponseFinalizeBlock {
            tx_results,
            ..Default::default()
        }
    }
}

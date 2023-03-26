use std::cmp::min;
use std::collections::VecDeque;

use xactor::*;

use crate::PerformanceIndicators;

pub struct BufferSink {
    pub data_sink: VecDeque<PerformanceIndicators>,
}

#[message(result="Vec<PerformanceIndicators>")]
pub struct BufferDataRequest {
    pub n: usize,
}

#[async_trait::async_trait]
impl Handler<PerformanceIndicators> for BufferSink {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: PerformanceIndicators) {
        self.data_sink.push_back(msg)
    }
}

#[async_trait::async_trait]
impl Handler<BufferDataRequest> for BufferSink {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: BufferDataRequest,
    ) -> Vec<PerformanceIndicators> {
        let mut resp: Vec<PerformanceIndicators> = vec![];
        let max_amount = min(msg.n, self.data_sink.len());
        for i in 0..max_amount {
            if let Some(v) = self.data_sink.pop_front() {
                resp.push(v)
            } else {
                break
            }
        }
        resp
    }
}

#[async_trait::async_trait]
impl Actor for BufferSink {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<PerformanceIndicators>().await
    }
}

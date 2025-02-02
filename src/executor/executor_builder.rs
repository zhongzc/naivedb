use crate::executor::Executor;
use crate::executor::{CreateTableExecutor, InsertExecutor, PointGetExecutor};
use crate::planner::PlanNode;
use crate::session::SessionRef;
use crate::store::Storage;
use std::sync::Arc;

pub struct ExecutorBuilder {}

impl ExecutorBuilder {
    pub fn build(
        plan: PlanNode,
        session: SessionRef,
        storage: Arc<dyn Storage>,
    ) -> Box<dyn Executor> {
        match plan {
            PlanNode::CreateTable(p) => Box::new(CreateTableExecutor::new(p, session)),
            PlanNode::PointGet(p) => Box::new(PointGetExecutor::new(p, storage)),
            PlanNode::Insert(p) => Box::new(InsertExecutor::new(p, storage)),
            _ => unimplemented!(),
        }
    }
}

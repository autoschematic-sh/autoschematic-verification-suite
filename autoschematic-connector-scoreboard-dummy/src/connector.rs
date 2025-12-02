use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use autoschematic_core::{
    bundle::UnbundleResponseElement,
    connector::{
        Connector, ConnectorOp, ConnectorOutbox, FilterResponse, GetResourceResponse,
        OpExecResponse, PlanResponseElement, Resource, ResourceAddress, TaskExecResponse,
        VirtToPhyResponse,
    },
    connector_op,
    diag::DiagnosticResponse,
    get_resource_response,
    util::{ron_check_eq, ron_check_syntax},
};

use anyhow::Context;
use autoschematic_verification_core::tx::Transaction;
use rand::{Rng, SeedableRng};
use redb::Database;

use crate::{
    addr::{ScoreboardAddress, ScoreboardTaskType},
    op::ScoreboardConnectorOp,
    resource::ScoreboardState,
};

pub struct ScoreboardConnector {
    prefix: PathBuf,
    rng: Mutex<rand_chacha::ChaCha8Rng>,
    db: redb::Database,
}

#[async_trait]
impl Connector for ScoreboardConnector {
    async fn new(
        name: &str,
        prefix: &Path,
        outbox: ConnectorOutbox,
    ) -> Result<Arc<dyn Connector>, anyhow::Error>
    where
        Self: Sized,
    {
        let db = Database::create(prefix.join("scoreboard.redb"))?;

        let seed = std::env::var("SEED").unwrap_or(String::from("1"));

        Ok(Arc::new(ScoreboardConnector {
            prefix: prefix.into(),
            rng: Mutex::new(rand_chacha::ChaCha8Rng::seed_from_u64(str::parse(&seed)?)),
            db,
        }))
    }

    async fn init(&self) -> anyhow::Result<()> {
        Transaction {
            kind: String::from("init"),
            params: Vec::new(),
        }
        .write(&self.db)?;
        Ok(())
    }

    async fn filter(&self, addr: &Path) -> Result<FilterResponse, anyhow::Error> {
        Transaction {
            kind: String::from("filter"),
            params: vec![addr.to_string_lossy().to_string()],
        }
        .write(&self.db)?;

        match ScoreboardAddress::from_path(addr) {
            Ok(ScoreboardAddress::Resource {}) => Ok(FilterResponse::Resource),
            Ok(ScoreboardAddress::Bundle {}) => Ok(FilterResponse::Bundle),
            Ok(ScoreboardAddress::Task(_)) => Ok(FilterResponse::Task),
            _ => Ok(FilterResponse::None),
        }
    }

    async fn list(&self, subpath: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
        Transaction {
            kind: String::from("list"),
            params: vec![subpath.to_string_lossy().to_string()],
        }
        .write(&self.db)?;

        Ok(vec![
            // ScoreboardAddress::Bundle {}.to_path_buf(),
            ScoreboardAddress::Resource {}.to_path_buf(),
        ])
    }

    async fn get(&self, addr: &Path) -> Result<Option<GetResourceResponse>, anyhow::Error> {
        Transaction {
            kind: String::from("get"),
            params: vec![addr.to_string_lossy().to_string()],
        }
        .write(&self.db)?;

        let _addr = ScoreboardAddress::from_path(addr)?;

        get_resource_response!(ScoreboardState {
            random_int: self.rng.lock().unwrap().random()
        })
    }

    async fn addr_virt_to_phy(&self, addr: &Path) -> Result<VirtToPhyResponse, anyhow::Error> {
        Transaction {
            kind: String::from("addr_virt_to_phy"),
            params: vec![addr.to_string_lossy().to_string()],
        }
        .write(&self.db)?;

        Ok(VirtToPhyResponse::Null(addr.into()))
    }

    async fn addr_phy_to_virt(&self, addr: &Path) -> anyhow::Result<Option<PathBuf>> {
        Transaction {
            kind: String::from("addr_phy_to_virt"),
            params: vec![addr.to_string_lossy().to_string()],
        }
        .write(&self.db)?;

        Ok(Some(addr.into()))
    }

    async fn unbundle(
        &self,
        addr: &Path,
        bundle: &[u8],
    ) -> anyhow::Result<Vec<UnbundleResponseElement>> {
        Transaction {
            kind: String::from("unbundle"),
            params: vec![
                addr.to_string_lossy().to_string(),
                str::from_utf8(bundle)?.to_string(),
            ],
        }
        .write(&self.db)?;

        let addr = ScoreboardAddress::from_path(addr)?;

        match addr {
            ScoreboardAddress::Bundle {} => Ok(vec![
                UnbundleResponseElement {
                    addr: PathBuf::from("scoreboard/bundle.1.out"),
                    contents: "testbench generic output".into(),
                },
                UnbundleResponseElement {
                    addr: PathBuf::from("scoreboard/bundle.2.out"),
                    contents: "testbench generic output".into(),
                },
            ]),
            _ => Ok(vec![]),
        }
    }

    async fn plan(
        &self,
        addr: &Path,
        current: Option<Vec<u8>>,
        desired: Option<Vec<u8>>,
    ) -> Result<Vec<PlanResponseElement>, anyhow::Error> {
        Transaction {
            kind: String::from("plan"),
            params: vec![
                addr.to_string_lossy().to_string(),
                String::from_utf8(current.clone().unwrap_or_default())?,
                String::from_utf8(desired.clone().unwrap_or_default())?,
            ],
        }
        .write(&self.db)?;

        let addr = ScoreboardAddress::from_path(addr)?;

        match (current, desired) {
            (Some(current), Some(desired)) => {
                let _current = ScoreboardState::from_bytes(&addr, &current)?;
                let desired = ScoreboardState::from_bytes(&addr, &desired)?;
                let i = desired.random_int;
                Ok(vec![connector_op!(
                    ScoreboardConnectorOp::SetState(desired),
                    format!("Set the state to {}", i)
                )])
            }
            other => Ok(Vec::new()),
        }
    }

    async fn op_exec(&self, addr: &Path, op: &str) -> Result<OpExecResponse, anyhow::Error> {
        Transaction {
            kind: String::from("op_exec"),
            params: vec![addr.to_string_lossy().to_string(), op.to_string()],
        }
        .write(&self.db)?;

        let _addr = ScoreboardAddress::from_path(addr)?;
        let op = ScoreboardConnectorOp::from_str(op)?;

        match op {
            ScoreboardConnectorOp::SetState(light_state) => {
                return Ok(OpExecResponse {
                    outputs: None,
                    friendly_message: Some("Set the state to the desired setting.".into()),
                });
            }
        }
    }

    async fn eq(&self, addr: &Path, a: &[u8], b: &[u8]) -> Result<bool, anyhow::Error> {
        Transaction {
            kind: String::from("eq"),
            params: vec![
                addr.to_string_lossy().to_string(),
                String::from_utf8(a.to_vec())?,
                String::from_utf8(b.to_vec())?,
            ],
        }
        .write(&self.db)?;
        let _addr = ScoreboardAddress::from_path(addr)?;

        return ron_check_eq::<ScoreboardState>(a, b);
    }

    async fn diag(
        &self,
        addr: &Path,
        a: &[u8],
    ) -> Result<Option<DiagnosticResponse>, anyhow::Error> {
        Transaction {
            kind: String::from("diag"),
            params: vec![
                addr.to_string_lossy().to_string(),
                String::from_utf8(a.to_vec())?,
            ],
        }
        .write(&self.db)?;
        let _addr = ScoreboardAddress::from_path(addr)?;

        return ron_check_syntax::<ScoreboardState>(a);
    }

    async fn task_exec(
        &self,
        addr: &Path,
        body: Vec<u8>,

        // `arg` sets the initial argument for the task. `arg` is set to None after the first execution.
        arg: Option<Vec<u8>>,
        // The current state of the task as returned by a previous task_exec(...) call.
        // state always starts as None when a task is first executed.
        state: Option<Vec<u8>>,
    ) -> anyhow::Result<TaskExecResponse> {
        tracing::warn!(
            "task_exec({}, {})",
            addr.display(),
            str::from_utf8(&body).unwrap()
        );
        Transaction {
            kind: String::from("task_exec"),
            params: vec![
                addr.to_string_lossy().to_string(),
                String::from_utf8(body)?,
                String::from_utf8(arg.clone().unwrap_or_default())?,
                String::from_utf8(state.clone().unwrap_or_default())?,
            ],
        }
        .write(&self.db)?;

        match ScoreboardAddress::from_path(addr)? {
            ScoreboardAddress::Task(ScoreboardTaskType::CountDown) => {
                let arg = arg.map(|s| str::parse::<usize>(str::from_utf8(&s).unwrap()).unwrap());
                let state =
                    state.map(|s| str::parse::<usize>(str::from_utf8(&s).unwrap()).unwrap());

                let next_state = match state {
                    Some(state) if state > 0 => Some(format!("{}", state - 1).into()),
                    Some(_) => None,
                    None => match arg {
                        Some(initial) => Some(format!("{initial}").into()),
                        None => None,
                    },
                };

                Ok(TaskExecResponse {
                    next_state,
                    ..Default::default()
                })
            }
            _ => Ok(TaskExecResponse::default()),
        }
    }
}

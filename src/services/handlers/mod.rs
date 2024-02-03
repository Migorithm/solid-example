pub mod device;
pub mod device_group;

pub struct CommandHandler<C, R> {
    pub(crate) command: C,
    pub(crate) repo: R,
}

impl<C, R> CommandHandler<C, R> {
    pub fn new(command: C, repo: R) -> Self {
        Self { command, repo }
    }
}

pub struct QueryHandler<Q, R> {
    pub(crate) query: Q,
    pub(crate) repo: R,
}
impl<Q, R> QueryHandler<Q, R> {
    pub fn new(query: Q, repo: R) -> Self {
        Self { query, repo }
    }
}

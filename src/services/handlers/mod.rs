pub mod device;
pub mod device_group;

pub struct RepositoryHandler<C, R> {
    pub(crate) command: C,
    pub(crate) repo: R,
}

impl<C, R> RepositoryHandler<C, R> {
    pub fn new(command: C, repo: R) -> Self {
        Self { command, repo }
    }
}

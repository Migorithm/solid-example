pub mod device;
pub mod device_group;

pub struct RepositoryHandler<C, R> {
    pub(crate) command: C,
    pub(crate) repo: R,
}

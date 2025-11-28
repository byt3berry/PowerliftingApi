use anyhow::Result;

pub trait Repository {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(self) -> Result<()>;
}

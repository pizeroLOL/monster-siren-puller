use super::{Task, TaskError, REPLACE};
use std::path::Path;
use tokio::io::AsyncWriteExt;

impl<'a> Task<'a> {
    pub async fn get_write_url(&self, url: &str, name: &str) -> Result<(), TaskError> {
        let file_name = url
            .split('.')
            .next_back()
            .ok_or(TaskError::NoneUrl(url.to_string()))?
            .replace(REPLACE, "");
        let file_name = format!("{name}.{file_name}");
        let path = self.path.join(file_name);
        let path = path.as_path();
        self.get_write(url, path).await?;
        Ok(())
    }

    pub async fn get_write(&self, url: &str, path: &Path) -> Result<(), TaskError> {
        let mut file = tokio::fs::File::create(path)
            .await
            .map_err(TaskError::CreateFile)?;
        file.write_all(&self.get_byte(url).await?)
            .await
            .map_err(TaskError::Write)?;
        file.sync_all().await.map_err(TaskError::Write)?;
        Ok(())
    }
}

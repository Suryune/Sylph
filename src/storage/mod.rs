use crate::data::Data;
use crate::error::Result;
use std::fs;
use std::path::Path;

mod bucket;

pub fn write_data(data: &Data, path: &Path) -> Result<()> {
    match data {
        Data::DocBucket(bucket) => {
            let full_path = path.join(bucket.id.to_string());
            ensure_parent_exists(&full_path)?;
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&bucket.content)?;
            let tmp_path = full_path.with_extension("tmp");
            fs::write(&tmp_path, &bytes)?;
            fs::rename(&tmp_path, &full_path)?;
        }
    }
    Ok(())
}

/// 确保父目录存在，不存在则创建
pub fn ensure_parent_exists(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

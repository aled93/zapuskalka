use std::{collections::HashMap, path::PathBuf};

pub struct AppDataPath(pub PathBuf);

pub struct AppsRunningStatus {
    pub app2child: HashMap<String, tokio::process::Child>,
}

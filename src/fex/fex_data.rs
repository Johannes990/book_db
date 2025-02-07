pub struct FileExplorerData {
    path_name: String,
    path_size: String,
    date_created: String,
    is_dir: bool,
}

impl FileExplorerData {
    pub fn new(path_name: String, path_size: String, date_created: String, is_dir: bool) -> Self {
        Self {
            path_name,
            path_size,
            date_created,
            is_dir
        }
    }
    
    pub const fn ref_array(&self) -> [&String; 3] {
        [&self.path_name, &self.path_size, &self.date_created]
    }

    pub fn path_name(&self) -> &str {
        &self.path_name
    }

    pub fn path_size(&self) -> &str {
        &self.path_size
    }

    pub fn date_created(&self) -> &str {
        &self.date_created
    }

    pub fn is_dir(&self) -> &bool {
        &self.is_dir
    }
}
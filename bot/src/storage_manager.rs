pub struct StorageManager {}

impl Default for StorageManager {
    fn default() -> Self {
        StorageManager {}
    }
}

impl StorageManager {
    pub async fn initialise(&self) {
        println!("Storage Manager online!");
    }
}

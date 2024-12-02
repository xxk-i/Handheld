pub mod commands {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    pub enum WalkthroughCommand {
        Search(String),
        Select(String),
    }
}

use dotenvy::dotenv;
#[derive(Debug, Clone)]
pub struct Config {
    preamble: String,
    anthropic_api_key: String,
}
impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        let preamble = std::env::var("PREAMBLE")?;
        let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY")?;
        Ok(Self {
            preamble,
            anthropic_api_key,
        })
    }
    pub fn preamble(&self) -> &str {
        &self.preamble
    }
    pub fn anthropic_api_key(&self) -> &str {
        &self.anthropic_api_key
    }
}

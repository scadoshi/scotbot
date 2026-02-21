use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Topic {
    #[default]
    General,
    News,
    Finance,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchDepth {
    Advanced,
    #[default]
    Basic,
    Fast,
    Utrafast,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    #[default]
    None,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Serialize, Deserialize)]
pub struct SearchArgs {
    query: String,
    auto_parameters: bool,
    topic: Topic,
    search_depth: SearchDepth,
    chunks_per_source: usize,
    max_results: usize,
    time_range: TimeRange,
    start_date: String,
    end_date: String,
    include_answer: bool,
    include_raw_content: bool,
    include_images: bool,
    include_image_descriptions: bool,
    include_favicon: bool,
    include_domains: Vec<String>,
    exclude_domains: Vec<String>,
    country: Option<String>,
    include_usage: bool,
}

pub struct Search;

impl Tool for Search {
    const NAME: &'static str = "add";
    type Args = SearchArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _promp: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "returns sum of two given numbers, lhs and rhs".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "lhs": {"type": "number", "description": "left hand side: the first number"},
                    "rhs": {"type": "number", "description": "right hand side: the second number"}
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.lhs + args.rhs)
    }
}

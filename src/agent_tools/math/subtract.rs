use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubtractArgs {
    pub lhs: i64,
    pub rhs: i64,
}

pub struct Subtract;

impl Tool for Subtract {
    const NAME: &'static str = "subtract";
    type Args = SubtractArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _promp: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "returns difference of two given numbers, lhs and rhs, rhs is subtracted from lhs"
                    .to_string(),
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
        Ok(args.lhs - args.rhs)
    }
}

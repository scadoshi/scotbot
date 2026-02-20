use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DivideArgs {
    pub lhs: i64,
    pub rhs: i64,
}

pub struct Divide;

impl Tool for Divide {
    const NAME: &'static str = "divide";
    type Args = DivideArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _promp: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "returns a quotient given two numbers, lhs and rhs, lhs is divided by rhs"
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "lhs": {"type": "number", "description": "left hand side: the first number, the dividend"},
                    "rhs": {"type": "number", "description": "right hand side: the second number, the divisor"}
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.lhs / args.rhs)
    }
}

use crate::agent_tools::math::{add::Add, divide::Divide, multiply::Multiply, subtract::Subtract};
use rig::tool::ToolDyn;

pub mod add;
pub mod divide;
pub mod multiply;
pub mod subtract;

pub fn math_tools() -> Vec<Box<dyn ToolDyn>> {
    vec![
        Box::new(Add),
        Box::new(Subtract),
        Box::new(Multiply),
        Box::new(Divide),
    ]
}

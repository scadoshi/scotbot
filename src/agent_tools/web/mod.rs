use rig::tool::ToolDyn;

pub mod crawl;
pub mod extract;
pub mod search;

pub fn web_tools() -> Vec<Box<dyn ToolDyn>> {
    vec![]
}

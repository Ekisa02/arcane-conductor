use arcane_conductor::agent::planner::SimplePlanner;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let planner = SimplePlanner::new();
        let plan = planner.plan("Research Rust async patterns. Summarize.").await;
        println!("Plan: {:#?}", plan);
    })
}

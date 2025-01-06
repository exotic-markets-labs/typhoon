mod setup;

pub fn main() {
    let pinocchio_metrics = setup::runner("pinocchio");
    let steel_metrics = setup::runner("steel");
    let typhoon_metrics = setup::runner("typhoon");

    panic!(
        "{} {} {}",
        serde_json::to_string(&pinocchio_metrics).unwrap(),
        serde_json::to_string(&steel_metrics).unwrap(),
        serde_json::to_string(&typhoon_metrics).unwrap()
    )
}

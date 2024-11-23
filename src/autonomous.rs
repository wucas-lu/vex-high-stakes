use anyhow::Result;
#[expect(unused_imports)]
use vexide::prelude::*;

use crate::robot::Robot;

pub async fn autonomous_control(_robot: &mut Robot) -> Result<()> {
    Ok(())
}

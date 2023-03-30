use crate::application::list_instances::Handler;
use anyhow::{anyhow, Result};
use comfy_table::Table;
use itertools::Itertools;

pub async fn list(handler: Handler<'_>) -> Result<()> {
    let running_instances = handler.list().await?;

    if running_instances.is_empty() {
        return Err(anyhow!("No instances were found"));
    }

    let mut table = Table::new();
    table.set_header(vec!["Environment", "Name", "Instance Id"]);

    let environment_count = running_instances.keys().count();
    for (i, env) in running_instances.keys().sorted().enumerate() {
        let mut instances = running_instances.get(env).unwrap().to_owned();
        instances.sort_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap());

        for instance in instances {
            table.add_row(vec![env, &instance.0, &instance.1]);
        }

        if i + 1 != environment_count {
            table.add_row(vec!["", "", ""]);
        }
    }

    println!("{table}");

    Ok(())
}

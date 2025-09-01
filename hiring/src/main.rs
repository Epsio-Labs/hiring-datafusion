use datafusion::prelude::*;
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::datasource::empty::EmptyTable;
use std::sync::Arc;
use datafusion_expr::{LogicalPlan};
use datafusion_common::{
    Result,
};

fn rewrite(plan: LogicalPlan) -> Result<LogicalPlan> {
    Ok(plan)
}


#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();

    let customers_schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
    ]));
    
    let vip_customers_schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
    ]));

    ctx.register_table("customers", Arc::new(EmptyTable::new(customers_schema)))?;
    ctx.register_table("vip_customers", Arc::new(EmptyTable::new(vip_customers_schema)))?;

    let df = ctx.sql("
        SELECT * FROM customers WHERE id IN (SELECT id FROM vip_customers)
    ").await?;

    let optimized_plan = ctx.state().optimize(df.logical_plan())?;

    let new_plan = rewrite(optimized_plan)?;
    
    println!("Optimized Logical Plan:");
    println!("{}", new_plan.display_indent());

    Ok(())
}
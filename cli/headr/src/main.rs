use headr::{get_args, run, MyResult};

fn main() -> MyResult<()> {
    run(get_args()?)?;
    Ok(())
}

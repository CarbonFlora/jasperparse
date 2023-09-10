use clap::Parser;

/// Small script to parse traffic data.
#[derive(Parser, Debug)]
#[command(author="Zi Hao L.", version="0.1.0", about="Small script to parse traffic data.", long_about = None)]

pub struct TrafficArgs {
    /// Select any number of workbooks
    #[arg(required = true)]
    pub input_sheet: String,

    /// Select any number of workbooks
    #[arg(default_value_t = String::from("./untitled_output.xlsx"))]
    pub output_dir: String,
}

use anyhow::Result;
use jasperparse::parse::excel_excel;

fn main() -> Result<()> {
    excel_excel()?;
    Ok(())
}

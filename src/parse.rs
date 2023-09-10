use std::{collections::VecDeque, path::PathBuf};

use anyhow::Result;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use clap::Parser;

use crate::args::TrafficArgs;

pub enum DirectionDecision {
    Left(u32),
    Thru(u32),
    Right(u32),
}

pub struct Bound {
    bound: [DirectionDecision; 3],
}

impl Bound {
    pub fn new() -> Self {
        Bound {
            bound: [
                DirectionDecision::Left(0),
                DirectionDecision::Thru(0),
                DirectionDecision::Right(0),
            ],
        }
    }

    pub fn peak_excel(col_index: u8, rows: &Vec<&[DataType]>) -> Bound {
        // let mut buffer = VecDeque::new();

        // for row in row_iter {
        //     let index_zero = row.get(0).unwrap_or(&DataType::Bool(false));
        //     if index_zero.get_string().is_some_and(|x| x.ends_with("m")) {}
        // }
        Bound::new()
    }
}

pub struct Intersection {
    north_bound: Bound,
    east_bound: Bound,
    south_bound: Bound,
    west_bound: Bound,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            north_bound: Bound::new(),
            east_bound: Bound::new(),
            south_bound: Bound::new(),
            west_bound: Bound::new(),
        }
    }
}

impl Intersection {
    pub fn from_excel(path: PathBuf) -> Result<Self> {
        let mut intersection = Intersection::new();
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        let worksheets = workbook.worksheets();
        let row_iter = worksheets
            .iter()
            .map(|worksheet| worksheet.1.rows())
            .flatten()
            .collect::<Vec<_>>();
        intersection.north_bound = Bound::peak_excel(1, &row_iter);

        Ok(intersection)
    }

    pub fn output_excel(&self) -> Result<()> {
        Ok(())
    }
}

pub fn excel_excel() -> Result<()> {
    let args = TrafficArgs::parse();
    let traffic_count = Intersection::from_excel(PathBuf::from(args.input_sheet))?;
    let _ = traffic_count.output_excel()?;

    Ok(())
}

use std::{fs, io};
use std::io::Read;
use std::ops::Div;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use plotters::prelude::*;

use network::network::statistics::FinalizedManagerStats;

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    file: Option<PathBuf>,

    #[clap(short, long, parse(from_os_str), value_name = "DIR")]
    dir: Option<PathBuf>,
}

fn read_to_string(path: &Option<PathBuf>) -> Result<String> {
    // Bit dirty but it's only for configs so doesn't really matter
    Ok(match path {
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        },
        Some(f) => fs::read_to_string(f)?
    })
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let json = read_to_string(&cli.file)?;
    let stats: FinalizedManagerStats = serde_json::from_str(&json)?;
    println!("{}", "Communication:".bold().green());
    println!("\tBytes sent:    {} B", stats.sent_bytes());
    println!("\tMessages sent: {}", stats.sent_count());
    println!();
    println!("{}", "Time:".bold().green());
    for handle_stats in stats.handle_stats() {
        if let Some(duration) = handle_stats.duration() {
            println!("\t{}: {} ms", handle_stats.get_label().as_ref().unwrap().bold(), duration);
            for (label, time) in handle_stats.get_events() {
                println!("\t\t{} after {} ms", label, *time - handle_stats.get_start().unwrap())
            }
        }
    }

    /*
    if let Some(Some(max_duration)) = stats.handle_stats().iter().map(|s| s.duration()).max() {
        let root = SVGBackend::new("stats.svg", (2000, 700)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            //.caption("APSS", ("sans-serif", 50).into_font())
            .margin(5u32)
            .build_cartesian_2d(0..(max_duration + max_duration.div(3)), 0..(stats.handle_stats().len() as u128 + stats.handle_stats().len().div(5) as u128))?;

        chart.configure_mesh().draw()?;

        for (i, handle_stats) in stats.handle_stats().iter().enumerate() {
            if let Some(end) = handle_stats.get_end() {
                let color = Palette99::pick(i);
                chart
                    .draw_series(LineSeries::new(
                        vec![(handle_stats.get_start().unwrap(), i as u128 +1), (end, i as u128 + 1)].into_iter(),
                        &color,
                    ))?
                    .label(handle_stats.get_label().as_ref().or_else(|| Some(&unlabled)).unwrap())
                    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
                chart.draw_series(
                    PointSeries::of_element(handle_stats.get_events().iter().map(|(_, x)| (*x, i as u128 + 1)), 3u32, Palette99::pick(i),
                                            &|coord, size, style| {
                                                EmptyElement::at(coord)
                                                    + Cross::new((0, 0), size, style)
                                            }
                    )
                )?;
            }
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
    }

     */

    Ok(())
}
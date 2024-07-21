use plotters::prelude::*;
use chrono::offset::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();

	let now = chrono::offset::Local::now().date();
	let mut chart = ChartBuilder::on(&root).build_cartesian_2d(now..now, 0..10)?;

	chart.configure_mesh().draw()?;
	Ok(())
}
use plotters::prelude::*;
use std::error::Error;

pub fn draw_rewards(caption: &str, rewards: &[f64], window: usize) -> Result<(), Box<dyn Error>> {
    if rewards.is_empty() {
        return Ok(());
    }

    let root = BitMapBackend::new("./rewards.png", (1200, 700)).into_drawing_area();

    root.fill(&WHITE)?;

    let min = rewards.iter().copied().fold(f64::INFINITY, f64::min);
    let max = rewards.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0usize..rewards.len(), (min - 10.0)..(max + 10.0))?;

    chart
        .configure_mesh()
        .x_desc("Episode")
        .y_desc("Reward")
        .draw()?;

    chart.draw_series(LineSeries::new(
        rewards.iter().enumerate().map(|(i, &r)| (i, r)),
        &BLUE.mix(0.2),
    ))?;

    let moving_average = rewards.windows(window).enumerate().map(|(i, values)| {
        let avg = values.iter().sum::<f64>() / window as f64;
        (i + window - 1, avg)
    });

    chart.draw_series(LineSeries::new(moving_average, &RED))?;

    root.present()?;

    Ok(())
}

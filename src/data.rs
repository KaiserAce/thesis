use plotters::prelude::*;
use plotters::style::RGBColor;
use rand::{rng, Rng};

pub fn test_figure() -> Result<(), Box<dyn std::error::Error>> {
    // Create output file
    let root = BitMapBackend::new("hawk_dove_stacked_grid.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Grid dimensions: 10 rows (strategies) × 20 cols (ranks)
    let (rows, cols) = (11, 21);
    let mut chart = ChartBuilder::on(&root)
        .caption("Hawk-Dove Stacked Circles Grid", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..cols, 0..rows)?;

    // Hide default grid lines (we'll draw our own)
    chart.configure_mesh().disable_mesh().draw()?;

    let threshold = 0.5;

    // Draw stacked circles in each grid cell
    for row in 1..rows {
        for col in 1..cols {
            let center = (col, row);

            let value = (col as f32) / (cols as f32);

            let mut rng = rng();

            let color = {
                // More red (transition to blue)
                RGBColor(
                    (255.0 * (1.0 - value / threshold)) as u8,
                    0,
                    (255.0 * (value / threshold)) as u8,
                )
            };

            let color_1 = {
                RGBColor(
                    (255.0 * ((1.0 - rng.random::<f32>() as f32) as f32  / threshold)) as u8,
                    0,
                    (255.0 * (rng.random::<f32>() as f32/ threshold)) as u8,
                )
            };

            // Larger circle (Hawk, blue)
            chart.draw_series(std::iter::once(
                Circle::new(center, 20, color.filled().stroke_width(2)),
            ))?;

            // Smaller circle (Dove, red) stacked on top
            chart.draw_series(std::iter::once(
                Circle::new(center, 10, color_1.filled().stroke_width(2)),
            ))?;
        }
    }

    // Label axes
    let font = ("sans-serif", 15);
    root.draw(&Text::new("Ranks (1-20)", (500, 750), font))?;

    Ok(())
}

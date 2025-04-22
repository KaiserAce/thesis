use plotters::prelude::*;
use plotters::style::RGBColor;

use std::fs::File;

pub fn figure_2b_plot(
    host_mat: Vec<Vec<(u32, f64)>>,
    visit_mat: Vec<Vec<(u32, f64)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create output file
    let root = BitMapBackend::new("figure_2b.png", (1200, 800)).into_drawing_area();
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

    let threshold = 0.8;

    for row in 0..host_mat.len() {
        for col in host_mat[row].clone() {
            let center = (col.0 as i32, row as i32 + 1);

            let color = if col.1 >= threshold {
                // More red (transition to blue)
                RGBColor(
                    255,
                    0,
                    0,
                )

            } else {
                RGBColor(
                    0,
                    0,
                    255,
                )
                };

            chart.draw_series(std::iter::once(Circle::new(
                center,
                20,
                color.filled().stroke_width(2),
            )))?;
        }
    }

    for row in 0..visit_mat.len() {
        for col in visit_mat[row].clone() {
            let center = (col.0 as i32, row as i32 + 1);
            let color = if col.1 >= threshold {
                // More red (transition to blue)
                RGBColor(
                    255,
                    0,
                    0,
                )

            } else {
                RGBColor(
                    0,
                    0,
                    255,
                )
                };

            chart.draw_series(std::iter::once(Circle::new(
                center,
                10,
                color.filled().stroke_width(2),
            )))?;
        }
    }

    // Label axes
    let font = ("sans-serif", 15);
    root.draw(&Text::new("Ranks (1-20)", (500, 750), font))?;

    Ok(())
}

pub fn figure_2b() -> Result<(), Box<dyn std::error::Error>> {
    let ff = vec![
        "0.1", "0.2", "0.3", "0.4", "0.5", "0.6", "0.7", "0.8", "0.9", "1.0",
    ];

    let mut host_mat: Vec<Vec<(u32, f64)>> = Vec::new();
    let mut visit_mat: Vec<Vec<(u32, f64)>> = Vec::new();

    for file in ff {
        let out_score = File::open(format!(
            "Output/figure_2b/ff_{}/OutScore_0.csv",
            file
        ))?;
        let strategy_host = File::open(format!(
            "Output/figure_2b/ff_{}/StrategyHost_0.csv",
            file
        ))?;
        let strategy_visit = File::open(format!(
            "Output/figure_2b/ff_{}/StrategyVisit_0.csv",
            file
        ))?;

        // Create a CSV reader
        let mut out_score_data = csv::Reader::from_reader(out_score);
        let mut strategy_host_data = csv::Reader::from_reader(strategy_host);
        let mut strategy_visit_data = csv::Reader::from_reader(strategy_visit);

        let mut out_score_vec: Vec<u32> = Vec::new();
        let mut host_vec: Vec<f64> = Vec::new();
        let mut visit_vec: Vec<f64> = Vec::new();

        if let Some(last_record) = out_score_data.records().last() {
            let last_record = last_record?;
            for field in last_record.iter() {
                out_score_vec.push(field.parse()?);
            }
        }

        // Similarly for other files
        if let Some(last_record) = strategy_host_data.records().last() {
            let last_record = last_record?;
            for field in last_record.iter() {
                host_vec.push(field.parse()?);
            }
        }

        if let Some(last_record) = strategy_visit_data.records().last() {
            let last_record = last_record?;
            for field in last_record.iter() {
                visit_vec.push(field.parse()?);
            }
        }

        let mut host_tup: Vec<(u32, f64)> = Vec::new();
        let mut visit_tup: Vec<(u32, f64)> = Vec::new();

        for i in 0..out_score_vec.len() {
            host_tup.push((out_score_vec[i], host_vec[i * 2]));
            visit_tup.push((out_score_vec[i], visit_vec[i * 2]));
        }

        host_mat.push(host_tup);
        visit_mat.push(visit_tup);
    }

    figure_2b_plot(host_mat, visit_mat)?;

    Ok(())
}

pub fn figure_3a_plot(
    host_mat: Vec<Vec<(u32, f64)>>,
    visit_mat: Vec<Vec<(u32, f64)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create output file
    let root = BitMapBackend::new("figure_3a.png", (1200, 800)).into_drawing_area();
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

    let threshold = 0.8;

    for row in 0..host_mat.len() {
        for col in host_mat[row].clone() {
            let center = (col.0 as i32, row as i32 + 1);

            let color = if col.1 >= threshold {
                // More red (transition to blue)
                RGBColor(
                    255,
                    0,
                    0,
                )

            } else {
                RGBColor(
                    0,
                    0,
                    255,
                )
                };

            chart.draw_series(std::iter::once(Circle::new(
                center,
                20,
                color.filled().stroke_width(2),
            )))?;
        }
    }

    for row in 0..visit_mat.len() {
        for col in visit_mat[row].clone() {
            let center = (col.0 as i32, row as i32 + 1);
            let color = if col.1 >= threshold {
                // More red (transition to blue)
                RGBColor(
                    255,
                    0,
                    0,
                )

            } else {
                RGBColor(
                    0,
                    0,
                    255,
                )
                };

            chart.draw_series(std::iter::once(Circle::new(
                center,
                10,
                color.filled().stroke_width(2),
            )))?;
        }
    }

    // Label axes
    let font = ("sans-serif", 15);
    root.draw(&Text::new("Ranks (1-20)", (500, 750), font))?;

    Ok(())
}

pub fn figure_3a() -> Result<(), Box<dyn std::error::Error>> {
    let ff = vec![
        "0.1", "0.2", "0.3", "0.4", "0.5", "0.6", "0.7", "0.8", "0.9", "1.0",
    ];

    let mut host_mat: Vec<Vec<(u32, f64)>> = Vec::new();
    let mut visit_mat: Vec<Vec<(u32, f64)>> = Vec::new();

    for file in ff {
        let out_score = File::open(format!(
            "Output/figure_3a/ff_{}/OutScore_0.csv",
            file
        ))?;
        let strategy_host = File::open(format!(
            "Output/figure_3a/ff_{}/StrategyHost_0.csv",
            file
        ))?;
        let strategy_visit = File::open(format!(
            "Output/figure_3a/ff_{}/StrategyVisit_0.csv",
            file
        ))?;

        // Create a CSV reader
        let mut out_score_data = csv::Reader::from_reader(out_score);
        let mut strategy_host_data = csv::Reader::from_reader(strategy_host);
        let mut strategy_visit_data = csv::Reader::from_reader(strategy_visit);

        let mut out_score_vec: Vec<u32> = Vec::new();
        let mut host_vec: Vec<f64> = Vec::new();
        let mut visit_vec: Vec<f64> = Vec::new();

        if let Some(last_record) = out_score_data.records().last() {
            let last_record = last_record?;
            for field in last_record.iter() {
                out_score_vec.push(field.parse()?);
            }
        }

        // Similarly for other files
        if let Some(last_record) = strategy_host_data.records().last() {
            let last_record = last_record?;
            for field in last_record.iter() {
                host_vec.push(field.parse()?);
            }
        }

        if let Some(last_record) = strategy_visit_data.records().last() {
            let last_record = last_record?;
            for field in last_record.iter() {
                visit_vec.push(field.parse()?);
            }
        }

        let mut host_tup: Vec<(u32, f64)> = Vec::new();
        let mut visit_tup: Vec<(u32, f64)> = Vec::new();

        for i in 0..out_score_vec.len() {
            host_tup.push((out_score_vec[i], host_vec[i * 2]));
            visit_tup.push((out_score_vec[i], visit_vec[i * 2]));
        }

        host_mat.push(host_tup);
        visit_mat.push(visit_tup);
    }

    figure_3a_plot(host_mat, visit_mat)?;

    Ok(())
}

pub fn figure_3b_plot(data: Vec<Vec<(f32, f32)>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("figure_3b.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Determine axis ranges
    let x_min = data.iter().flatten().map(|(x, _)| *x).fold(f32::INFINITY, f32::min);
    let x_max = data.iter().flatten().map(|(x, _)| *x).fold(f32::NEG_INFINITY, f32::max);
    let y_min = data.iter().flatten().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
    let y_max = data.iter().flatten().map(|(_, y)| *y).fold(f32::NEG_INFINITY, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Hawk-Dove Strategy Analysis", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh()
        .x_desc("X Axis Label")
        .y_desc("Y Axis Label")
        .draw()?;

    // Define colors as static array
    static COLORS: [RGBColor; 5] = [
        RGBColor(255, 0, 0),    // Red
        RGBColor(0, 0, 255),    // Blue
        RGBColor(0, 255, 0),    // Green
        RGBColor(255, 0, 255),  // Magenta
        RGBColor(0, 255, 255),  // Cyan
    ];

    let populations = [20, 50, 100, 200, 500];

    for (i, data_line) in data.iter().enumerate() {
        let color = COLORS[i % COLORS.len()];
        let style = ShapeStyle {
            color: color.to_rgba(),
            filled: false,
            stroke_width: 2,
        };
        
        chart.draw_series(LineSeries::new(
            data_line.iter().map(|&(x, y)| (x, y)),
            style,
        ))?
        .label(format!("Population {}", populations[i]))
        .legend(move |(x, y)| {
            PathElement::new(vec![(x, y), (x + 20, y)], style)
        });
    }

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn figure_3b() -> Result<(), Box <dyn std::error::Error>>{
    let pop = vec!["20", "50", "100", "200", "500"];
    let ff = vec![
        "0.1", "0.2", "0.3", "0.4", "0.5", "0.6", "0.7", "0.8", "0.9", "1.0",
    ];

    let mut data: Vec<Vec<(f32, f32)>> = Vec::new();

    for num in pop.iter() {
        let mut data_pop: Vec<(f32, f32)> = Vec::new();

        for file in ff.iter() {
            let hstrat = File::open(format!("Output/figure_3b/{}/ff_{}/StrategyHost_0.csv", num, file))?;
            let vstrat = File::open(format!("Output/figure_3b/{}/ff_{}/StrategyVisit_0.csv", num, file))?;

            let mut hstrat_data = csv::Reader::from_reader(hstrat);
            let mut vstrat_data = csv::Reader::from_reader(vstrat);

            let mut hstrat_vec: Vec<f32> = Vec::new();
            let mut vstrat_vec: Vec<f32> = Vec::new();

            if let Some(last_record) = hstrat_data.records().last() {
                let last_record = last_record?;
                for field in last_record.iter() {
                    hstrat_vec.push(field.parse()?);
                }
            }

            if let Some(last_record) = vstrat_data.records().last() {
                let last_record = last_record?;
                for field in last_record.iter() {
                    vstrat_vec.push(field.parse()?);
                }
            }

            let mut count = 0;

            for i in 0..hstrat_vec.len()/2 {
                if hstrat_vec[i*2] >= 0.9 && vstrat_vec[i*2] >= 0.9 {
                    count += 1;
                }
            }


            data_pop.push((100.0 * (count as f32/hstrat_vec.len() as f32) as f32, file.parse()?));
        }
        data.push(data_pop);
    }

    let _ = figure_3b_plot(data);

    Ok(())
}

pub fn test_figure() -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing backend (bitmap in this case)
    let root = BitMapBackend::new("multiple_lines.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define chart properties
    let mut chart = ChartBuilder::on(&root)
        .caption("Multiple Line Graphs", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10f32, 0f32..100f32)?;

    // Configure and draw grid
    chart.configure_mesh().draw()?;

    // Generate data for three different lines
    let line1_data: Vec<(f32, f32)> = (0..=10).map(|x| (x as f32, x as f32 * 5.0)).collect();
    let line2_data: Vec<(f32, f32)> = (0..=10).map(|x| (x as f32, x as f32 * x as f32)).collect();
    let line3_data: Vec<(f32, f32)> = (0..=10).map(|x| (x as f32, 100.0 - x as f32 * 8.0)).collect();

    // Draw the lines with different colors and labels
    chart
        .draw_series(LineSeries::new(line1_data, &RED))?
        .label("Linear Growth")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(line2_data, &BLUE))?
        .label("Quadratic Growth")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(line3_data, &GREEN))?
        .label("Linear Decline")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // Configure the legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}


















use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn return_25th_quantile(data_: &DataFrame, target: &str) -> Result<f64, Box<dyn Error>> {
    let target_quantile = data_[target].quantile(0.25)?;
    Ok(target_quantile)
}

fn return_mean(data_: &DataFrame, target: &str) -> Result<f64, Box<dyn Error>> {
    let target_mean = data_[target].mean()?;
    Ok(target_mean)
}

fn return_std_dev(data_: &DataFrame, target: &str) -> Result<f64, Box<dyn Error>> {
    let target_std = data_[target].std()?;
    Ok(target_std)
}

fn return_median(data_: &DataFrame, target: &str) -> Result<f64, Box<dyn Error>> {
    let target_median = data_[target].median()?;
    Ok(target_median)
}

fn visualize_dataset(
    data_: &DataFrame,
    outcome_var: &str,
    target_var: &str,
    interaction_term: &str,
) -> Result<(), Box<dyn Error>> {
    // Get the unique categories from the interaction_term column
    let categories = data_[interaction_term].unique()?;
    // Define a colormap based on the number of unique categories
    let cmap = plotters::style::colors::ColorMap::viridis();

    // Add scatter plot of outcome vs predictor
    let mut scatter_ctx = plotters::prelude::BitMapBackend::new("output/visualization.png", (640, 480)).into_drawing_area();
    scatter_ctx.fill(&plotters::style::colors::WHITE)?;
    scatter_ctx
        .draw(&plotters::prelude::ChartBuilder::on(&scatter_ctx)
            .caption(format!("Descriptive Statistics {} VS {}", target_var, outcome_var), ("sans-serif", 20))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0f64..data_[target_var].max()?, 0f64..data_[outcome_var].max()?)?
            .configure_mesh()
            .x_desc(target_var)
            .y_desc(outcome_var)
            .draw()?)?;

    for (i, cat) in categories.into_iter().enumerate() {
        let data_c = data_.filter(data_[interaction_term].eq(cat));

        let slope_intercept = data_c[target_var].polyfit(data_c[outcome_var], 1)?;
        let slope = slope_intercept[0];
        let intercept = slope_intercept[1];
        let best_fit_line = data_c[target_var].apply(|x| slope * x + intercept);

        scatter_ctx.draw_series(
            data_c.select((target_var, outcome_var))
                .map(|(x, y)| plotters::prelude::Circle::new((*x, *y), 3, plotters::style::colors::BLUE))
                .translate((0, i as i32 * 20))?
        )?;

        scatter_ctx.draw_series(
            plotters::prelude::line_series::LineSeries::new(
                data_c.select((target_var,))
                    .apply(|x| slope * x + intercept)
                    .into_iter()
                    .zip(data_c.select((outcome_var,)).into_iter())
                    .map(|(x, y)| (*x, *y))
                    .collect::<Vec<_>>()
            )
            .color(cmap.mix(i as f64 / categories.len() as f64))
            .width(2)
            .label(format!("Best Fit For Interaction Category: {}", cat))
        )?;
    }

    // Plot mean, median, std dev, and 25th quantile
    let mean = return_mean(data_, target_var)?;
    scatter_ctx.draw(&plotters::prelude::LineSeries::new(
        vec![(mean, 0f64), (mean, data_[outcome_var].max()?)],
        plotters::style::colors::RED.mix(0.5),
    ))?;

    let median = return_median(data_, target_var)?;
    scatter_ctx.draw(&plotters::prelude::LineSeries::new(
        vec![(median, 0f64), (median, data_[outcome_var].max()?)],
        plotters::style::colors::GREEN.mix(0.5),
    ))?;

    let stand_dev = return_std_dev(data_, target_var)?;
    scatter_ctx.draw(&plotters::prelude::LineSeries::new(
        vec![(mean - stand_dev, 0f64), (mean - stand_dev, data_[outcome_var].max()?)],
        plotters::style::colors::ORANGE.mix(0.5),
    ))?;
    scatter_ctx.draw(&plotters::prelude::LineSeries::new(
        vec![(mean + stand_dev, 0f64), (mean + stand_dev, data_[outcome_var].max()?)],
        plotters::style::colors::ORANGE.mix(0.5),
    ))?;

    scatter_ctx.draw(&plotters::prelude::Text::new(
        format!("Mean: {:.2}", mean),
        (mean, data_[outcome_var].max()?)
    ).color(plotters::style::colors::RED))?;

    scatter_ctx.draw(&plotters::prelude::Text::new(
        format!("Median: {:.2}", median),
        (median, data_[outcome_var].max()?)
    ).color(plotters::style::colors::GREEN))?;

    scatter_ctx.draw(&plotters::prelude::Text::new(
        format!("Mean + StDev: {:.2}", mean + stand_dev),
        (mean + stand_dev, data_[outcome_var].max()?)
    ).color(plotters::style::colors::ORANGE))?;

    scatter_ctx.draw(&plotters::prelude::Text::new(
        format!("Mean - StDev: {:.2}", mean - stand_dev),
        (mean - stand_dev, data_[outcome_var].max()?)
    ).color(plotters::style::colors::ORANGE))?;

    scatter_ctx.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = CsvReader::from_path("data/iris_data.csv")?.finish()?;
    let target_column = "sepal_width";

    println!("{}", return_25th_quantile(&data, target_column)?);
    println!("{}", return_mean(&data, target_column)?);
    println!("{}", return_median(&data, target_column)?);
    println!("{}", return_std_dev(&data, target_column)?);

    visualize_dataset(&data, "petal_width", target_column, "species")?;

    Ok(())
}
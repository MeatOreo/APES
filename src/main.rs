slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = ExploreWindow::new()?;

    ui.run()
}
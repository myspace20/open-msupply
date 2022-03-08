use std::{fs, path::PathBuf, str::FromStr};

use headless_chrome::{protocol::page::PrintToPdfOptions, Browser, LaunchOptionsBuilder};

pub fn html_to_pdf(document: &String) -> Result<Vec<u8>, failure::Error> {
    let pdf_options=  Some(PrintToPdfOptions {
    display_header_footer: Some(true),
    prefer_css_page_size: Some(false),
    landscape: Some(true),
    print_background: None,
    scale: None,
    // Assuming 96 DPI (dots per inch)
    paper_width: Some(8.0),
    paper_height: Some(11.0),
    margin_top: Some(1.0),
    margin_bottom: Some(1.0),
    margin_left: Some(0.0),
    margin_right: Some(0.0),
    page_ranges: None,
    ignore_invalid_page_ranges: None,
    header_template: Some(r#"<div style="font-size: 15px; padding-top: 8px; text-align: center; width: 100%;"><span>Some Rad Store's Header here. </div>"#.to_string()),
    footer_template: Some(r#"<div style="font-size: 15px; padding-top: 8px; text-align: center; width: 100%;"><span>Footer here.</span> Page number: <span class="pageNumber"></span></div>"#.to_string()),
});

    // create a new browser and a tab in that browser using headless-chrome
    let launch_options = LaunchOptionsBuilder::default()
        .headless(true)
        .build()
        .map_err(|err| failure::err_msg(err))?;
    let browser = Browser::new(launch_options)?;
    let tab = browser.wait_for_initial_tab()?;

    let file_path = PathBuf::from_str("./document.html")?;
    fs::write(&file_path, document)?;
    let local_pdf = tab
        .navigate_to(&file_path.to_string_lossy())?
        .wait_until_navigated()?
        .print_to_pdf(pdf_options)?;
    Ok(local_pdf)
}

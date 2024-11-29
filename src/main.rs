use std::{fs::read_to_string, io::Error};

use dialoguer::{theme::ColorfulTheme, Input};

fn main() -> anyhow::Result<()> {
    println!("Duksegrupper - Stjernholm Edition");

    let codes = read_codes("codes.txt")?;
    println!("Total mængde duksegrupper der laves baseret på codes.txt: {}", codes.len());

    let swgroups: u32 = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Mængde af Software grupper")
    .interact_text()?;

    let datgroups: u32 = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Mængde af Datalogi grupper")
    .interact_text()?;

    let mailswprefix: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Software email prefix")
    .with_initial_text("cs-24-sw-3-")
    .interact_text()?;

    let maildatprefix: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Datalogi email prefix")
    .with_initial_text("cs-24-dat-3-")
    .interact_text()?;
    
    let mailsuffix: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Email suffix")
    .with_initial_text("@student.aau.dk")
    .interact_text()?;

    Ok(())
}

fn read_codes(filename: &str) -> Result<Vec<String>, Error> {
    let codefileread = read_to_string(filename).expect(format!("Could not find {} file", filename).as_str());
    Ok(codefileread.lines().map(String::from).collect())
}

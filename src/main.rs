use std::{fs::{self, read_to_string}, io::Error};

use chrono::{Datelike, Duration, Local, NaiveDate};
use dialoguer::{theme::ColorfulTheme, Input};
use rand::{seq::SliceRandom, thread_rng};

fn main() -> anyhow::Result<()> {
    println!("Duksegrupper - Stjernholm Edition");

    let codes = read_codes("codes.txt")?;
    println!("Total mængde duksegrupper der laves baseret på valid codes i codes.txt: {}", codes.len());

    let swgroups: u32 = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Mængde af Software grupper")
    .interact_text()?;

    let datgroups: u32 = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Mængde af Datalogi grupper")
    .interact_text()?;

    let mailsw: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Software email (%d|%D placeholder)")
    .with_initial_text("cs-24-sw-3-%d@student.aau.dk")
    .interact_text()?;

    let maildat: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Datalogi email (%d|%D placeholder)")
    .with_initial_text("cs-24-dat-3-%d@student.aau.dk")
    .interact_text()?;

    let mut startdate: NaiveDate = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Start date")
    .with_initial_text(Local::now().date_naive().to_string())
    .interact_text()?;

    let mut groups: Vec<Group> = Vec::new();

    // Generate SW Groups:
    for i in 1..=swgroups {
        groups.push(Group{
            email: mailsw.replace("%d", &format!("{}", i)).replace("%D", &format!("{:02}", i)),
            prioritised: false
        });
    }
    // Generate DAT Groups:
    for i in 1..=datgroups {
        groups.push(Group{
            email: maildat.replace("%d", &format!("{}", i)).replace("%D", &format!("{:02}", i)),
            prioritised: false
        });
    }

    shuffle_groups(&mut groups);

    let mut output: Vec<String> = Vec::new();

    let mut current_group_index: usize = 0;
    while codes.len() > output.len() {
        let amount_of_groups_left = groups.len() - (current_group_index + 1);
        match amount_of_groups_left {
            0 => {
                shuffle_groups(&mut groups);
                current_group_index = 0;
            },
            1 => {
                groups.last_mut().unwrap().prioritised = true;
                current_group_index = 0;
            },
            _ => {
                let (current_group, next_group) = index_twice(&mut groups, current_group_index, current_group_index+1).unwrap();
                if current_group.prioritised { current_group.prioritised = false; }
                
                output.push(format!("{}|{};{};treo@fklub.dk|treo@fklub.dk|Duksegruppe|GROUP={} og {};WEEK={};CODE={}", startdate, current_group.email, next_group.email, current_group.email.split_once("@").unwrap().0, next_group.email.split_once("@").unwrap().0, startdate.iso_week().week(), codes.get(output.len()).unwrap()));

                current_group_index += 2;
                startdate = startdate.checked_add_signed(Duration::days(7)).unwrap();
            }
        }
    }

    output.iter().for_each(|output| {
        println!("{}", output)
    });
    
    let filename: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Output file")
    .with_initial_text("dukse.txt")
    .interact_text()?;

    fs::write(filename, output.join("\n")).expect("");

    Ok(())
}

fn shuffle_groups(groups: &mut Vec<Group>) {
    let prioritised = groups.iter().find(|g| g.prioritised).cloned();
    // Keep only non-prioritised groups
    groups.retain(|g| !g.prioritised);
    
    // Shuffle the remaining groups
    let mut rng = thread_rng();
    groups.shuffle(&mut rng);
    
    // Add the prioritised group back to the top (if it exists)
    if let Some(prioritised_group) = prioritised {
        groups.insert(0, prioritised_group);
    }
}

fn index_twice(slc: &mut Vec<Group>, a: usize, b: usize) -> Option<(&mut Group, &mut Group)> {
    if a == b {
        None
    } else {
        if a >= slc.len() || b >= slc.len() {
            None
        } else {
            // safe because a, b are in bounds and distinct
            unsafe {
                let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
                let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
                Some((ar, br))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Group {
    email: String,
    prioritised: bool
}

fn read_codes(filename: &str) -> Result<Vec<u16>, Error> {
    let codefileread = read_to_string(filename).expect(format!("Could not find {} file", filename).as_str());
    Ok(codefileread.lines().filter_map(|l| l.trim().parse::<u16>().ok()).collect())
}

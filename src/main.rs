use std::{fs::{write, read_to_string}, io::Error};

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
    .with_initial_text("cs-24-sw-3-%D@student.aau.dk")
    .interact_text()?;

    let maildat: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Datalogi email (%d|%D placeholder)")
    .with_initial_text("cs-24-dat-3-%D@student.aau.dk")
    .interact_text()?;

    let startdate: NaiveDate = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Start date")
    .with_initial_text(Local::now().date_naive().to_string())
    .interact_text()?;

    let mut groups: Vec<Group> = Vec::new();

    // Generate SW Groups:
    for i in 1..=swgroups {
        groups.push(Group{
            email: mailsw.replace("%d", &format!("{}", i)).replace("%D", &format!("{:02}", i)),
            used: 0
        });
    }
    // Generate DAT Groups:
    for i in 1..=datgroups {
        groups.push(Group{
            email: maildat.replace("%d", &format!("{}", i)).replace("%D", &format!("{:02}", i)),
            used: 0
        });
    }

    shuffle_groups(&mut groups);

    let output = generate_dukse(codes, &mut groups, startdate)?;

    output.iter().for_each(|output| {
        println!("{}", output)
    });
    
    let filename: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Output file")
    .with_initial_text("dukse.txt")
    .interact_text()?;

    write(filename, output.join("\n")).expect("");

    Ok(())
}

fn generate_dukse(codes: Vec<u16>, groups: &mut Vec<Group>, mut startdate: NaiveDate) -> anyhow::Result<Vec<String>> {
    let mut output: Vec<String> = Vec::new();
    let mut prioritised_group: Option<String> = None; 
    let mut current_group_index: usize = 0;
    while codes.len() > output.len() {
        let amount_of_groups_left = groups.len() - current_group_index;
        match amount_of_groups_left {
            0 => {
                shuffle_groups(groups);
                current_group_index = 0;
            },
            1 => {
                prioritised_group = Some(groups.last_mut().unwrap().email.clone());
                current_group_index = 0;
                shuffle_groups(groups);
            },
            _ => {
                if let Some((current_group, next_group)) = get_group_pair(groups, &mut current_group_index, &prioritised_group) {
                    if prioritised_group.is_some() { prioritised_group = None; }
                
                    output.push(format!("{}|{};{};treo@fklub.dk|treo@fklub.dk|Duksegruppe|GROUP={} og {};WEEK={};CODE={}", startdate, current_group.email, next_group.email, current_group.email.split_once("@").unwrap().0, next_group.email.split_once("@").unwrap().0, startdate.iso_week().week(), codes.get(output.len()).unwrap()));
                    current_group.used += 1;
                    next_group.used += 1;
                    startdate = startdate.checked_add_signed(Duration::days(7)).unwrap();
                } else {
                    shuffle_groups(groups);
                }
            }
        }
    }
    Ok(output)
} 

fn shuffle_groups(groups: &mut Vec<Group>) {    
    let mut rng = thread_rng();
    groups.shuffle(&mut rng);
}

fn get_group_pair<'a>(groups: &'a mut Vec<Group>, current_index: &mut usize, prioritized_group_email: &Option<String>) -> Option<(&'a mut Group, &'a mut Group)> {
    if let Some(prioritized_group_email) = prioritized_group_email {
        let prioritized_group_index = groups.iter_mut().enumerate().find(|(_, g)| g.email.eq(prioritized_group_email))?.0;
        let next_group = unsafe { &mut *(groups.get_unchecked_mut(prioritized_group_index) as *mut _) };
        let current_group = groups.get_mut(*current_index).unwrap();
        if current_group.eq(&next_group) { return None }
        *current_index += 1;
        Some((current_group, next_group))
    } else {
        if *current_index >= groups.len() || *current_index+1 >= groups.len() {
            None
        } else {
            unsafe {
                let current_group = &mut *(groups.get_unchecked_mut(*current_index) as *mut _);
                let next_group = &mut *(groups.get_unchecked_mut(*current_index+1) as *mut _);
                *current_index += 2;
                Some((current_group, next_group))
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Group {
    email: String,
    used: u128
}

fn read_codes(filename: &str) -> Result<Vec<u16>, Error> {
    let codefileread = read_to_string(filename).expect(format!("Could not find {} file", filename).as_str());
    Ok(codefileread.lines().filter_map(|l| l.trim().parse::<u16>().ok()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_groups_should_be_overused() {
        for _ in 0..50 {
            let codes: Vec<u16> = (1000..9999).collect();

            println!("Generated {} codes", codes.len());
    
            let startdate = Local::now().date_naive();
    
            let mut groups: Vec<Group> = Vec::new();
            // Generates 6 users which makes sures there is atleast a difference of 1 based on 8999 codes = 8999 group pairs to generate aswell.
            for i in 1..=6 {
                groups.push(Group{
                    email: format!("whatever{}@whatever.dk", i),
                    used: 0
                });
            }
    
            let _ = generate_dukse(codes, &mut groups, startdate);
    
            groups.iter_mut().for_each(|g| {
                println!("Group {}: {}", g.email , g.used)
            });
    
            let used: Vec<u128> = groups.iter().map(|g| g.used).collect();
    
            let difference = used.iter().max().unwrap() - used.iter().min().unwrap();
            println!("Difference: {}", difference);
            
            assert!(difference < 2);
        }

        
    }
}
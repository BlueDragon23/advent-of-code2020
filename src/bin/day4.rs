use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;

#[derive(Clone, Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

fn main() {
    let f = File::open("input/input4_1.txt").unwrap();
    let reader = BufReader::new(f);
    let result = reader.lines()
        .map(|line| line.unwrap())
        .fold((vec![], true), |(mut xs, is_new), line| {
            if line.is_empty() {
                (xs, true)
            } else {
                // Process the line
                if is_new {
                    // Create a new object
                    xs.push(add_fields(line, Passport::default()));
                    (xs, false)
                } else {
                    // Add properties to the last thing in the list
                    let result = xs.pop().unwrap();
                    xs.push(add_fields(line, result));
                    (xs, false)
                }
            }
        }).0.into_iter().filter(|passport| valid_fields(passport)).count();
        println!("{}", result);
}

fn add_fields(line: String, old_result: Passport) -> Passport {
    let mut result = old_result.clone();
    line.split(" ").for_each(|segment| {
        let mut parts = segment.split(":");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap().to_string();
        match key {
            "byr" => result.byr = Some(value),
            "iyr" => result.iyr = Some(value),
            "eyr" => result.eyr = Some(value),
            "hgt" => result.hgt = Some(value),
            "hcl" => result.hcl = Some(value),
            "ecl" => result.ecl = Some(value),
            "pid" => result.pid = Some(value),
            "cid" => result.cid = Some(value),
            _ => panic!("Invalid key")
        };
    });
    result
}

fn valid_fields(passport: &Passport) -> bool {
    let hair_regex = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
    let eye_colours = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let pid_regex = Regex::new(r"^([0-9]){9}$").unwrap();
    passport.byr.is_some() && 
        passport.byr.clone().map_or(false, |byr_str| 
            byr_str.len() == 4 && byr_str.parse::<usize>().map(|byr| byr >= 1920 && byr <= 2002).unwrap_or(false)) &&
    passport.iyr.is_some() &&
    passport.iyr.clone().map_or(false, |iyr_str| 
        iyr_str.len() == 4 && iyr_str.parse::<usize>().map(|iyr| iyr >= 2010 && iyr <= 2020).unwrap_or(false)) &&
    passport.eyr.is_some() &&
    passport.eyr.clone().map_or(false, |eyr_str| 
        eyr_str.len() == 4 && eyr_str.parse::<usize>().map(|eyr| eyr >= 2020 && eyr <= 2030).unwrap_or(false)) &&
    passport.hgt.is_some() &&
    passport.hgt.clone().map_or(false, |hgt_str| 
        match hgt_str {
            s if s.ends_with("cm") => {
                let height = s.split("c").next().unwrap().parse::<usize>().unwrap();
                height >= 150 && height <= 193
            },
            s if s.ends_with("in") => {
                let height = s.split("i").next().unwrap().parse::<usize>().unwrap();
                height >= 59 && height <= 76
            },
            _ => false
        }
     ) &&
    passport.hcl.is_some() && passport.hcl.clone().map_or(false, |hair| hair_regex.is_match(&hair)) &&
    passport.ecl.is_some() && passport.ecl.clone().map_or(false, |eye_colour| eye_colours.contains(&eye_colour.as_str())) &&
    passport.pid.is_some() && passport.pid.clone().map_or(false, |p| pid_regex.is_match(&p))
}
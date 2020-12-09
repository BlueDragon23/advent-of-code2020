use reformation::Reformation;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[derive(Reformation)]
#[reformation(r"{year}")]
struct BirthYear {
    year: usize,
}

#[derive(Reformation)]
#[reformation(r"{year}")]
struct IssueYear {
    year: usize,
}

#[derive(Reformation)]
#[reformation(r"{year}")]
struct ExpiryYear {
    year: usize,
}

#[derive(Reformation, Debug)]
enum Height {
    #[reformation(r"{}cm")]
    Cm(i32),
    #[reformation(r"{}in")]
    In(i32),
}

#[derive(Reformation)]
#[reformation(r"{colour}")]
struct HairColour {
    #[reformation(r"#([0-9]|[a-f]){6}")]
    #[allow(dead_code)]
    colour: String,
}

#[derive(Reformation)]
#[reformation(r"{colour}")]
struct EyeColour {
    #[reformation(r"(amb|blu|brn|gry|grn|hzl|oth)")]
    #[allow(dead_code)]
    colour: String,
}

#[derive(Reformation)]
#[reformation(r"{id}")]
struct PID {
    #[reformation(r"\d{9}")]
    #[allow(dead_code)]
    id: String,
}

fn main() {
    let f = File::open("input/input4_1.txt").unwrap();
    let reader = BufReader::new(f);
    let result = reader
        .lines()
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
        })
        .0
        .into_iter()
        .filter(|passport| valid_fields(passport))
        .count();
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
            _ => panic!("Invalid key"),
        };
    });
    result
}

fn valid_fields(passport: &Passport) -> bool {
    passport.byr.clone().map_or(false, |byr_str| {
        byr_str.len() == 4
            && BirthYear::parse(byr_str.as_str()).map_or_else(
                |_| panic!("byr: {}", byr_str),
                |byr| byr.year >= 1920 && byr.year <= 2002,
            )
    }) && passport.iyr.clone().map_or(false, |iyr_str| {
        iyr_str.len() == 4
            && IssueYear::parse(iyr_str.as_str())
                .map_or(false, |iyr| iyr.year >= 2010 && iyr.year <= 2020)
    }) && passport.eyr.clone().map_or(false, |eyr_str| {
        eyr_str.len() == 4
            && ExpiryYear::parse(eyr_str.as_str())
                .map_or(false, |eyr| eyr.year >= 2020 && eyr.year <= 2030)
    }) && passport.hgt.clone().map_or(false, |hgt_str| {
        let height = Height::parse(hgt_str.as_str());
        match height {
            Ok(Height::Cm(cm)) => cm >= 150 && cm <= 193,
            Ok(Height::In(inches)) => inches >= 59 && inches <= 76,
            _ => false,
        }
    }) && passport
        .hcl
        .clone()
        .map_or(false, |hair| HairColour::parse(hair.as_str()).is_ok())
        && passport.ecl.clone().map_or(false, |eye_colour| {
            EyeColour::parse(eye_colour.as_str()).is_ok()
        })
        && passport
            .pid
            .clone()
            .map_or(false, |p| PID::parse(p.as_str()).is_ok())
}

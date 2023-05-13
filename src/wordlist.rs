use std::io::{self, Write, BufRead, BufReader, Error};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use colored::Colorize;

use crate::config::*;
use crate::munge::*;

// Function to read the wordlist
pub fn read_wordlist(filename: &str) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let wordlist: Result<Vec<String>, Error> = reader.lines().collect();
    wordlist
}

// Function for taking input from the user
fn input_prompt(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush the stdout!");
    io::stdin().read_line(&mut input).expect("Failed to read input!");
    input.trim().to_string().to_lowercase()
}

// Function to write the generated wordlist in a file
pub fn write_to_file(filename: &str, data: &[String]) -> std::io::Result<()> {
    let content = data.join("\n");

    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

// Function to capitalize the first letter of any word
fn capitalize(s: &str) -> String {
    if let Some(c) = s.chars().next() {
        let first_letter = c.to_uppercase();
        let remaining_letters = s.chars().skip(1).collect::<String>();
        format!("{}{}", first_letter, remaining_letters)
    } else {
        String::new()
    }
}

// Function to reverse specified word
pub fn reverse_string(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

// Function to combine two strings together
pub fn comb(a: &[String], b: &[String], sep: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in a {
        result.push(i.clone());
        let b_refs = b.iter().cloned().collect::<Vec<String>>();
        for j in &b_refs {
            result.push(format!("{}{}{}", i, sep, j));
        }
    }
    result
}

// Function to concatenate strings together
pub fn concats<'a>(seq: &'a [String], start: u32, stop: u32) -> impl Iterator<Item = String> + 'a {
    seq.iter()
        .flat_map(move |mystr| (start..stop)
        .map(move |num| format!("{}{}", mystr, num)))
}

// Function to clone a given hashmap
pub fn deduplicate(items: &[String]) -> HashSet<String> {
    items.iter().cloned().collect()
}

// Getting profile data from user's input
pub fn profile_data() -> HashMap<String, String> {
    let mut profile: HashMap<String, String> = HashMap::new();

    let mut name = input_prompt("Enter Name: ");
    while name.len() == 0 {
        println!("\n{}\n", "Enter the name atleast...".yellow());
        name = input_prompt("Enter Name: ");
    }

    profile.insert(String::from("name"), name);
    profile.insert(String::from("surname"), input_prompt("Enter Surname: "));
    profile.insert(String::from("nick"), input_prompt("Enter Nickname: "));

    let mut mbd = input_prompt("Enter Birth Date [DDMMYYYY]: ");
    while mbd.len() != 8 {
        if mbd.len() == 0 {
            break;
        } else {
            println!("\n{}\n", "Enter the DOB in [DDMMYYYY] (i.e. 01011998)...".yellow());
            mbd = input_prompt("Enter Birth Date [DDMMYYYY]: ");
        }
    }
    
    profile.insert(String::from("birthdate"), mbd);
    profile.insert(String::from("wife"), input_prompt("Enter Wife Name: "));
    profile.insert(String::from("wifen"), input_prompt("Enter Wife Nickname: "));

    let mut wbd = input_prompt("Enter Wife's Birth Date [DDMMYYYY]: ");
    while wbd.len() != 8 {
        if wbd.len() == 0 {
            break;
        } else {
            println!("\n{}\n", "Enter the DOB in [DDMMYYYY] (i.e. 01011998)...".yellow());
            wbd = input_prompt("Enter Birth Date [DDMMYYYY]: ");
        }
    }

    profile.insert(String::from("wifeb"), wbd);
    profile.insert(String::from("kid"), input_prompt("Enter Kid Name: "));
    profile.insert(String::from("kidn"), input_prompt("Enter Kid Nickname: "));

    let mut kbd = input_prompt("Enter Kid's Birth Date [DDMMYYYY]: ");
    while kbd.len() != 8 {
        if kbd.len() == 0 {
            break;
        } else {
            println!("\n{}\n", "Enter the DOB in [DDMMYYYY] (i.e. 01011998)...".yellow());
            kbd = input_prompt("Enter Birth Date [DDMMYYYY]: ");
        }
    }

    profile.insert(String::from("kidb"), kbd);
    profile.insert(String::from("pet"), input_prompt("Enter Pet Name: "));
    profile.insert(String::from("company"), input_prompt("Enter Company Name: "));
    profile.insert(String::from("words"), input_prompt("Enter Any Keywords (Seperated by ',' i.e. hacker,kid,coding): "));

    let spec = match input_prompt("Include Special Character [y/N]: ").to_lowercase().as_str() {
        "y" => String::from("true"),
        _ => String::from("false"),
    };
    let randnum = match input_prompt("Include Random Numbers at the End [y/N]: ").to_lowercase().as_str() {
        "y" => String::from("true"),
        _ => String::from("false"),
    };
    let leet = match input_prompt("Add Leet Characters in wordlist [y/N]: ").to_lowercase().as_str() {
        "y" => String::from("true"),
        _ => String::from("false"),
    };

    profile.insert(String::from("spec"), spec);
    profile.insert(String::from("randnum"), randnum);
    profile.insert(String::from("leet"), leet);

    profile
}

// Now here we have to do some string modifications
// Function to generate the complete wordlist and do the string modifications
// and all of the program's magic.
pub fn generate_wordlists(output: String) {
    let profile = profile_data();

    let mbd = profile.get("birthdate").unwrap_or(&"".to_string()).clone();
    let (mbd_yy, mbd_yyy, mbd_yyyy, mbd_xd, mbd_xm, mbd_dd, mbd_mm) = if mbd.len() == 8 {
        (
            mbd[mbd.len() - 2..].to_string(),
            mbd[mbd.len() - 3..].to_string(),
            mbd[mbd.len() - 4..].to_string(),
            mbd[1..2].to_string(),
            mbd[3..4].to_string(),
            mbd[..2].to_string(),
            mbd[2..4].to_string(),
        )
    } else {
        (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        )
    };

    let wbd = profile.get("wifeb").unwrap_or(&"".to_string()).clone();
    let (wbd_yy, wbd_yyy, wbd_yyyy, wbd_xd, wbd_xm, wbd_dd, wbd_mm) = if wbd.len() == 8 {
        (
            wbd[wbd.len() - 2..].to_string(),
            wbd[wbd.len() - 3..].to_string(),
            wbd[wbd.len() - 4..].to_string(),
            wbd[1..2].to_string(),
            wbd[3..4].to_string(),
            wbd[..2].to_string(),
            wbd[2..4].to_string(),
        )
    } else {
        (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        )
    };

    let kbd = profile.get("kidb").unwrap_or(&"".to_string()).clone();
    let (kbd_yy, kbd_yyy, kbd_yyyy, kbd_xd, kbd_xm, kbd_dd, kbd_mm) = if wbd.len() == 8 {
        (
            kbd[kbd.len() - 2..].to_string(),
            kbd[kbd.len() - 3..].to_string(),
            kbd[kbd.len() - 4..].to_string(),
            kbd[1..2].to_string(),
            kbd[3..4].to_string(),
            kbd[..2].to_string(),
            kbd[2..4].to_string(),
        )
    } else {
        (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        )
    };

    // Convert first letters to uppercase...

    let nameup = capitalize(profile.get("name").expect("Name not found").as_str());
    let surnameup = capitalize(profile.get("surname").expect("Surname not found").as_str());
    let nickup = capitalize(profile.get("nick").expect("Nickname not found").as_str());
    let wifeup = capitalize(profile.get("wife").expect("Wife's name not found").as_str());
    let wifenup = capitalize(profile.get("wifen").expect("Wife's nickname not found").as_str());
    let kidup = capitalize(profile.get("kid").expect("Wife's nickname not found").as_str());
    let kidnup = capitalize(profile.get("kidn").expect("Kid's name not found").as_str());
    let petup = capitalize(profile.get("pet").expect("Kid's nickname not found").as_str());
    let companyup = capitalize(profile.get("company").expect("Company name not found").as_str());
    let word: Vec<&str> = profile.get("words").expect("Words not found").split(',').collect();
    let mut words: Vec<String> = word.iter().map(|&s| s.to_string()).collect();

    let mut wordup: Vec<String> = Vec::new();

    for i in &word {
        wordup.push(capitalize(i));
    }

    words.extend(wordup);

    // Reverse the names
    let rev_name = reverse_string(profile.get("name").expect("Name not found"));
    let rev_nameup = reverse_string(&nameup);
    let rev_nick = reverse_string(profile.get("nick").expect("Nickname not found"));
    let rev_nickup = reverse_string(&nickup);
    let rev_wife = reverse_string(profile.get("wife").expect("Wife's name not found"));
    let rev_wifeup = reverse_string(&wifeup);
    let rev_wifen = reverse_string(profile.get("wifen").expect("Wife's nickname not found"));
    let rev_wifenup = reverse_string(&wifenup);
    let rev_kid = reverse_string(profile.get("kid").expect("Kid's name not found"));
    let rev_kidup = reverse_string(&kidup);
    let rev_kidn = reverse_string(profile.get("kidn").expect("Kid's nickname not found"));
    let rev_kidnup = reverse_string(&kidnup);

    let reverse: Vec<String> = vec![
        rev_name.clone(),
        rev_nameup.clone(),
        rev_nick.clone(),
        rev_nickup.clone(),
        rev_wife.clone(),
        rev_wifeup.clone(),
        rev_wifen.clone(),
        rev_wifenup.clone(),
        rev_kid.clone(),
        rev_kidup.clone(),
        rev_kidn.clone(),
        rev_kidnup.clone(),
    ];

    let rev_m: Vec<String> = vec![rev_name, rev_nameup, rev_nick, rev_nickup];
    let rev_w: Vec<String> = vec![rev_wife, rev_wifeup, rev_wifen, rev_wifenup];
    let rev_k: Vec<String> = vec![rev_kid, rev_kidup, rev_kidn, rev_kidnup];

    // Now it's time to do some serious work, the code from here is going to be dirty, But...
    // Who cares ;)
    
    // Birthdate combinations

    let mbds: Vec<String> = vec![
        mbd_yy,
        mbd_yyy,
        mbd_yyyy,
        mbd_xd,
        mbd_xm,
        mbd_dd,
        mbd_mm,
    ];

    let mut mbdss: Vec<String> = Vec::new();
    if mbds.len() == 7 {
        for i in &mbds {
            mbdss.push(i.to_string());
            for j in &mbds {
                if mbds.iter().position(|x| x == i) != mbds.iter().position(|x| x == j) {
                    mbdss.push(format!("{}{}", i, j));
                    for k in &mbds {
                        if mbds.iter().position(|x| x == i) != mbds.iter().position(|x| x == j)
                            && mbds.iter().position(|x| x == j) != mbds.iter().position(|x| x == k)
                            && mbds.iter().position(|x| x == i) != mbds.iter().position(|x| x == k)
                        {
                            mbdss.push(format!("{}{}{}", i, j, k));
                        }
                    }
                }
            }
        }
    }

    // For Wife's Birthdate
    let wbds: Vec<String> = vec![
        wbd_yy,
        wbd_yyy,
        wbd_yyyy,
        wbd_xd,
        wbd_xm,
        wbd_dd,
        wbd_mm,
    ];

    let mut wbdss: Vec<String> = Vec::new();
    if wbds.len() == 7 {
        for i in &wbds {
            wbdss.push(i.to_string());
            for j in &wbds {
                if wbds.iter().position(|x| x == i) != wbds.iter().position(|x| x == j) {
                    wbdss.push(format!("{}{}", i, j));
                    for k in &wbds {
                        if wbds.iter().position(|x| x == i) != wbds.iter().position(|x| x == j)
                            && wbds.iter().position(|x| x == j) != wbds.iter().position(|x| x == k)
                            && wbds.iter().position(|x| x == i) != wbds.iter().position(|x| x == k)
                        {
                            wbdss.push(format!("{}{}{}", i, j, k));
                        }
                    }
                }
            }
        }
    }

    // For Kid's Birthdate
    let kbds: Vec<String> = vec![
        kbd_yy,
        kbd_yyy,
        kbd_yyyy,
        kbd_xd,
        kbd_xm,
        kbd_dd,
        kbd_mm,
    ];

    let mut kbdss: Vec<String> = Vec::new();
    if kbds.len() == 7 {
        for i in &kbds {
            kbdss.push(i.to_string());
            for j in &kbds {
                if kbds.iter().position(|x| x == i) != kbds.iter().position(|x| x == j) {
                    kbdss.push(format!("{}{}", i, j));
                    for k in &kbds {
                        if kbds.iter().position(|x| x == i) != kbds.iter().position(|x| x == j)
                            && kbds.iter().position(|x| x == j) != kbds.iter().position(|x| x == k)
                            && kbds.iter().position(|x| x == i) != kbds.iter().position(|x| x == k)
                        {
                            kbdss.push(format!("{}{}{}", i, j, k));
                        }
                    }
                }
            }
        }
    }

    // String Combinations
    let other_info: Vec<String> = vec![
        profile.get("pet").expect("Value not found").to_string(),
        petup,
        profile.get("company").expect("Value not found").to_string(),
        companyup,
    ];

    let combi_m: Vec<String> = vec![
        profile.get("name").expect("Value not found").to_string(),
        profile.get("surname").expect("Value not found").to_string(),
        profile.get("nick").expect("Value not found").to_string(),
        nameup,
        surnameup.clone(),
        nickup,
    ];

    let combi_w: Vec<String> = vec![
        profile.get("wife").expect("Value not found").to_string(),
        profile.get("surname").expect("Value not found").to_string(),
        profile.get("wifen").expect("Value not found").to_string(),
        wifeup,
        surnameup.clone(),
        wifenup,
    ];

    let combi_k: Vec<String> = vec![
        profile.get("kid").expect("Value not found").to_string(),
        profile.get("surname").expect("Value not found").to_string(),
        profile.get("kidn").expect("Value not found").to_string(),
        kidup,
        surnameup.clone(),
        kidnup,
    ];

    let mut combia_m: Vec<String> = Vec::new();
    for i in combi_m.clone() {
        combia_m.push(i.to_string());
        for j in combi_m.clone() {
            if combi_m.iter().position(|x| x == &i) != combi_m.iter().position(|x| x == &j)
                && combi_m.iter().position(|x| x == &capitalize(&i)) != combi_m.iter().position(|x| x == &capitalize(&j))
            {
                combia_m.push(format!("{}{}", i, j));
            }
        }
    }

    let mut combia_w: Vec<String> = Vec::new();
    for i in combi_w.clone() {
        combia_w.push(i.to_string());
        for j in combi_w.clone() {
            if combi_w.iter().position(|x| x == &i) != combi_w.iter().position(|x| x == &j)
                && combi_w.iter().position(|x| x == &capitalize(&i)) != combi_w.iter().position(|x| x == &capitalize(&j))
            {
                combia_w.push(format!("{}{}", i, j));
            }
        }
    }

    let mut combia_k: Vec<String> = Vec::new();
    for i in combi_k.clone() {
        combia_k.push(i.to_string());
        for j in combi_k.clone() {
            if combi_k.iter().position(|x| x == &i) != combi_k.iter().position(|x| x == &j)
                && combi_k.iter().position(|x| x == &capitalize(&i)) != combi_k.iter().position(|x| x == &capitalize(&j))
            {
                combia_k.push(format!("{}{}", i, j));
            }
        }
    }

    let years: Vec<String> = (1990..=2023).map(|year| year.to_string()).collect();
    let mut comb_g: HashMap<usize, Vec<String>> = HashMap::new();

    comb_g.insert(1, comb(&combia_m, &mbdss, ""));
    comb_g.get_mut(&1).unwrap().extend(comb(&combia_m, &mbdss, "_"));
    comb_g.insert(2, comb(&combia_w, &wbdss, ""));
    comb_g.get_mut(&2).unwrap().extend(comb(&combia_w, &wbdss, "_"));
    comb_g.insert(3, comb(&combia_k, &kbdss, ""));
    comb_g.get_mut(&3).unwrap().extend(comb(&combia_k, &kbdss, "_"));
    comb_g.insert(4, comb(&combia_m, &years, ""));
    comb_g.get_mut(&4).unwrap().extend(comb(&combia_m, &years, "_"));
    comb_g.insert(5, comb(&other_info, &years, ""));
    comb_g.get_mut(&5).unwrap().extend(comb(&other_info, &years, "_"));
    comb_g.insert(6, comb(&combia_w, &years, ""));
    comb_g.get_mut(&6).unwrap().extend(comb(&combia_w, &years, "_"));
    comb_g.insert(7, comb(&combia_k, &years, ""));
    comb_g.get_mut(&7).unwrap().extend(comb(&combia_k, &years, "_"));
    comb_g.insert(8, comb(&words, &mbdss, ""));
    comb_g.get_mut(&8).unwrap().extend(comb(&words, &mbdss, "_"));
    comb_g.insert(9, comb(&words, &wbdss, ""));
    comb_g.get_mut(&9).unwrap().extend(comb(&words, &wbdss, "_"));
    comb_g.insert(10, comb(&words, &kbdss, ""));
    comb_g.get_mut(&10).unwrap().extend(comb(&words, &kbdss, "_"));
    comb_g.insert(11, comb(&words, &years, ""));
    comb_g.get_mut(&11).unwrap().extend(comb(&words, &years, "_"));
    comb_g.insert(12, vec!["".to_string()]);
    comb_g.insert(13, vec!["".to_string()]);
    comb_g.insert(14, vec!["".to_string()]);
    comb_g.insert(15, vec!["".to_string()]);
    comb_g.insert(16, vec!["".to_string()]);
    comb_g.insert(21, vec!["".to_string()]);

    if profile.get("randnum").map(|s| s == "true").unwrap_or(false) {
        comb_g.insert(12, concats(&words, CONFIG_NUMFROM, CONFIG_NUMTO).collect());
        comb_g.insert(13, concats(&combi_m, CONFIG_NUMFROM, CONFIG_NUMTO).collect());
        comb_g.insert(14, concats(&other_info, CONFIG_NUMFROM, CONFIG_NUMTO).collect());
        comb_g.insert(15, concats(&combi_w, CONFIG_NUMFROM, CONFIG_NUMTO).collect());
        comb_g.insert(16, concats(&combi_k, CONFIG_NUMFROM, CONFIG_NUMTO).collect());
        comb_g.insert(21, concats(&reverse, CONFIG_NUMFROM, CONFIG_NUMTO).collect());
    }

    comb_g.insert(17, comb(&reverse, &years, ""));
    comb_g.get_mut(&17).unwrap().extend(comb(&reverse, &years, "_"));
    comb_g.insert(18, comb(&rev_w, &wbdss, ""));
    comb_g.get_mut(&18).unwrap().extend(comb(&rev_w, &wbdss, "_"));
    comb_g.insert(19, comb(&rev_k, &kbdss, ""));
    comb_g.get_mut(&19).unwrap().extend(comb(&rev_k, &kbdss, "_"));
    comb_g.insert(20, comb(&rev_m, &mbdss, ""));
    comb_g.get_mut(&20).unwrap().extend(comb(&rev_m, &mbdss, "_"));

    let mut comb001: Vec<String> = vec!["".to_string()];
    let mut comb002: Vec<String> = vec!["".to_string()];
    let mut comb003: Vec<String> = vec!["".to_string()];
    let mut comb004: Vec<String> = vec!["".to_string()];
    let mut comb005: Vec<String> = vec!["".to_string()];
    let mut comb006: Vec<String> = vec!["".to_string()];

    let spechars: Vec<String> = vec!["!".to_string(), "@".to_string(), 
        "#".to_string(), "$".to_string(), "%%".to_string(),
        "&".to_string(), "*".to_string()];

    if spechars.len() > 0 {
        comb001 = comb(&combi_m, &spechars, "");
        comb002 = comb(&other_info, &spechars, "");
        comb003 = comb(&combi_w, &spechars, "");
        comb004 = comb(&combi_k, &spechars, "");
        comb005 = comb(&words, &spechars, "");
        comb006 = comb(&reverse, &spechars, "");    
    }

    let mut comb_unique: Vec<HashSet<String>> = vec![HashSet::new(); 22];

    for i in 1..22 {
        comb_unique[i] = deduplicate(&comb_g[&i]);
    }

    let comb_unique01 = deduplicate(&combi_m);
    let comb_unique02 = deduplicate(&other_info);
    let comb_unique03 = deduplicate(&combi_w);
    let comb_unique04 = deduplicate(&combi_k);
    let comb_unique05 = deduplicate(&words);
    let comb_unique07 = deduplicate(&comb001);
    let comb_unique08 = deduplicate(&comb002);
    let comb_unique09 = deduplicate(&comb003);
    let comb_unique010 = deduplicate(&comb004);
    let comb_unique011 = deduplicate(&comb005);
    let comb_unique012 = deduplicate(&comb006);

    let mut uniqlist = mbdss
        .iter()
        .chain(wbdss.iter())
        .chain(kbdss.iter())
        .chain(reverse.iter())
        .chain(comb_unique01.iter())
        .chain(comb_unique02.iter())
        .chain(comb_unique03.iter())
        .chain(comb_unique04.iter())
        .chain(comb_unique05.iter())
        .cloned()
        .collect::<HashSet<String>>();

    for i in 1..21 {
        uniqlist.extend(comb_unique[i].iter().cloned());
    }

    uniqlist.extend(
        comb_unique07
            .iter()
            .chain(comb_unique08.iter())
            .chain(comb_unique09.iter())
            .chain(comb_unique010.iter())
            .chain(comb_unique011.iter())
            .chain(comb_unique012.iter())
            .cloned(),
    );

    // Convert the wordlist into leet characters
    let mut leet_wordlist: Vec<String> = Vec::new();
    if profile.get("leet").map(|s| s == "true").unwrap_or(false) {
        leet_wordlist = leet_speak(uniqlist.iter().cloned().collect());
    }

    let mut unique_lista: Vec<String> = uniqlist.iter().cloned().collect();
    unique_lista.extend(leet_wordlist);
    let mut unique_list_finished: Vec<String> = Vec::new();

    for i in &unique_lista {
        if i.len() >= 5 {
            unique_list_finished.push(i.to_string());
        }
    }

    // Writing data into the output file.
    match write_to_file(&output, &unique_list_finished) {
        Ok(()) => println!("\n[{}] {}\t({} words)", "+"
                .bold()
                .green(),
            "Wordlist generated successfully..."
                .green(),
            &unique_list_finished
                .len()
                .to_string()
                .cyan()),
        Err(_) => println!("Error while generating the wordlist file..."),
    };

}


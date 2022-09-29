use std::io;

fn main() {
    let mut s1 = String::new();
    match io::stdin().read_line(&mut s1) {
        Ok(_) => {},
        Err(_) => println!("Failed to read input"),
    }
    s1.make_ascii_lowercase();
    match s1.find("mi") {
        Some(_) => convert_mi(&s1),
        None => match s1.find("ft") {
            Some(_) => convert_f(&s1),
            None => match s1.find("km") { 
                Some(_) => convert_km(&s1),
                None => match s1.find('m') {
                    Some(_) => convert_m(&s1),
                    None => match s1.find(char::is_alphabetic) {
                        Some(_) => println!("Please label units with [Mi] Miles, [Km] Kilometers,\n\
                        [Ft] Feet, and [M] Meters"),
                        None => println!("Didn't find a label, Please label units with [Mi] Miles,\n\
                        [Km] Kilometers, [Ft] Feet, and [M] Meters"),
                    },
                },
            },
        },
     }
}

fn convert_m(temp: &str) {
    let num = temp.replace('m',"");
    let num: f32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Please input a number"); return },
    };
    println!("{}Ft", (num * 100.) / 2.54 / 12.)
}

fn convert_f(temp: &str) {
    let t_num = temp.replace("ft","");
    let num: f32 = match t_num.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("Please input a number"); return },
    };
    println!("{}M", (num * 12. * 2.54 ) / 100.)
}

fn convert_km(temp: &str) {
    let t_num = temp.replace("km","");
    let num: f32 = match t_num.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("Please input a number"); return },
    };
    println!("{}Mi", num / 1.609344)
}

fn convert_mi(temp: &str) {
    let t_num = temp.replace("mi","");
    let num: f32 = match t_num.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("Please input a number"); return },
    };
    println!("{}Km", num * 1.609344)
}

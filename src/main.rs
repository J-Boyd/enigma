use enigma::Enigma;
use enigma::Rotor;
use enigma::Reflector;
use enigma::Plugboard;

use std::io::{self, Write};
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
{
    #[arg(long, value_parser = reflector_parser, required = true,
        help = "Sets the reflector type.")]
    reflector: String,

    #[arg(long, value_parser = rotor_parser, num_args = 3, value_name = "ROTOR", required = true,
        help = "Sets the rotor order (Walzenlage), starting in the leftmost position.")]
    rotors: Vec<String>,

    #[arg(long, value_parser = clap::value_parser!(u32).range(1..=26), num_args = 3, value_name = "SETTING", required = true,
        help = "Sets the ring settings for the rotors (Ringstellung). Valid values are numbers in the range 1 to 26.")]
    rings: Vec<u32>,

    #[arg(long, value_parser = key_parser, required = true,
        help = "Sets the intial positions for the rotors (Grundstellung/Kenngruppen). Valid values are letters in the range A to Z.")]
    key: String,

    #[arg(long, value_parser = plug_parser, required = false, num_args = 0..=10, value_name = "PLUG", 
        help = "Sets the plug connections on the plugboard (Steckerverbindungen). Valid values are pairs of letters such as 'AL' for linking the letter 'A' to the letter 'L'.")]
    plugs: Vec<String>
}

fn reflector_parser(s: &str) -> Result<String, String> {
    let reflectors = ["Beta", "Gamma", "A", "B", "C", "ThinB", "ThinC", "ETW"];

    if reflectors.contains(&s) {
        return Ok(s.to_string());
    }

    Err(format!("Must be one of {:?}", reflectors))
}

fn rotor_parser(s: &str) -> Result<String, String> {
    let rotors = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII"];

    if rotors.contains(&s) {
        return Ok(s.to_string());
    }

    Err(format!("Must be one of {:?}", rotors))
}

fn key_parser(s: &str) -> Result<String, String> {
    if s.len() > 3 {
        return Err(format!("Too many key values. 3 keys are required."));
    }

    if s.len() < 3 {
        return Err(format!("Not enough key values. 3 keys are required."));
    }

    Ok(s.to_string())
}

fn plug_parser(s: &str) -> Result<String, String> {
    if s.len() != 2 {
        return Err(format!("Not a valid plug pair."))
    }

    Ok(s.to_string())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("{:?}", args);

    let reflector = Reflector::get_reflector_type_from_string(&args.reflector)?;
    let mut rotors: Vec<Rotor> = Vec::new();

    for i in 0..args.rotors.len() {

        let rotor_type = Rotor::get_rotor_type_from_string(&args.rotors[i])?;
        let position = args.key.chars().nth(i).unwrap();
        let ring = args.rings[i] as usize;

        let rotor = Rotor::new(rotor_type, position, ring)?;
        rotors.push(rotor);
    }

    let mut plugs: Vec<[char; 2]> = Vec::new();

    for i in 0..args.plugs.len() {
        let mut pair = args.plugs[i].chars();
        plugs.push([pair.next().unwrap(), pair.next().unwrap()]);
    }

    let plugboard = Plugboard::new(&plugs)?;
    let mut enigma = Enigma::new(reflector, rotors, plugboard);

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!(">");

        std::io::stdout().flush()?;

        stdin.read_line(&mut buffer)?;

        if buffer.trim().eq("exit") {
            break;
        }

        let output = enigma.encrypt(buffer.trim())?;
        println!("{}", output);

        buffer.clear();
    }

    Ok(())
}
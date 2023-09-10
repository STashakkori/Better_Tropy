extern crate structopt;

use structopt::StructOpt;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::exit;
use tropy::colour::{Hsl, Rgb};
use tropy::Calculator;
use ansi_rgb::{ Foreground, Background };
use rgb::RGB8;

/// Read bytes from file or stdin and calculate the Shannon entropy for for chunks of a fixed size.
/// Then display it colour-coded in the terminal or write it to stdout as csv.
#[derive(Debug, StructOpt)]
struct Tropy {
    #[structopt(
        name = "input",
        help = "File to be read for input or \'-\' to use open stdin"
    )]
    file: String,
    #[structopt(
        long = "bytes",
        default_value = "1024",
        help = "The number of bytes to be read for each entropy calculation"
    )]
    bytes: u32,
    #[structopt(
        long = "csv",
        help = "Output as csv to stdout instead of using color-coding on the terminal.\nFormats as: <startbyte>;<entropy>"
    )]
    csv: bool,
}

fn main() {
    let cfg = Tropy::from_args();
    let mut r: Box<dyn BufRead> = {
        if cfg.file.eq("-") {
            let s = io::stdin();
            eprintln!("* Using stdin for input");
            let r = BufReader::with_capacity(2048usize, s);
            Box::new(r)
        } else {
            let s = match File::open(&cfg.file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Opening file failed with: {}", e.description());
                    exit(e.raw_os_error().unwrap_or(1))
                }
            };
            eprintln!("*\x1b[38;5;10mUsing {} for input\x1b[0m", cfg.file);
            let r = BufReader::with_capacity(2048usize, s);
            Box::new(r)
        }
    };

    let mut chunksize = cfg.bytes as usize;
    let mut buf = vec![0u8; chunksize];
    let mut c = Calculator::new();
    let mut chunknum = 0usize;
    let mut addr = 0;
    eprintln!("*\x1b[38;5;10mUsing chunks of {}bytes\x1b[0m", chunksize);

    if !cfg.csv {
  let test = String::from("  ");
  println!("Entropy color map:");     
  let colored_test = ansi_rgb_string(test.clone(),0,0,0);
          print!("Low {}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,0,1);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,0,2);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,0,3);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,0,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,0,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,1,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,1,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,1,3);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),0,2,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),1,4,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,5,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,4,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),1,2,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),1,2,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,2,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,1,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,0,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,0,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),1,0,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),1,0,3);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),1,0,2);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,0,3);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),3,0,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),4,0,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),4,0,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),5,0,5);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),5,0,4);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),5,0,3);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),4,0,3);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),4,0,2);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),4,0,1);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),5,0,2);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),5,0,1);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),5,0,0);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),4,0,0);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),3,0,0);
          print!("{}", colored_test);
  let colored_test =  ansi_rgb_string(test.clone(),2,0,0);
          print!("\x1b[0m{} High\n\x1b[0m", colored_test);
    } else {
        // use raw data
        eprintln!("Outputting raw data as csv in the format <startbyte>;<entropy/byte>");
        println!("\"start\";\"entropy\"");
    }
    println!();
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    println!();
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m blksize={}B\x1b[0m",chunksize);
    while let Ok(_) = r.read_exact(&mut buf[..]).and_then(|_| c.write(&buf[..])) {
        let e = c.entropy();

        if !cfg.csv {
            if chunknum % 80 == 0 {
              if addr == 0 { 
                addr = addr + (80 * chunksize);
              }
              else {
                print!(" \x1b[38;5;208m{:#04x}\x1b[0m", addr);
                addr = addr + (80 * chunksize);
              }
              println!();
            }
            // scale entropy to bits (i.e. value/8)
            // i.e. perfectly uniform data would have an entropy of 1 (i.e. 8bits/byte)
            let h = (240.0 + e / 8.0 * 120.0) as u32;
            //print!("{} ", h);
            let out = ansi_rgb_string_special("█".to_string(), h, 1, 1);
            print!("{}", out);
        } else {
            // output as csv
            println!("{};{:.6}", chunknum * chunksize, e);
        }

        chunknum += 1;
    }
    if chunknum > 0 {
      print!(" \x1b[38;5;208m{:#04x}\x1b[0m", (addr - (80 * chunksize) + ((chunknum % 80) * chunksize)));
    }
    println!();
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    println!();
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m1\x1b[0m");
    print!("\x1b[38;5;11m2\x1b[0m");
    print!("\x1b[38;5;11m3\x1b[0m");
    print!("\x1b[38;5;11m4\x1b[0m");
    print!("\x1b[38;5;11m5\x1b[0m");
    print!("\x1b[38;5;11m6\x1b[0m");
    print!("\x1b[38;5;11m7\x1b[0m");
    print!("\x1b[38;5;11m8\x1b[0m");
    print!("\x1b[38;5;11m9\x1b[0m");
    print!("\x1b[38;5;11ma\x1b[0m");
    print!("\x1b[38;5;11mb\x1b[0m");
    print!("\x1b[38;5;11mc\x1b[0m");
    print!("\x1b[38;5;11md\x1b[0m");
    print!("\x1b[38;5;11me\x1b[0m");
    print!("\x1b[38;5;11mf\x1b[0m");
    print!("\x1b[38;5;11m0\x1b[0m");
    print!("\x1b[38;5;11m blksize={}B\x1b[0m",chunksize);
    println!();
}

fn ansi_rgb_string_special(s: String, mut r: u32, mut g: u32, mut b: u32) -> String {
  let mut ansi_code = String::from("\x1b[38;5;");
  if r < 255 { r = 0; g = 0; b = 0; } //000
  else if r >= 255 && r < 260 { r = 0; g = 0; b = 1; } //001
  else if r >= 260 && r < 265 { r = 0; g = 0; b = 2; } //002
  else if r >= 265 && r < 270 { r = 0; g = 0; b = 3; } //003
  else if r >= 270 && r < 272 { r = 0; g = 0; b = 4; } //004
  else if r >= 272 && r < 274 { r = 0; g = 0; b = 5; } //005
  else if r >= 274 && r < 276 { r = 0; g = 1; b = 5; } //015
  else if r >= 276 && r < 278 { r = 0; g = 1; b = 4; } //014
  else if r >= 278 && r < 280 { r = 0; g = 1; b = 3; } //013 
  else if r >= 280 && r < 285 { r = 0; g = 2; b = 4; } //024
  else if r >= 285 && r < 290 { r = 1; g = 4; b = 5; } //145
  else if r >= 290 && r < 292 { r = 2; g = 5; b = 5; } //255
  else if r >= 292 && r < 294 { r = 2; g = 4; b = 5; } //245
  else if r >= 294 && r < 296 { r = 1; g = 2; b = 4; } //124
  else if r >= 296 && r < 298 { r = 1; g = 2; b = 5; } //125
  else if r >= 298 && r < 300 { r = 2; g = 2; b = 5; } //225
  else if r >= 300 && r < 305 { r = 2; g = 1; b = 5; } //215
  else if r >= 305 && r < 310 { r = 2; g = 0; b = 5; } //205
  else if r >= 310 && r < 312 { r = 2; g = 0; b = 4; } //204
  else if r >= 312 && r < 314 { r = 1; g = 0; b = 4; } //104
  else if r >= 314 && r < 316 { r = 1; g = 0; b = 3; } //103
  else if r >= 316 && r < 318 { r = 1; g = 0; b = 2; } //102
  else if r >= 318 && r < 320 { r = 2; g = 0; b = 3; } //203
  else if r >= 320 && r < 322 { r = 3; g = 0; b = 4; } //304
  else if r >= 322 && r < 324 { r = 4; g = 0; b = 5; } //405
  else if r >= 324 && r < 326 { r = 4; g = 0; b = 4; } //404
  else if r >= 326 && r < 328 { r = 5; g = 0; b = 5; } //505
  else if r >= 328 && r < 330 { r = 5; g = 0; b = 4; } //504
  else if r >= 330 && r < 334 { r = 5; g = 0; b = 3; } //503
  else if r >= 334 && r < 340 { r = 4; g = 0; b = 3; } //403
  else if r >= 340 && r < 342 { r = 4; g = 0; b = 2; } //402
  else if r >= 342 && r < 344 { r = 4; g = 0; b = 1; } //401
  else if r >= 344 && r < 346 { r = 5; g = 0; b = 2; } //502
  else if r >= 346 && r < 350 { r = 5; g = 0; b = 1; } //501
  else if r >= 350 && r < 355 { r = 5; g = 0; b = 0; } //500
  else if r >= 355 && r < 360 { r = 4; g = 0; b = 0; } //400
  else if r >= 360 && r < 365 { r = 3; g = 0; b = 0; } //300
  else if r > 365 { r = 2; g = 0; b = 0; } //200
  
  let color = 16 + (r * 36) + (g * 6) + b;
  ansi_code.push_str(&color.to_string()[..]);
  ansi_code.push_str("m");
  ansi_code.push_str(&s);
  ansi_code.push_str("\x1b[0m");
  return ansi_code; 
}

fn ansi_rgb_string(s: String, r: u32, g: u32, b: u32) -> String {
  if r > 6 || g > 6 || b > 6 { return s; }
  let mut ansi_code = String::from("\x1b[48;5;");
  let color = 16 + (r * 36) + (g * 6) + b;
  ansi_code.push_str(&color.to_string()[..]);
  ansi_code.push_str("m");
  ansi_code.push_str(&s);
  ansi_code.push_str("\x1b[0m");
  return ansi_code;
}


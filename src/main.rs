use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};

use clap::{Command, command, arg, ArgAction,Arg};

type StreamResult<T> = Result<T,Box<dyn Error>>;

fn open(filepath:&str)-> StreamResult<Box<dyn BufRead>>{
    match filepath{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filepath)?))),
    }
}

fn main() {
    let matches = command!()
    .version("v0.0.1")
    .author("Ken Chan, chankamlam@icloud.com")
    .about(r#"
====================================================================================
   The cat utility reads files sequentially, writing them to the standard output.  The file operands are processed in command-line
order.  If file is a single dash (‘-’) or absent, cat reads from the standard input.  If file is a UNIX domain socket, cat connects
to it and then reads it until EOF. This complements the UNIX domain binding capability available in inetd(8).
===================================================================================="#)
    .arg(
        Arg::new("FILE")
        .help("Input files")
        .default_value("-")
        .action(ArgAction::Append)
    )
    .arg(
        arg!(-n --number "Number the output line, starting at 1.").conflicts_with("number_nonblank")
    )
    .arg(
        arg!(-b --number_nonblank "Number the non-blank output lines, starting at 1.").conflicts_with("number")
    )
    .get_matches();
    let arg_number = matches.get_flag("number");
    let arg_number_nonblank = matches.get_flag("number_nonblank");
    let files = matches.get_many::<String>("FILE").unwrap_or_default().collect::<Vec<_>>();
    // println!("number=>{}",arg_number);
    // println!("number_nonblank=>{}",arg_number_nonblank);
    // println!("files=>{:?}",files);

    for fpath in files{
        match open(fpath){
            Err(err)=> eprintln!("{}=>{}",fpath,err),
            Ok(file)=>{
                let mut idx = 0;
                for line in file.lines(){
                    let content = line.unwrap();
                    if arg_number || arg_number_nonblank{
                        if arg_number_nonblank{
                            if content.is_empty(){
                                println!("{}",content);
                            }else{
                                idx+=1;
                                println!("{:>6}\t{}",idx, content);
                            }
                        }else{
                            idx+=1;
                            println!("{:>6}\t{}",idx, content);
                        }
                    }else{
                        println!("{}", content);
                    }
                }
            },
        }
    }
}

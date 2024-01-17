use clap::Subcommand;


#[derive(Subcommand)]
pub enum CommandsEnum {
    Count {
        /// Word to count
        input: String,

        /// File to read from
        file: String,
    },
    Find {
        /// Word to find
        input: String,

        /// File to read from
        file: String,
      
    },
}

use std::{fs::File, io::{BufReader, Read, Error}};

pub fn count(input: &String, file_path: &String) -> Result<(), Box<Error>>{

    let contents = find_file(file_path)?;

    let mut count = 0;
    for word in contents.split_ascii_whitespace().into_iter(){
      if word == input{
        count+=1;
      }
    }
    
    println!("Há {:?} entradas de {:?}, no arquivo {:?}", count, input, file_path);

    Ok(())
}

pub fn find_word(input: &String, file_path: &String) -> Result<(), Box<Error>>{
  let content = find_file(file_path)?;

  let mut has_word = false;
  
  for word in content.split_ascii_whitespace().into_iter(){
    if word == input {
      has_word = true;
      break;
    }
  }

  match has_word {
      true => println!("Foi encontrado {:?} no {:?}", input, file_path),
      false => println!("{:?} não foi encontrado", input)
  }  

  Ok(())
}

fn find_file(file_path: &String) -> Result<String, Box<Error>>{
  let file = File::open(file_path)?;

  let mut bf = BufReader::new(file);
  let mut contents = String::new();
  bf.read_to_string(&mut contents)?;
  Ok(contents)
}
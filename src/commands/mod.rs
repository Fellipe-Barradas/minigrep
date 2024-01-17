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

use std::{fs::{self, File}, io::{BufReader, Read}};

pub fn count(input: &String, file_path: &String){
    let contents = find_file(file_path);

    let mut count = 0;
    for word in contents.split_ascii_whitespace().into_iter(){
      if word == input{
        count+=1;
      }
    }
    
    
    println!("Há {:?} entradas de {:?}, no arquivo {:?}", count, input, file_path);
}

pub fn find_word(input: &String, file_path: &String){
  let content = find_file(file_path);
  let mut hasWord = false;

  for word in content.split_ascii_whitespace().into_iter(){
    if word == input {
      hasWord = true;
      break;
    }
  }

  match hasWord {
      true => println!("Foi encontrado {:?} no {:?}", input, file_path),
      false => println!("{:?} não foi encontrado", input)
  }  
}

fn find_file(file_path: &String) -> String{
  let file = File::open(file_path).expect("Erro, arquivo não encontrado!");
  let mut bf = BufReader::new(file);
  let mut contents = String::new();
  bf.read_to_string(&mut contents).unwrap();
  contents
}
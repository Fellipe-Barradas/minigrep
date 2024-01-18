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
        /// Words to find
        #[arg(num_args=1,short, long)]
        input: Vec<String>,

        /// File to read from
        #[arg(short, long)]
        file: String,
      
    },
}

use std::{fs::File, io::{BufReader, Read, Error}, collections::HashMap};

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

pub fn find_word(input: &Vec<String>, file_path: &String) -> Result<(), Box<Error>>{
  let content = find_file(file_path)?;

  let mut finded: HashMap<String, bool> = HashMap::new();
  for word in content.split_ascii_whitespace().into_iter(){
    for i in input.iter(){
      if word.contains(i){
        finded.insert(word.to_owned(), true);
        break;
      }
    }
  } 

  finded.iter()
   .filter(|(_,v)|{**v})
    .for_each(|d|{
    println!("{:?}", d.0);
  });
  Ok(())
}

fn find_file(file_path: &String) -> Result<String, Box<Error>>{
  let file = File::open(file_path)?;

  let mut bf = BufReader::new(file);
  let mut contents = String::new();
  bf.read_to_string(&mut contents)?;
  Ok(contents)
}
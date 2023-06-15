// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("{} = nmes",name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn command_with_str(msg :String) -> String{
    format!("hello {}",msg)
}

#[tauri::command]
fn command_prime_factorization(_num :&str) -> String{
    let num = _num.parse().expect("Error: cannot cast num <- num.value");
    let mut prime_fac = PrimeFactorization::new(num);

    prime_fac.get_prime_fac_str()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            command_with_str,
            command_prime_factorization
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct PrimeFactorization{
    num : i32,
    prime_fac_list : Vec::<i32>
}

impl PrimeFactorization{
    fn new(num:i32) -> Self{
        PrimeFactorization { 
            num,
            prime_fac_list: Vec::<i32>::new()
         }
    }

    fn calculate(&mut self){
        const MAX : i32 = std::i32::MAX;
        let mut target_num = self.num;

        for _i in  0..MAX  {
            if target_num % 2 != 0 { break; }
                println!("2 ::: {} / 2 = {}",target_num,target_num / 2);
                target_num /= 2;
                let _ = &self.prime_fac_list.push(2);
            }

            let mut count = 3;
            loop{
                println!("i ::: {} / {} = {}",target_num,count,target_num / count);
                if target_num == 1 { break; }
                if target_num % count != 0 { count += 1; continue; }
                target_num /= count;
                let _ = &self.prime_fac_list.push(count);
            }
    }

    fn get_prime_fac_str(&mut self) -> String {
        let _ = &self.calculate();

        let string_vec: Vec<String> = self.prime_fac_list.iter().map(|&x| x.to_string()).collect();
    
        let result = string_vec.join(", ");
        println!("{}",result);
        result
    }
}
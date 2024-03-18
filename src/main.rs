use database::player_database;
use functions::func_module;
use numbers::number_module;
use prints::print_module;

pub mod database;
pub mod functions;
pub mod numbers;
pub mod prints;

fn main() {
    let arr: [u8; 5] = [0; 5];
    let pass_num: u16 = 34;
    number_module::print_arr(&arr);
    number_module::print_number(&pass_num);

    println!("Enter your name: ");
    let name: String = func_module::read_string();
    print_module::print_name(&name);
    print_module::print_size();

    let player_address: player_database::Addressoffootballer =
        player_database::Addressoffootballer::new(
            "Roman Street".to_string(),
            "Barcelona".to_string(),
            40030,
        );
    player_address.displaylayeraddress();

    let player_data: player_database::Footballer = player_database::Footballer::new(
        "Lionel Messi".to_string(),
        "Argentina".to_string(),
        700,
        1073,
        vec![
            "Champions League".to_string(),
            "La Liga".to_string(),
            "Copa del Rey".to_string(),
        ],
        player_address,
    );
    player_data.displayplayerdata();

    let list: Vec<i32> = vec![32, 450, 500, 600, 201];
    func_module::number_loop(&list);

    println!("Please enter your age: ");
    match func_module::read_number() {
        Err(err) => println!("{err}"),
        Ok(age) => {
            if age == 0 {
                println!("Invalid age!");
            } else {
                println!("Your age is {}", age);
            }
        }
    }
}

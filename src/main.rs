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

    match player_database::Footballer::read_player_data() {
        Ok(player_data) => {
            player_data.displayplayerdata();
            println!("Data was read successfully!");
        }
        Err(err) => println!("There was some error in reading player data - {}", err),
    }

    // let player_address: player_database::Addressoffootballer =
    //     player_database::Addressoffootballer::new(
    //         "Roman Street".to_string(),
    //         "Barcelona".to_string(),
    //         40030,
    //     );
    // player_address.displaylayeraddress();

    // let player_data: player_database::Footballer = player_database::Footballer::new(
    //     "Lionel Messi".to_string(),
    //     "Argentina".to_string(),
    //     700,
    //     1073,
    //     vec![
    //         "Champions League".to_string(),
    //         "La Liga".to_string(),
    //         "Copa del Rey".to_string(),
    //     ],
    //     player_address,
    // );
    // player_data.displayplayerdata();

    // match player_data.write_player_data() {
    //     Ok(_) => println!("File created successfully - player_data.txt"),
    //     Err(err) => println!("There was some error in file creation - {}", err),
    // }

    let list: Vec<i32> = vec![32, 450, 500, 600, 201];
    func_module::number_loop(&list);

    println!("Please enter your age: ");
    match func_module::read_number() {
        Err(err) => {
            println!("{err}");
            panic!("Execution stopped!");
        }
        Ok(age) => {
            if age == 0 {
                panic!("Invalid age entered!");
            } else {
                println!("Your age is {}", age);
            }
        }
    }

    let (area, circumference) = func_module::circle(5.0);
    println!("Area: {:.2}, Circumference: {:.2}", area, circumference);
}

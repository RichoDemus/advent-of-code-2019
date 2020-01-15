
use std::env;

mod one;
mod two;
mod three;
mod four;
mod five;
mod five_part_two;
mod six;
mod intputer;
mod seven;
mod eight;
mod nine;
mod ten;
mod eleven;
mod twelve;
mod read_lines;

fn main() {
    // this is just an ugle hack to not get unused warnings because I'm lazy
    let args: Vec<String> = env::args().collect();
    match args.get(9999) {
        Some(_arg) => {
            one::calc_fuel_req();
            two::bruteforce();
            three::three();
            three::three_part2();
            four::four();
            five::five();
            five_part_two::five();
            six::six();
            seven::seven();
            seven::seven_part2();
            eight::eight();
            eight::part_two();
            nine::nine();
            nine::nine_part2();
            ten::ten();
            ten::ten_part2();
            eleven::eleven();
            twelve::twelve();
        }
        _ => {}
    }

    ten::ten();
}

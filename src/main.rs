use notify_rust::Notification;
use std::env;
use std::io::{stdout, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
fn format_timer(seconds: u64) -> String {
    let mins: String = if seconds / 60 < 10 {
        String::from("0") + (seconds / 60).to_string().as_str()
    } else {
        (seconds / 60).to_string()
    };
    let seconds: String = if seconds % 60 < 10 {
        String::from("0") + (seconds % 60).to_string().as_str()
    } else {
        (seconds % 60).to_string()
    };
    format!("\r{}:{}", mins, seconds)
}
fn main() {
    let mut stdout = stdout();
    let first_arg = match env::args().nth(1).unwrap().as_str().parse::<u64>() {
        Ok(val) => val,
        Err(_) => {
            panic!("please give two arguments with both being numbers\n");
        }
    };
    let twenty_five = Duration::new(60 * first_arg, 0);
    let second_arg = match env::args().nth(2).unwrap().as_str().parse::<u64>() {
        Ok(val) => val,
        Err(_) => panic!("please give two arguments with both being numbers\n"),
    };
    let ten_min = Duration::new(60 * second_arg, 0);
    let mut counter: u64 = 0;
    loop {
        if counter == 0 {
            match Notification::new()
                .summary("timers: Work Time")
                .timeout(notify_rust::Timeout::Milliseconds(1000 * 5))
                .show()
            {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e.to_string());
                    panic!();
                }
            }
        }
        sleep(Duration::new(1, 0));
        counter += 1;
        if twenty_five.as_secs() - counter <= 0 {
            Command::new("ffplay")
                .arg("-t")
                .arg("1")
                .arg("ding.mp3")
                .arg("-nodisp")
                .arg("-autoexit")
                .output()
                .expect("");
            let mut break_counter: u64 = 0;
            if break_counter == 0 {
                match Notification::new()
                    .summary("timers: Break Time")
                    .timeout(notify_rust::Timeout::Milliseconds(1000 * 5))
                    .show()
                {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e.to_string());
                        panic!();
                    }
                }
            }
            loop {
                sleep(Duration::new(1, 0));
                break_counter += 1;
                if ten_min.as_secs() - break_counter <= 0 {
                    Command::new("ffplay")
                        .arg("-t")
                        .arg("1")
                        .arg("punch.mp3")
                        .arg("-nodisp")
                        .arg("-autoexit")
                        .output()
                        .expect("");
                    counter = 0;
                    break;
                }
                let formatted = format_timer(ten_min.as_secs() - break_counter);
                stdout.write_all(formatted.as_bytes());
                stdout.flush();
            }
        }
        let formatted = format_timer(twenty_five.as_secs() - counter);
        stdout.write_all(formatted.as_bytes());
        stdout.flush();
    }
}

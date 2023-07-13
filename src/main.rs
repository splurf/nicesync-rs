mod err;

use std::{
    io::stdin,
    sync::{Arc, Condvar, Mutex},
    thread::{spawn, JoinHandle},
};

fn hehe_thread(pair: Arc<(Mutex<u16>, Condvar)>) -> JoinHandle<err::Result<()>> {
    spawn(move || {
        //  write lock / condition variable
        let (g, cvar) = &*pair;

        let mut started = g.lock()?;

        //  write "hehe" to the terminal (stdout) if `69` was entered
        //  then reset the value back to `0`
        while *started != 69 {
            started = cvar.wait(started)?;

            //  end thread if `420` was entered
            if *started == 420 {
                break;
            }
            //  reset value
            *started = u16::default();

            //  write to stdout
            println!("hehe")
        }
        Ok(())
    })
}

fn user_input(pair: Arc<(Mutex<u16>, Condvar)>) -> err::Result<()> {
    //  write lock / condition variable
    let (g, cvar) = &*pair;

    let stdin = stdin();
    let mut input = String::new();

    while *g.lock()? != 420 {
        //  read line from user input
        stdin.read_line(&mut input)?;

        if let Ok(num) = input.trim_end().parse::<u16>() {
            if num == 69 || num == 420 {
                //  modify the value if an important number has been entered
                *g.lock()? = num;

                //  notify thread that the value has been changed
                cvar.notify_one()
            }
        }
        //  reset input variable
        input.clear()
    }
    Ok(())
}

fn main() -> err::Result<()> {
    //  instantiate write lock and condition variable for synchronization shit
    let pair: Arc<(Mutex<u16>, Condvar)> = Default::default();

    //  instantiate fucking stupid external thread
    let hehe = hehe_thread(pair.clone());

    //  initiate user input
    user_input(pair)?;

    print!("Joining thread...");

    //  wait for the `hehe` thread to finish just to prove that by
    //  entering `420` the thread realizes that the value ([`Mutex<u16>`])
    //  has changed and kills itself lmao
    let result = hehe.join().map_err(|_| err::Error::JoinThread)?;

    println!("Done");
    result
}

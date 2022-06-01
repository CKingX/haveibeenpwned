use crate::error;
use sha1::{Digest, Sha1};

pub fn interactive() {
    loop {
        let password = rpassword::prompt_password("Password to check: ");
        if password.is_err() || password.as_ref().unwrap().is_empty() {
            println!("No password given. Try again");
            continue;
        }

        if let Ok(password) = password {
            let result = hash(password);

            let request = ureq::get(&format!(
                "https://api.pwnedpasswords.com/range/{}",
                &result[..5]
            ))
            .set("Add-Padding", "true")
            .call();

            if request.is_err() {
                error::server_error();
                break;
            }

            let request = request.unwrap().into_string();

            if request.is_err() {
                error::server_error();
                break;
            }
            let request = request.unwrap();
            let request = request.lines().filter(|n| !n.ends_with(":0")).collect::<Vec<_>>();

            match request.iter().find(|n| n.contains(&result[5..])) {
                Some(output) => println!("The password is compromised. It has been seen {} times", output.split(":").nth(1).unwrap()),
                None => println!("Password not compromised"),
            }

            break;
        }
    }
}

fn hash(password: String) -> String {
    let mut hash = Sha1::new();
            hash.update(password);

            let result = hash
                .finalize();
            
            let result = result.as_slice();

            let mut array = [0;16];

            for i in 0..16 {
                array[i] = result[i];
            }

            format!("{:X}", u128::from_be_bytes(array))
}
use sha1::{Digest, Sha1};

pub enum PasswordWithUsage {
    SafePassword,
    CompromisedPassword(usize),
}

pub enum Password {
    SafePassword,
    CompromisedPassword,
}

pub struct ServerError;

impl From<ureq::Error> for ServerError {
    fn from(_: ureq::Error) -> Self {
        ServerError
    }
}

impl From<std::io::Error> for ServerError {
    fn from(_: std::io::Error) -> Self {
        ServerError
    }
}

fn hash(password: &str) -> String {
    let mut hash = Sha1::new();
    hash.update(password);

    let result = hash.finalize();

    let result = result.as_slice();

    let mut array = [0; 16];

    array[..16].copy_from_slice(&result[..16]);

    format!("{:X}", u128::from_be_bytes(array))
}

pub fn check_password_online(password: String) -> Result<PasswordWithUsage, ()> {
    let result = hash(&password);

    let request = ureq::get(&format!(
        "https://api.pwnedpasswords.com/range/{}",
        &result[..5]
    ))
    .set("Add-Padding", "true")
    .call();

    if request.is_err() {
        return Err(());
    }

    let request = request.unwrap().into_string();

    if request.is_err() {
        return Err(());
    }

    let request = request.unwrap();
    let mut request = request.lines().filter(strip_padding);

    match request.find(|n| n.contains(&result[5..])) {
        Some(output) => Ok(PasswordWithUsage::CompromisedPassword(
            output.split(':').nth(1).unwrap().parse::<usize>().unwrap(),
        )),
        None => Ok(PasswordWithUsage::SafePassword),
    }
}

pub fn download_range(range: u64) -> Result<String, ServerError> {
    let range = format!("{:05X}", range);
    let request = ureq::get(&format!("https://api.pwnedpasswords.com/range/{range}"))
        .set("Add-Padding", "true")
        .call()?
        .into_string()?
        .lines()
        .filter(strip_padding)
        .map(|n| format!("{range}{n}"))
        .fold("".to_string(), |mut a, b| {
            a.push_str(&b);
            a.push('\n');
            a
        });

    Ok(request)
}

pub fn strip_padding(line: &&str) -> bool {
    !line.ends_with(":0")
}

pub fn remove_usage(line: &str) -> String {
    line.split(':').next().unwrap().to_string()
}

pub fn get_password() -> String {
    let password = loop {
        let password = rpassword::prompt_password("Password to check: ");
        if password.is_err() || password.as_ref().unwrap().is_empty() {
            println!("No password given. Try again");
            continue;
        }
        break password.unwrap();
    };
    password
}

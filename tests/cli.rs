use assert_cmd::cargo::*;

#[test]
fn test_default_length() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("pswd");

    cmd.assert()
        .success()
        .stdout(predicates::str::is_match(r"^.{32}\n$").unwrap());

    Ok(())
}

#[test]
fn test_custom_length() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("pswd");

    cmd.arg("12");
    cmd.assert()
        .success()
        .stdout(predicates::str::is_match(r"^.{12}\n$").unwrap());

    Ok(())
}

#[test]
fn test_exclude_special() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("pswd");

    cmd.arg("--exclude-special");
    cmd.assert()
        .success()
        .stdout(predicates::str::is_match(r"^[A-Za-z0-9]{32}\n$").unwrap());

    Ok(())
}

#[test]
fn test_custom_length_exclude_special() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("pswd");
    cmd.args(["16", "--exclude-special"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::is_match(r"^[A-Za-z0-9]{16}\n$").unwrap());
    Ok(())
}

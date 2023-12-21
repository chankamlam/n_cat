use std::process::{Command,Stdio};

use assert_cmd::{Command as ACMD, output::OutputOkExt};

use assert_cmd::prelude::*;
// use predicates::prelude::*;

#[test]
fn run_without_number_and_number_nonblank(){
    let org_res = Command::new("cat").arg("tests/data1.txt").output().unwrap();
    let new_res = ACMD::cargo_bin("n_cat").unwrap().arg("tests/data1.txt").output().unwrap();

    assert_eq!(String::from_utf8(org_res.stdout).unwrap(),String::from_utf8(new_res.stdout).unwrap())

}

#[test]
fn run_with_number_only(){
    let org_res = Command::new("cat").arg("-n").arg("tests/data1.txt").output().unwrap();
    let new_res = ACMD::cargo_bin("n_cat").unwrap().arg("-n").arg("tests/data1.txt").output().unwrap();

    assert_eq!(String::from_utf8(org_res.stdout).unwrap(),String::from_utf8(new_res.stdout).unwrap())
}

#[test]
fn run_with_number_nonblank_only(){
    let org_res = Command::new("cat").arg("-b").arg("tests/data1.txt").output().unwrap();
    let new_res = ACMD::cargo_bin("n_cat").unwrap().arg("-b").arg("tests/data1.txt").output().unwrap();

    assert_eq!(String::from_utf8(org_res.stdout).unwrap(),String::from_utf8(new_res.stdout).unwrap())
}

#[test]
fn run_without_number_and_number_nonblank_multiple_files(){
    let org_res = Command::new("cat").arg("tests/data2.txt").arg("tests/data1.txt").output().unwrap();
    let new_res = ACMD::cargo_bin("n_cat").unwrap().arg("tests/data2.txt").arg("tests/data1.txt").output().unwrap();

    println!("{}",String::from_utf8(org_res.stdout.clone()).unwrap());
    println!("{}",String::from_utf8(new_res.stdout.clone()).unwrap());
    assert_eq!(String::from_utf8(org_res.stdout).unwrap(),String::from_utf8(new_res.stdout).unwrap())
}
#[test]
fn run_with_number_multiple_files(){
    let org_res = Command::new("cat").arg("-n").arg("tests/data2.txt").arg("tests/data1.txt").output().unwrap();
    let new_res = ACMD::cargo_bin("n_cat").unwrap().arg("-n").arg("tests/data2.txt").arg("tests/data1.txt").output().unwrap();
    println!("{}",String::from_utf8(org_res.stdout.clone()).unwrap());
    println!("{}",String::from_utf8(new_res.stdout.clone()).unwrap());
    assert_eq!(String::from_utf8(org_res.stdout.clone()).unwrap(),String::from_utf8(new_res.stdout).unwrap())
}
#[test]
fn run_with_number_nonblank_multiple_files(){
    let org_res = Command::new("cat").arg("-b").arg("tests/data2.txt").arg("tests/data1.txt").output().unwrap();
    let new_res = ACMD::cargo_bin("n_cat").unwrap().arg("-b").arg("tests/data2.txt").arg("tests/data1.txt").output().unwrap();
    println!("{}",String::from_utf8(org_res.stdout.clone()).unwrap());
    println!("{}",String::from_utf8(new_res.stdout.clone()).unwrap());

    assert_eq!(String::from_utf8(org_res.stdout).unwrap(),String::from_utf8(new_res.stdout).unwrap())
}

#[test]
fn run_with_number_nonblank_multiple_files_combine_cmd(){
    let org_cat_res = Command::new("cat").arg("tests/data2.txt").arg("tests/data1.txt").stdout(Stdio::piped()).spawn().unwrap();
    let org_head_res = Command::new("head").arg("-n").arg("2").stdin(Stdio::from(org_cat_res.stdout.unwrap())).unwrap();
    let new_cat_res = ACMD::cargo_bin("n_cat").unwrap().arg("tests/data2.txt").arg("tests/data1.txt").output().unwrap();
    let new_head_res = ACMD::new("head").arg("-n").arg("2").write_stdin(new_cat_res.stdout).unwrap();
    println!("{}",String::from_utf8(org_head_res.stdout.clone()).unwrap());
    println!("{}",String::from_utf8(new_head_res.stdout.clone()).unwrap());

    assert_eq!(String::from_utf8(org_head_res.stdout).unwrap(),String::from_utf8(new_head_res.stdout).unwrap())
}
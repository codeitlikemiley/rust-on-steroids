use std::env;
use std::path::Path;

fn main() {
    cornucopia();
}

fn cornucopia() {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = "queries";
    dbg!(&queries_path);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    dbg!(&out_dir);
    let file_path = Path::new(&out_dir).join("cornucopia.rs");
    dbg!(&file_path);
    let db_url = match env::var_os("DATABASE_URL") {
        Some(url) => url,
        None => "postgresql://postgres:@127.0.0.1:5432/nails?sslmode=disable".to_string().into(),
    };
    dbg!(&db_url);


    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    // Call cornucopia. Use whatever CLI command you need.
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    dbg!(&output);

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}
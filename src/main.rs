use reqwest::blocking::get;

fn main() -> Result<(), reqwest::Error> {
    let url = "http://api.open-notify.org/astros.json";
    let resq: serde_json::Value = get(url)?
                .json()?;
    let n = &resq["number"];
    let n = n.as_u64().unwrap_or_default();
    println!("{n}");
    Ok(())
}
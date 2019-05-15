use crate::Result;

pub fn resource_id() -> Result<String> {
    get_text("instance-id")
}

pub fn ami_id() -> Result<String> {
    get_text("ami-id")
}

pub fn instance_type() -> Result<String> {
    get_text("instance-type")
}

fn get_url(info: &str) -> String {
    return format!("http://169.254.169.254/latest/meta-data/{}", info);
}

fn get_text(info: &str) -> Result<String> {

    let mut resp = reqwest::get(get_url(info).as_str())?;
    let text = resp.text()?;
    return Ok(text);
}
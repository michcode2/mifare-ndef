use std::process::Command;
fn main() {
    let url = "http://images.michcode.space/shrimple.gif";
    let url_bytes = url.as_bytes();
    let url_len = url_bytes.len() + 1;
    
    let header= [0xd1, 0x01];
    let payload_type = [0x55, 0x00];

    let mut without_mifare_simple = Vec::from(header);
    without_mifare_simple.push(url_len as u8);
    payload_type.iter().for_each(|&byte| without_mifare_simple.push(byte));
    url_bytes.iter().for_each(|&byte| without_mifare_simple.push(byte));

    let tlv_prefix: [u8; 2] = [0x03,  without_mifare_simple.len() as u8 ];

    let full = format!("{}{}{}", hex::encode(&tlv_prefix), hex::encode(without_mifare_simple), "fe");
    println!("{}\n{}", full, url_len);

    let output = Command::new("../../proxmark3/pm3")
        .args(["-c",&format!("hf mf ndefwrite -d {}", full)])
        .output().expect("didnt run right");
    output.stdout.iter().for_each(|&a| print!("{}", a as char));
    println!(" --- written --- ");
    
    let output = Command::new("../../proxmark3/pm3")
        .args(["-c", "hf mf ndefread"])
        .output().expect("didnt run right");
    output.stdout.iter().for_each(|&a| print!("{}", a as char));
}
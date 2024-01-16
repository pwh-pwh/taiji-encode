const BASE64_TABLE: [u8; 65] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

const TAIJI_TABLE: [char; 65] = [
    '䷁', '䷗', '䷆', '䷒', '䷎', '䷣', '䷭', '䷊',
    '䷏', '䷲', '䷧', '䷵', '䷽', '䷶', '䷟', '䷡',
    '䷇', '䷂', '䷜', '䷻', '䷦', '䷾', '䷯', '䷄',
    '䷬', '䷐', '䷮', '䷹', '䷞', '䷰', '䷛', '䷪',
    '䷖', '䷚', '䷃', '䷨', '䷳', '䷕', '䷑', '䷙',
    '䷢', '䷔', '䷿', '䷥', '䷷', '䷝', '䷱', '䷍',
    '䷓', '䷩', '䷺', '䷼', '䷴', '䷤', '䷸', '䷈',
    '䷋', '䷘', '䷅', '䷉', '䷠', '䷌', '䷫', '䷀', '☯'
];

fn find_table_index<const U: usize, T>(table: [T; U], key: T) -> Option<usize>
    where T: Eq + PartialEq
{
    for x in table.iter().enumerate() {
        if *x.1 == key {
            return Some(x.0);
        }
    }
    None
}

pub fn base64_encode<T: AsRef<[u8]>>(str: T) -> String {
    let mut encoded = Vec::new();
    let mut padding = Vec::new();
    str.as_ref().chunks(3).for_each(
        |chunk| {
            let buf = match chunk.len() {
                3 => [chunk[0], chunk[1], chunk[2]],
                2 => [chunk[0], chunk[1], 0],
                1 => [chunk[0], 0, 0],
                _ => [0, 0, 0],
            };
            let n = (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32);
            encoded.push(BASE64_TABLE[(n >> 18) as usize]);
            encoded.push(BASE64_TABLE[((n >> 12) & 0x3F) as usize]);
            encoded.push(BASE64_TABLE[((n >> 6) & 0x3F) as usize]);
            encoded.push(BASE64_TABLE[(n & 0x3F) as usize]);
            let pad = 3 - chunk.len();
            if pad > 0 {
                padding.extend(vec![b'='; pad]);
            }
        }
    );
    encoded.truncate(encoded.len() - padding.len());
    encoded.extend(padding);
    String::from_utf8(encoded).unwrap()
}

pub fn base64_decode(taiji: &str) -> Result<String, &'static str> {
    let mut bytes = Vec::new();
    if taiji.as_bytes().len() % 4 != 0 {
        return Err("Input length is not a multiple of 4");
    }
    let input = taiji.trim_end_matches('=');
    for chunk in input.as_bytes().chunks(4) {
        let indices: Vec<_> = chunk
            .iter()
            .map(|&c| BASE64_TABLE.iter().position(|&val| val == c))
            .collect();
        if indices.contains(&None) {
            return Err("Input contains invalid Base64 characters");
        }
        let n = (indices[0].unwrap() << 18)
            + (indices[1].unwrap() << 12)
            + (indices.get(2).unwrap_or(&Some(0)).unwrap() << 6)
            + indices.get(3).unwrap_or(&Some(0)).unwrap();
        bytes.push((n >> 16) as u8);
        if chunk.len() == 4 && chunk[2] != b'=' {
            bytes.push((n >> 8 & 0xFF) as u8);
        }
        if chunk.len() == 4 && chunk[3] != b'=' {
            bytes.push((n & 0xFF) as u8);
        }
    }
    String::from_utf8(bytes).map_err(|_| "tran utf8 error")
}

pub fn taiji_encode<T: AsRef<[u8]>>(str: T) -> String {
    base64_encode(str)
        .chars().map(|c| {
        let index = find_table_index(BASE64_TABLE, c as u8).unwrap();
        TAIJI_TABLE[index]
    }).collect()
}

pub fn taiji_decode(taiji: &str) -> Result<String, &'static str> {
    let str = taiji.chars().map(|c| {
        let index = find_table_index(TAIJI_TABLE, c).unwrap();
        BASE64_TABLE[index]
    }).collect::<Vec<u8>>();
    base64_decode(&String::from_utf8(str).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = "你好世界!";
        let result = taiji_encode(str);
        assert_eq!(result,"䷘䷵䷸䷖䷘䷮䷯䷌䷘䷵䷃䷯䷘䷘䷯䷽䷏䷇☯☯");
        let de_result = taiji_decode(&result);
        assert_eq!(de_result,Ok::<_,&str>("你好世界!".to_string()))
    }
}

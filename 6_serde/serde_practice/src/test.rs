#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use serde_json;
    use crate::Request;

    #[test]
    fn test() {
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();
        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.public_tariff.price, 110);
        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].description, "Gift 1");
    }
}
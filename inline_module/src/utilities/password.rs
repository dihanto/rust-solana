    
        pub fn hash(text: &str) -> String {
            let result = bcrypt::hash(text, bcrypt::DEFAULT_COST).unwrap();
            return result
        }
        pub fn is_valid(plain: &str, hashed: &str) -> bool {
            let valid = bcrypt::verify(plain, hashed).unwrap();
            return  valid;
        }

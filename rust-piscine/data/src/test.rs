#[cfg(test)]

mod tests {

    // changes.rs
    use crate::changes::*;
    #[test]
    fn test_changes() {
        test_non_existing_alias();
        test_alias();

        fn test_non_existing_alias() {
            let mut lights = Vec::new();

            for i in 0..5 {
                let alias = format!("light_{}", i);
                lights.push(Light::new(&alias));
            }

            let mut copy = lights.clone();
            change_brightness(&mut copy, "light_6", 99);
            assert_eq!(copy, lights);
        }
        fn test_alias() {
            let mut lights = Vec::new();

            for i in 0..5 {
                let alias = format!("light_{}", i);
                lights.push(Light::new(&alias));
            }

            let alias = "light_3";
            change_brightness(&mut lights, alias, 99);
            assert_eq!(lights[3].brightness, 99);
        }
    }
}

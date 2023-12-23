#[cfg(test)]

mod tests {
    use crate::student::{first_name, id, last_name, Student};
    #[test]
    fn test_student() {
        let mut student = Student(1, "Jordan".to_string(), "Peterson".to_string());
        assert_eq!(id(&mut student), 1);
        assert_eq!(first_name(&student), "Jordan");
        assert_eq!(last_name(&student), "Peterson");
        assert_eq!(
            format!("{:?}", student),
            "Student(1, \"Jordan\", \"Peterson\")"
        );
    }
}

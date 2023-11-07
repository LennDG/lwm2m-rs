use crate::err::ObjectParserError;

#[derive(Debug, Clone)]
pub struct ObjectLink {
    pub link: String,
    pub object_id: u16,
    pub object_instance: u16,
}

impl TryFrom<String> for ObjectLink {
    type Error = ObjectParserError;

    fn try_from(link: String) -> Result<Self, Self::Error> {
        let mut object_link = ObjectLink {
            link: link.clone(),
            object_id: 0,
            object_instance: 0,
        };
        let split: Vec<&str> = link.split(':').collect();

        if split.len() > 2 {
            return Err(ObjectParserError::new(
                "Object Link should match u16:u16 pattern",
            ));
        }

        object_link.object_id = parse_id(0, split[0])?;
        object_link.object_instance = parse_id(1, split[1])?;

        Ok(object_link)
    }
}

fn parse_id(index: usize, id: &str) -> Result<u16, ObjectParserError> {
    id.parse().map_err(|err| {
        ObjectParserError::new(
            format!("Object Link index {}, value {} is not a u16", index, id,).as_str(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_object_link() {
        let link = "123:456".to_string();
        let object_link = ObjectLink::try_from(link);
        assert!(object_link.is_ok());
        if let Ok(link) = object_link {
            assert_eq!(link.object_id, 123);
            assert_eq!(link.object_instance, 456);
        }
    }

    #[test]
    fn test_invalid_object_link_format() {
        let link = "123:456:789".to_string();
        let object_link = ObjectLink::try_from(link);
        assert!(object_link.is_err());
        if let Err(e) = object_link {
            assert_eq!(e.to_string(), "Object Link should match u16:u16 pattern");
        }
    }

    #[test]
    fn test_invalid_object_link_values() {
        let link = "abc:def".to_string();
        let object_link = ObjectLink::try_from(link);
        assert!(object_link.is_err());
        if let Err(e) = object_link {
            assert_eq!(e.to_string(), "Object Link index 0, value abc is not a u16");
        }
    }
}

use crate::err::ObjectParserError;

#[derive(Debug, Clone)]
pub struct CoreLink {
    pub link: String,
    pub object_id: u16,
    pub object_instance: Option<u16>,
    pub resource_id: Option<u16>,
    pub resource_instance: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelType {
    Object,
    Resource,
}

// CoRELink looks like </1/0/0>
impl TryFrom<&str> for CoreLink {
    type Error = ObjectParserError;

    fn try_from(link: &str) -> Result<Self, Self::Error> {
        use regex::Regex;
        let re = Regex::new(r"^<(\/[\d]+){1,4}>$").unwrap();
        if !re.is_match(link) {
            Err(ObjectParserError::new("LwM2M CoRE link is not valid"))?;
        }

        let mut core_link = CoreLink {
            link: link.to_owned(),
            object_id: 0,
            object_instance: None,
            resource_id: None,
            resource_instance: None,
        };

        for (index, id) in link.clone().replace(['<', '>'], "")[1..]
            .split('/')
            .enumerate()
        {
            match index {
                0 => parse_id(index, id).map(|value| core_link.object_id = value),
                1 => parse_id(index, id).map(|value| core_link.object_instance = Some(value)),
                2 => parse_id(index, id).map(|value| core_link.resource_id = Some(value)),
                3 => parse_id(index, id).map(|value| core_link.resource_instance = Some(value)),
                _ => Err(ObjectParserError::new(
                    "LwM2M CoRE link can not have more than 4 elements",
                )),
            }?;
        }

        Ok(core_link)
    }
}

fn parse_id(index: usize, id: &str) -> Result<u16, ObjectParserError> {
    id.parse().map_err(|err| {
        ObjectParserError::new(&format!(
            "CoRE link index {}, value {} is not a u16",
            index, id,
        ))
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_valid_string() {
        let core_link = CoreLink::try_from("</3/1/3/0>");
        assert!(core_link.is_ok());
        if let Ok(core_link) = core_link {
            assert_eq!(core_link.object_id, 3);
            assert_eq!(core_link.object_instance, Some(1));
            assert_eq!(core_link.resource_id, Some(3));
            assert_eq!(core_link.resource_instance, Some(0));
        }
    }

    #[test]
    fn test_try_from_valid_string2() {
        let core_link = CoreLink::try_from("</3/1>");
        assert!(core_link.is_ok());
        if let Ok(core_link) = core_link {
            assert_eq!(core_link.object_id, 3);
            assert_eq!(core_link.object_instance, Some(1));
            assert_eq!(core_link.resource_id, None);
            assert_eq!(core_link.resource_instance, None);
        }
    }

    #[test]
    fn test_try_from_valid_string3() {
        let core_link = CoreLink::try_from("</3>");
        assert!(core_link.is_ok());
        if let Ok(core_link) = core_link {
            assert_eq!(core_link.object_id, 3);
            assert_eq!(core_link.object_instance, None);
            assert_eq!(core_link.resource_id, None);
            assert_eq!(core_link.resource_instance, None);
        }
    }

    #[test]
    fn test_try_from_invalid_string() {
        let core_link = CoreLink::try_from("</a/2/b>");
        assert!(core_link.is_err());
    }

    #[test]
    fn test_try_from_invalid_string2() {
        let core_link = CoreLink::try_from("hello");
        assert!(core_link.is_err());
    }

    #[test]
    fn test_try_from_too_many_elements() {
        let core_link = CoreLink::try_from("</1/2/3/4/5>");
        assert!(core_link.is_err());
    }
}

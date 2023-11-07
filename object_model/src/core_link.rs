use crate::err::ObjectParserError;

#[derive(Debug, Clone)]
pub struct CoreLink {
    pub link: String,
    pub object_id: u16,
    pub object_instance: Option<u16>,
    pub resource_id: Option<u16>,
    pub resource_instance: Option<u16>,
    pub model_type: ModelType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelType {
    Object,
    Resource,
}

impl TryFrom<String> for CoreLink {
    type Error = ObjectParserError;

    fn try_from(link: String) -> Result<Self, Self::Error> {
        let mut core_link = CoreLink {
            link: link.clone(),
            object_id: 0,
            object_instance: None,
            resource_id: None,
            resource_instance: None,
            model_type: ModelType::Object,
        };
        for (index, id) in link.clone().split('/').enumerate() {
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

        if core_link.resource_id.is_some() {
            core_link.model_type = ModelType::Resource
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
        let link = "3/1/3/0".to_string();
        let core_link = CoreLink::try_from(link);
        assert!(core_link.is_ok());
        if let Ok(core_link) = core_link {
            assert_eq!(core_link.object_id, 3);
            assert_eq!(core_link.object_instance, Some(1));
            assert_eq!(core_link.resource_id, Some(3));
            assert_eq!(core_link.resource_instance, Some(0));
            assert_eq!(core_link.model_type, ModelType::Resource);
        }
    }

    #[test]
    fn test_try_from_valid_string2() {
        let link = "3/1".to_string();
        let core_link = CoreLink::try_from(link);
        assert!(core_link.is_ok());
        if let Ok(core_link) = core_link {
            assert_eq!(core_link.object_id, 3);
            assert_eq!(core_link.object_instance, Some(1));
            assert_eq!(core_link.resource_id, None);
            assert_eq!(core_link.resource_instance, None);
            assert_eq!(core_link.model_type, ModelType::Object);
        }
    }

    #[test]
    fn test_try_from_invalid_string() {
        let link = "a/2/b".to_string();
        let core_link = CoreLink::try_from(link);
        assert!(core_link.is_err());
    }

    #[test]
    fn test_try_from_invalid_string2() {
        let link = "hello".to_string();
        let core_link = CoreLink::try_from(link);
        assert!(core_link.is_err());
    }

    #[test]
    fn test_try_from_too_many_elements() {
        let link = "1/2/3/4/5".to_string();
        let core_link = CoreLink::try_from(link);
        assert!(core_link.is_err());
    }
}

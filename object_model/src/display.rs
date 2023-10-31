use std::fmt;

impl fmt::Display for crate::ObjectModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self.description.clone() {
            None => "".to_string(),
            Some(v) => v,
        };

        let mandatory = match self.mandatory {
            true => "Mandatory",
            false => "Optional",
        };

        let multiple = match self.multiple {
            true => "Multiple",
            false => "Single",
        };

        let description2 = match self.description2.clone() {
            None => "".to_string(),
            Some(v) => v,
        };

        let object_version = self.version.clone();

        let lwm2m_version = self.version.clone();

        let mut resources = "".to_string();
        for (key, value) in self.resources.clone() {
            resources.push_str(add_tab_to_lines(format!("{}\n", value)).as_str())
        }

        write!(
            f,
            "ID: {}\n\
            Name: {}\n\
            URN: {}\n\
            Description: {}\n\
            Mandatory: {}\n\
            Multiple: {}\n\
            Description 2: {}\n\
            Object Version: {}\n\
            LWM2M Version: {}\n\
            Resources:\n{}\n\
            ",
            self.id,
            self.name,
            self.urn,
            description,
            mandatory,
            multiple,
            description2,
            object_version,
            lwm2m_version,
            resources
        )
    }
}

impl fmt::Display for crate::ResourceModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self.description.clone() {
            None => "".to_string(),
            Some(v) => v,
        };

        let mandatory = match self.mandatory {
            true => "Mandatory",
            false => "Optional",
        };

        let multiple = match self.multiple {
            true => "Multiple",
            false => "Single",
        };

        let resourcetype = match self.resourcetype.clone() {
            None => "".to_string(),
            Some(v) => v.to_string(),
        };

        let resourcerange = match self.range.clone() {
            None => "".to_string(),
            Some(v) => v.to_string(),
        };

        let units = match self.units.clone() {
            None => "".to_string(),
            Some(v) => v,
        };

        let operations = match self.operations {
            None => "".to_string(),
            Some(v) => v.to_string(),
        };

        write!(
            f,
            "ID: {}\n\
            Name: {}\n\
            Description: {}\n\
            Mandatory: {}\n\
            Multiple: {}\n\
            Resource Type: {}\n\
            Resource Range: {}\n\
            Units: {}\n\
            Operations: {}",
            self.id,
            self.name,
            description,
            mandatory,
            multiple,
            resourcetype,
            resourcerange,
            units,
            operations
        )
    }
}

impl fmt::Display for crate::ResourceOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for crate::ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for crate::ResourceRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for crate::Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.oma_version)
    }
}

fn add_tab_to_lines(input: String) -> String {
    let mut result = String::with_capacity(input.len());
    let mut start_of_line = true;

    for c in input.chars() {
        if start_of_line {
            result.push('\t');
            start_of_line = false;
        }

        result.push(c);

        if c == '\n' {
            start_of_line = true;
        }
    }

    result
}

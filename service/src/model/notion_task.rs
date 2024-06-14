use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Results {
    pub results: Vec<Page>,
}

impl Results {
    
    pub fn deserialize(input: &str) -> Results {
        serde_json::from_str(input).unwrap()
    }
    
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Page {
    pub properties: Task,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Task {
    #[serde(rename = "Name")]
    name: NameType,
    #[serde(rename = "Created")]
    created: CreatedTimeType,
    #[serde(rename = "Status")]
    status: StatusType,
}

impl Task {
    fn get_name(&self) -> Option<String> {
        let title = Title::default();
        let some = self.name.title.iter().next().unwrap_or(&title);
        some.name.clone()
    }

    fn get_created_time(&self) -> String {
        self.created.created_time.clone()
    }

    fn get_status(&self) -> String {
        self.status.status.name.clone()
    }

    pub fn to_task(&self) -> crate::model::task::Task {
        crate::model::task::Task {
            name: self.get_name(),
            created: self.get_created_time(),
            status: self.get_status(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NameType {
    title: Vec<Title>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Title {
    #[serde(rename = "plain_text")]
    name: Option<String>,
}

impl Title {
    fn default() -> Title {
        Title {
            name: None,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CreatedTimeType {
    created_time: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct StatusType {
    status: Status,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Status {
    pub name: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_JSON: &str = r#"
        {
            "Created": {
              "id": "u%3Cui",
              "type": "created_time",
              "created_time": "2024-04-26T08:38:00.000Z"
            },
            "Name": {
              "id": "title",
              "type": "title",
              "title": [
                {
                  "type": "text",
                  "text": {
                    "content": "Go to Abuelo",
                    "link": null
                  },
                  "annotations": {
                    "bold": false,
                    "italic": false,
                    "strikethrough": false,
                    "underline": false,
                    "code": false,
                    "color": "default"
                  },
                  "plain_text": "Go to Abuelo",
                  "href": null
                }
              ]
            },
            "Status": {
              "id": "UdxN",
              "type": "status",
              "status": {
                "id": "fcaba1ac-baa6-43d0-b3d7-2451ad118a26",
                "name": "Not started",
                "color": "default"
              }
            }
        }"#;

    #[test]
    fn name_deserialization() {
        let a: Task = serde_json::from_str(EXAMPLE_JSON).unwrap();

        let expected = "Go to Abuelo".to_string();

        assert_eq!(a.get_name().unwrap(), expected)
    }

    #[test]
    fn created_time_deserialization() {
        let a: Task = serde_json::from_str(EXAMPLE_JSON).unwrap();

        let expected = "2024-04-26T08:38:00.000Z".to_string();

        assert_eq!(a.get_created_time(), expected)
    }

    fn status_deserialization() {
        let a: Task = serde_json::from_str(EXAMPLE_JSON).unwrap();

        let expected = "Not started".to_string();

        assert_eq!(a.get_created_time(), expected)
    }
}
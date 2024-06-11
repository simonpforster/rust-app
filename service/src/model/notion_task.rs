
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Task {
    Name: Name,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Name {
    r#type: String,
    title: Vec<Title>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Title {
    r#type: String,
    text: Text,
    annotations: Annotations,
    plain_text: Option<String>,
    href: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Text {
    content: String,
    link: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Annotations {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
    code: bool,
    color: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_deserialization() {
        let name_json = r#"
        {
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
            }
        }"#;

        let a: Task = serde_json::from_str(name_json).unwrap();


        let expected_name = "Go to Abuelo".to_string();


        assert_eq!(a.Name.title.iter().next().unwrap().plain_text.as_ref().unwrap(), &expected_name)
    }

}
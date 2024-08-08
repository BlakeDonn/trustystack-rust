pub struct Software {
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[juniper::graphql_object]
impl Software {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn image_url(&self) -> &str {
        &self.image_url
    }
}

pub fn get_softwares() -> Vec<Software> {
    vec![
        Software {
            name: "PIX4D".to_string(),
            description: "Description for PIX4D".to_string(),
            image_url: "software/pix4d.png".to_string(),
        },
        Software {
            name: "Autodesk".to_string(),
            description: "Description for Autodesk".to_string(),
            image_url: "software/autodesk.png".to_string(),
        },
        Software {
            name: "RenderMan".to_string(),
            description: "Description for RenderMan".to_string(),
            image_url: "software/renderman.png".to_string(),
        },
        Software {
            name: "Blender".to_string(),
            description: "Description for Blender".to_string(),
            image_url: "software/blender.png".to_string(),
        },
    ]
}

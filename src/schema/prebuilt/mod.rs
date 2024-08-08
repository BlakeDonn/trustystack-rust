use juniper::GraphQLObject;

pub mod spec;

use spec::Spec;

#[derive(GraphQLObject)]
pub struct Prebuilt {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub specifications: Vec<Spec>,
}

pub fn get_prebuilts() -> Vec<Prebuilt> {
    vec![
        Prebuilt {
            name: "Stealth".to_string(),
            description: "Intel Core i3".to_string(),
            image_url: "prebuilts/stealth.png".to_string(),
            specifications: vec![
                Spec {
                    key: "CPU".to_string(),
                    value: "Intel Core i3".to_string(),
                },
                Spec {
                    key: "RAM".to_string(),
                    value: "16GB DDR4 RAM".to_string(),
                },
                Spec {
                    key: "Storage".to_string(),
                    value: "1TB M.2 SSD".to_string(),
                },
                Spec {
                    key: "GPU".to_string(),
                    value: "NVIDIA RTX 4060 8GB Graphics Card".to_string(),
                },
                Spec {
                    key: "PSU".to_string(),
                    value: "750W Certified Power Supply".to_string(),
                },
                Spec {
                    key: "OS".to_string(),
                    value: "Windows 11".to_string(),
                },
            ],
        },
        Prebuilt {
            name: "Darkwake".to_string(),
            description: "Intel Core i5".to_string(),
            image_url: "prebuilts/darkwake.png".to_string(),
            specifications: vec![
                Spec {
                    key: "CPU".to_string(),
                    value: "Intel Core i5".to_string(),
                },
                Spec {
                    key: "RAM".to_string(),
                    value: "32GB DRR5 RAM".to_string(),
                },
                Spec {
                    key: "Storage".to_string(),
                    value: "2TB M.2 SSD".to_string(),
                },
                Spec {
                    key: "GPU".to_string(),
                    value: "NVIDIA RTX 4070 SUPER 12GB Graphics Card".to_string(),
                },
                Spec {
                    key: "PSU".to_string(),
                    value: "850W Certified Power Supply".to_string(),
                },
                Spec {
                    key: "OS".to_string(),
                    value: "Windows 11".to_string(),
                },
            ],
        },
        Prebuilt {
            name: "Shadowblade".to_string(),
            description: "Intel Core i7".to_string(),
            image_url: "prebuilts/shadowblade.png".to_string(),
            specifications: vec![
                Spec {
                    key: "CPU".to_string(),
                    value: "Intel Core i7".to_string(),
                },
                Spec {
                    key: "RAM".to_string(),
                    value: "64GB DRR5 RAM".to_string(),
                },
                Spec {
                    key: "Storage".to_string(),
                    value: "4TB M.2 SSD".to_string(),
                },
                Spec {
                    key: "GPU".to_string(),
                    value: "NVIDIA RTX 4080 SUPER 16GB Graphics Card".to_string(),
                },
                Spec {
                    key: "PSU".to_string(),
                    value: "1000W Certified Power Supply".to_string(),
                },
                Spec {
                    key: "OS".to_string(),
                    value: "Windows 11".to_string(),
                },
            ],
        },
    ]
}

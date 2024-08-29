/// Represents a service offered, used in GraphQL queries.
pub struct Service {
    pub name: String,
    pub description: String,
    pub image_url: String,
}

/// Implements GraphQL queries for the `Service` struct.
#[juniper::graphql_object]
impl Service {
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

/// Provides the queries related to services for GraphQL.
pub struct ServiceQuery;

impl ServiceQuery {
    /// Returns a list of services as a vector of `Service` objects.
    pub fn get_services() -> Vec<Service> {
        vec![
        Service {
            name: "Workstations for Creators".to_string(),
            description: "Take your creative projects to new heights with meticulously designed workstations,  to support leading software like Maya, Redshift, Autodesk, Adobe Premiere, and Vegas. Coastal Configurations specializes in creating advanced computing solutions that cater specifically to the demands of high-end rendering and editing software. Each workstation is built to offer a seamless, quiet computing experience, ensuring reliability and top-tier performance, allowing you to focus on crafting stunning visuals and animations with your preferred software suite.".to_string(),
            image_url: "services/workstations.jpg".to_string(),
        },
        Service {
            name: "3D Scanning Workstations".to_string(),
            description: "Enhance your remote sensing tasks with a bespoke 3D Scanning Workstation from Coastal Configurations, crafted with your specific software requirements in mind. Coastal Configurations presents computing solutions dedicated to laser scanning, photogrammetry, and point cloud management, designed to streamline your workflow and optimize your focus on project execution.".to_string(),
            image_url: "services/scanning.jpg".to_string(),
        },
        Service {
            name: "Commercial Computing Solutions".to_string(),
            description: "No matter your the size of your project, type of hardware, or computing power needed, Coastal has got you covered when it comes to your commercial productivity needs. Sourcing the most up to date systems, peripherals, audio video, and collaboration technologies, there is not a solution we cannot solve for. Learn how you can enhance your workplace productivity with business solutions from Coastal Configurations.".to_string(),
            image_url: "services/commercial.jpg".to_string(),
        },
        Service {
            name: "Gaming Systems".to_string(),
            description: "Coastal Configurations Gaming PCs are engineered to deliver unparalleled performance for all your favorite titles. Whether you're battling in the latest AAA games, exploring vast open worlds, or competing in high-stakes eSports, our gaming PCs are built to exceed expectations. Combining cutting-edge design with silent, reliable, and powerful components, each system is carefully assembled by our expert technicians in the USA, tailored to ensure you stay ahead of the game with smooth, lag-free gameplay and breathtaking visuals.".to_string(),
            image_url: "services/gaming.jpg".to_string(),
        },
    ]
    }
}


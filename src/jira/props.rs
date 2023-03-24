pub mod props {
    pub struct PropsService();

    impl PropsService {
        //GET /rest/api/2/application-properties
        pub async fn get_prop(&self, url: String, token: String) {

        }

        //PUT /rest/api/2/application-properties/{id}
        pub async fn set_prop(&self, url: String, token: String, id: String) {
            
        }
    }
}
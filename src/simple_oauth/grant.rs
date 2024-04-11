pub enum SimpleOauthGrant {
    ClientCredentials {
        client_id: String,
        client_secret: String,
        scopes: Vec<String>,
    },
    RefreshToken {
        client_id: String,
        client_secret: String,
        refresh_token: String,
        scopes: Vec<String>,
    }
}

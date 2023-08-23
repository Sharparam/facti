macro_rules! api_client_builder {
    ($base_client:ty, $api_client:ident) => {
        /// A builder for
        #[doc = concat!("[`", stringify!($api_client), "`].")]
        ///
        /// You can construct one by calling
        #[doc = concat!("[`", stringify!($api_client), "::builder()`].")]
        #[derive(Default)]
        pub struct ApiClientBuilder {
            client: Option<$base_client>,
            portal_base_url: Option<Url>,
            portal_api_base_url: Option<Url>,
            game_base_url: Option<Url>,
            api_key: Option<String>,
        }

        impl ApiClientBuilder {
            fn new() -> Self {
                Default::default()
            }

            /// Sets the underlying [`reqwest`] client to use.
            ///
            /// If this is not configured, it will use the default client.
            pub fn client(&mut self, client: $base_client) -> &mut Self {
                self.client = Some(client);
                self
            }

            /// Configures the base URL for the mod portal (non-API resources).
            ///
            /// If not configured, it will default to [`DEFAULT_PORTAL_BASE_URL`].
            pub fn portal_base_url<T: Into<Url>>(&mut self, base_url: T) -> &mut Self {
                self.portal_base_url = Some(base_url.into());
                self
            }

            /// Configures the base URL for the mod portal API.
            ///
            /// If not configured, it will default to [`DEFAULT_PORTAL_API_BASE_URL`].
            pub fn portal_api_base_url<T: Into<Url>>(&mut self, base_url: T) -> &mut Self {
                self.portal_api_base_url = Some(base_url.into());
                self
            }

            /// Configures the base URL for the game API.
            ///
            /// If not configured, it will default to [`DEFAULT_GAME_BASE_URL`].
            pub fn game_base_url<T: Into<Url>>(&mut self, base_url: T) -> &mut Self {
                self.game_base_url = Some(base_url.into());
                self
            }

            /// Configures an API key to use.
            ///
            /// If not configured, the API will be used in anonymous mode.
            pub fn api_key<T: Into<String>>(&mut self, api_key: T) -> &mut Self {
                self.api_key = Some(api_key.into());
                self
            }

            /// Builds a finished
            #[doc = concat!("[`", stringify!($api_client), "`].")]
            pub fn build(self) -> $api_client {
                let client = self.client.unwrap_or_default();
                let portal_base_url = self
                    .portal_base_url
                    .unwrap_or(Url::parse(DEFAULT_PORTAL_API_BASE_URL).unwrap());
                let portal_api_base_url = self
                    .portal_api_base_url
                    .unwrap_or(Url::parse(DEFAULT_PORTAL_API_BASE_URL).unwrap());
                let game_base_url = self
                    .game_base_url
                    .unwrap_or(Url::parse(DEFAULT_GAME_BASE_URL).unwrap());

                let urls = FactorioUrls {
                    portal_base_url,
                    portal_api_base_url,
                    game_base_url,
                };

                $api_client {
                    client,
                    urls,
                    api_key: self.api_key,
                }
            }
        }
    };
}

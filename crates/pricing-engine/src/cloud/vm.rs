//! Fresh provider-owned VM rates in USD per node-hour. Missing prices are never defaults.
//!
//! AWS Query pricing uses the optional `aws` feature and a caller-owned SDK config.
//! Azure: <https://learn.microsoft.com/rest/api/cost-management/retail-prices/azure-retail-prices>
//! DigitalOcean: <https://docs.digitalocean.com/reference/api/reference/sizes/>
//! Vultr has no complete hourly binding: <https://github.com/vultr/govultr/blob/master/plan.go>

use crate::error::{PricingError, Result};
use crate::types::CloudProvider;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use url::Url;
use zeroize::Zeroizing;

#[derive(Clone, Debug)]
pub struct InstanceInfo {
    pub name: String,
    pub vcpus: f32,
    pub memory_gb: f32,
    pub hourly_price: f64,
}

/// Provider credentials stay in headers. Every request obtains a fresh rate.
#[derive(Clone)]
pub struct PricingFetcher {
    client: reqwest::Client,
    tokens: HashMap<CloudProvider, Zeroizing<String>>,
    #[cfg(feature = "aws")]
    aws: Option<aws_sdk_pricing::Client>,
    endpoints: HashMap<CloudProvider, Url>,
}

impl std::fmt::Debug for PricingFetcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PricingFetcher")
            .field("providers", &self.tokens.keys().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}

impl Default for PricingFetcher {
    fn default() -> Self {
        Self::new_or_default()
    }
}

impl PricingFetcher {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .map_err(|e| PricingError::HttpError(e.without_url().to_string()))?,
            tokens: HashMap::new(),
            #[cfg(feature = "aws")]
            aws: None,
            endpoints: HashMap::new(),
        })
    }

    pub fn new_or_default() -> Self {
        Self::new().expect("secure provider pricing HTTP client")
    }

    fn with_token(mut self, provider: CloudProvider, token: String) -> Result<Self> {
        if token.trim().is_empty() || reqwest::header::HeaderValue::from_str(&token).is_err() {
            return Err(PricingError::Config(
                "Invalid provider pricing token".into(),
            ));
        }
        self.tokens.insert(provider, Zeroizing::new(token));
        Ok(self)
    }

    /// Explicit environment bridge for CLI callers; library callers supply credentials.
    pub async fn from_env() -> Result<Self> {
        let mut fetcher = Self::new()?;
        if let Ok(token) = std::env::var("DIGITALOCEAN_TOKEN") {
            fetcher = fetcher.with_digitalocean_token(token)?;
        }
        if let Ok(token) = std::env::var("VULTR_API_KEY") {
            fetcher = fetcher.with_vultr_token(token)?;
        }
        #[cfg(feature = "aws")]
        if [
            "AWS_ACCESS_KEY_ID",
            "AWS_SECRET_ACCESS_KEY",
            "AWS_PROFILE",
            "AWS_WEB_IDENTITY_TOKEN_FILE",
        ]
        .iter()
        .any(|name| std::env::var_os(name).is_some())
        {
            let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
                .region(aws_config::Region::new("us-east-1"))
                .load()
                .await;
            fetcher = fetcher.with_aws_config(&config)?;
        }
        Ok(fetcher)
    }

    pub fn with_digitalocean_token(self, token: String) -> Result<Self> {
        self.with_token(CloudProvider::DigitalOcean, token)
    }

    pub fn with_vultr_token(self, token: String) -> Result<Self> {
        self.with_token(CloudProvider::Vultr, token)
    }

    #[cfg(feature = "aws")]
    pub fn with_aws_config(mut self, config: &aws_config::SdkConfig) -> Result<Self> {
        if let Some(endpoint) = config.endpoint_url() {
            Self::parse_endpoint(endpoint)?;
        }
        let pricing = aws_sdk_pricing::config::Builder::from(config)
            .region(aws_sdk_pricing::config::Region::new("us-east-1"))
            .build();
        self.aws = Some(aws_sdk_pricing::Client::from_conf(pricing));
        Ok(self)
    }

    /// A caller-owned API gateway; plaintext is restricted to loopback HTTP peers.
    pub fn with_endpoint(mut self, provider: CloudProvider, endpoint: &str) -> Result<Self> {
        if matches!(provider, CloudProvider::AWS | CloudProvider::GCP) {
            return Err(PricingError::Config(
                "Supply the AWS gateway in its SDK config; GCP has no complete VM price binding"
                    .into(),
            ));
        }
        let url = Self::parse_endpoint(endpoint)?;
        self.endpoints.insert(provider, url);
        Ok(self)
    }

    fn parse_endpoint(endpoint: &str) -> Result<Url> {
        let url = Url::parse(endpoint)
            .map_err(|_| PricingError::Config("Invalid pricing endpoint".into()))?;
        let loopback = url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        if !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
        {
            return Err(PricingError::Config(
                "Pricing endpoint requires HTTPS or loopback HTTP without credentials".into(),
            ));
        }
        Ok(url)
    }

    fn endpoint(&self, provider: &CloudProvider) -> Result<Url> {
        if let Some(url) = self.endpoints.get(provider) {
            return Ok(url.clone());
        }
        let base = match provider {
            CloudProvider::Azure => "https://prices.azure.com",
            CloudProvider::DigitalOcean => "https://api.digitalocean.com",
            CloudProvider::Vultr => "https://api.vultr.com",
            _ => {
                return Err(PricingError::Config(
                    "No whole-instance pricing API configured".into(),
                ));
            }
        };
        Url::parse(base)
            .map_err(|_| PricingError::Config("Invalid provider pricing endpoint".into()))
    }

    async fn json(&self, provider: &CloudProvider, url: Url) -> Result<Value> {
        let base = self.endpoint(provider)?;
        if url.origin() != base.origin() || !url.username().is_empty() || url.password().is_some() {
            return Err(PricingError::Config(
                "Pricing pagination left the configured API origin".into(),
            ));
        }
        let mut request = self.client.get(url);
        if let Some(token) = self.tokens.get(provider) {
            request = request.bearer_auth(token.as_str());
        }
        let mut response = request
            .send()
            .await
            .map_err(|e| PricingError::HttpError(e.without_url().to_string()))?;
        if matches!(response.status().as_u16(), 401 | 403) {
            return Err(PricingError::AuthRejected);
        }
        if !response.status().is_success() {
            return Err(PricingError::HttpError(format!(
                "Provider pricing HTTP {}",
                response.status()
            )));
        }
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| PricingError::HttpError(e.without_url().to_string()))?
        {
            if body.len().saturating_add(chunk.len()) > 10 * 1024 * 1024 {
                return Err(PricingError::HttpError(
                    "Provider pricing response exceeds the JSON body limit".into(),
                ));
            }
            body.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&body)
            .map_err(|_| PricingError::HttpError("Invalid provider pricing JSON".into()))
    }

    /// Fresh exact-SKU total, or None when the provider has no usable whole-node rate.
    pub async fn quote_instance(
        &self,
        provider: CloudProvider,
        region: &str,
        exact_sku: &str,
    ) -> Result<Option<f64>> {
        if exact_sku.is_empty() || region.is_empty() {
            return Ok(None);
        }
        // Compute machine types are not Billing SKU ids; components do not quote a whole VM.
        // https://cloud.google.com/billing/docs/reference/rest/v1/services.skus/list
        if provider == CloudProvider::GCP {
            return Ok(None);
        }
        if provider == CloudProvider::AWS {
            #[cfg(feature = "aws")]
            return Ok(self
                .aws_instances(region, Some(exact_sku))
                .await?
                .into_iter()
                .map(|instance| instance.hourly_price)
                .min_by(f64::total_cmp));
            #[cfg(not(feature = "aws"))]
            return Ok(None);
        }
        Ok(self
            .instances(provider, region, Some(exact_sku))
            .await?
            .into_iter()
            .filter(|instance| instance.name == exact_sku)
            .map(|instance| instance.hourly_price)
            .min_by(f64::total_cmp))
    }

    pub async fn find_best_instance(
        &mut self,
        provider: CloudProvider,
        region: &str,
        min_cpu: f32,
        min_memory_gb: f32,
        max_price: f64,
    ) -> Result<InstanceInfo> {
        if !min_cpu.is_finite()
            || min_cpu <= 0.0
            || !min_memory_gb.is_finite()
            || min_memory_gb <= 0.0
            || !max_price.is_finite()
            || max_price <= 0.0
        {
            return Err(PricingError::Config(
                "Invalid VM pricing requirements".into(),
            ));
        }
        self.get_instances(provider, region)
            .await?
            .into_iter()
            .filter(|instance| {
                instance.vcpus >= min_cpu
                    && instance.memory_gb >= min_memory_gb
                    && instance.hourly_price <= max_price
            })
            .min_by(|a, b| a.hourly_price.total_cmp(&b.hourly_price))
            .ok_or_else(|| PricingError::Pricing("No priced instance satisfies the request".into()))
    }

    /// Fresh provider-owned rates with available hardware metadata.
    pub async fn get_instances(
        &self,
        provider: CloudProvider,
        region: &str,
    ) -> Result<Vec<InstanceInfo>> {
        if region.is_empty() {
            return Ok(Vec::new());
        }
        if provider == CloudProvider::GCP {
            return Ok(Vec::new());
        }
        if provider == CloudProvider::AWS {
            #[cfg(feature = "aws")]
            return self.aws_instances(region, None).await;
            #[cfg(not(feature = "aws"))]
            return Ok(Vec::new());
        }
        self.instances(provider, region, None).await
    }

    #[cfg(feature = "aws")]
    async fn aws_instances(&self, region: &str, sku: Option<&str>) -> Result<Vec<InstanceInfo>> {
        use aws_sdk_pricing::{
            error::ProvideErrorMetadata,
            types::{Filter, FilterType},
        };
        let Some(client) = &self.aws else {
            return Ok(Vec::new());
        };
        let mut dimensions = vec![
            ("regionCode", region),
            ("operatingSystem", "Linux"),
            ("tenancy", "Shared"),
            ("preInstalledSw", "NA"),
            ("capacitystatus", "Used"),
        ];
        if let Some(sku) = sku {
            dimensions.push(("instanceType", sku));
        }
        let filters = dimensions
            .into_iter()
            .map(|(field, value)| {
                Filter::builder()
                    .field(field)
                    .value(value)
                    .r#type(FilterType::TermMatch)
                    .build()
                    .map_err(|_| PricingError::Config("Invalid AWS pricing filter".into()))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut token = None;
        let mut seen = HashSet::new();
        let mut instances = Vec::new();
        loop {
            let page = client
                .get_products()
                .service_code("AmazonEC2")
                .set_filters(Some(filters.clone()))
                .set_next_token(token)
                .max_results(100)
                .send()
                .await
                .map_err(|error| {
                    if error
                        .as_service_error()
                        .and_then(|error| error.code())
                        .is_some_and(|code| {
                            matches!(
                                code,
                                "AccessDeniedException"
                                    | "UnrecognizedClientException"
                                    | "InvalidSignatureException"
                                    | "ExpiredTokenException"
                            )
                        })
                    {
                        PricingError::AuthRejected
                    } else {
                        PricingError::HttpError("AWS Pricing Query failed".into())
                    }
                })?;
            for document in page.price_list() {
                let product: Value = serde_json::from_str(document)
                    .map_err(|_| PricingError::HttpError("Invalid AWS price document".into()))?;
                let attrs = &product["product"]["attributes"];
                let Some(name) = attrs["instanceType"].as_str() else {
                    continue;
                };
                if sku.is_some_and(|sku| sku != name)
                    || attrs["regionCode"] != region
                    || attrs["operatingSystem"] != "Linux"
                    || attrs["tenancy"] != "Shared"
                    || attrs["preInstalledSw"] != "NA"
                    || attrs["capacitystatus"] != "Used"
                {
                    continue;
                }
                let Some(terms) = product["terms"]["OnDemand"].as_object() else {
                    continue;
                };
                let mut price: Option<f64> = None;
                for term in terms.values() {
                    let Some(dimensions) = term["priceDimensions"].as_object() else {
                        continue;
                    };
                    for dimension in dimensions.values() {
                        if dimension["unit"] != "Hrs"
                            || dimension["beginRange"] != "0"
                            || dimension["endRange"] != "Inf"
                        {
                            continue;
                        }
                        if let Some(rate) = number(&dimension["pricePerUnit"]["USD"]) {
                            price = Some(price.map_or(rate, |current| current.min(rate)));
                        }
                    }
                }
                if let Some(hourly_price) = price {
                    let vcpus = attrs["vcpu"]
                        .as_str()
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(0.0);
                    let memory_gb = attrs["memory"]
                        .as_str()
                        .and_then(|value| value.split_whitespace().next())
                        .and_then(|value| value.replace(',', "").parse().ok())
                        .unwrap_or(0.0);
                    instances.push(InstanceInfo {
                        name: name.into(),
                        vcpus,
                        memory_gb,
                        hourly_price,
                    });
                }
            }
            token = page
                .next_token()
                .filter(|token| !token.is_empty())
                .map(str::to_owned);
            let Some(next) = &token else {
                break;
            };
            if seen.len() >= 100 || !seen.insert(next.clone()) {
                return Err(PricingError::HttpError(
                    "AWS pricing page budget or repeated cursor".into(),
                ));
            }
        }
        Ok(instances)
    }

    async fn instances(
        &self,
        provider: CloudProvider,
        region: &str,
        sku: Option<&str>,
    ) -> Result<Vec<InstanceInfo>> {
        let path: &str = match provider {
            CloudProvider::Azure => "/api/retail/prices",
            CloudProvider::DigitalOcean => "/v2/sizes?per_page=200",
            CloudProvider::Vultr => "/v2/plans?per_page=100",
            _ => return Ok(Vec::new()),
        };
        let base = self.endpoint(&provider)?;
        let mut url = base
            .join(path)
            .map_err(|_| PricingError::Config("Invalid provider pricing route".into()))?;
        if provider == CloudProvider::Azure {
            let mut filter = format!(
                "serviceName eq 'Virtual Machines' and armRegionName eq '{}' and priceType eq 'Consumption'",
                region.replace('\'', "''")
            );
            if let Some(sku) = sku {
                filter.push_str(&format!(" and armSkuName eq '{}'", sku.replace('\'', "''")));
            }
            url.query_pairs_mut()
                .append_pair("currencyCode", "USD")
                .append_pair("$filter", &filter);
        }
        let mut seen = HashSet::new();
        let mut instances = Vec::new();
        loop {
            if seen.len() >= 100 {
                return Err(PricingError::HttpError(
                    "Provider pricing page budget exceeded".into(),
                ));
            }
            if !seen.insert(url.to_string()) {
                return Err(PricingError::HttpError(
                    "Provider pricing pagination repeated".into(),
                ));
            }
            let data = self.json(&provider, url).await?;
            match provider {
                CloudProvider::Azure => {
                    for row in rows(&data, "Items")? {
                        let name = row["armSkuName"].as_str().unwrap_or("");
                        let product = row["productName"].as_str().unwrap_or("");
                        let meter = row["meterName"].as_str().unwrap_or("");
                        if name.is_empty()
                            || sku.is_some_and(|expected| expected != name)
                            || row["armRegionName"] != region
                            || row["currencyCode"] != "USD"
                            || row["type"] != "Consumption"
                            || row["unitOfMeasure"] != "1 Hour"
                            || product.is_empty()
                            || meter.is_empty()
                            || row["serviceName"] != "Virtual Machines"
                            || row["isPrimaryMeterRegion"] == false
                            || [product, meter, row["skuName"].as_str().unwrap_or("")]
                                .iter()
                                .any(|label| {
                                    ["windows", "spot", "low priority"]
                                        .iter()
                                        .any(|kind| label.to_ascii_lowercase().contains(kind))
                                })
                            || meter.to_ascii_lowercase().contains("spot")
                            || meter.to_ascii_lowercase().contains("low priority")
                            || row["tierMinimumUnits"]
                                .as_f64()
                                .is_some_and(|tier| tier != 0.0)
                        {
                            continue;
                        }
                        if let Some(price) = number(&row["retailPrice"]) {
                            instances.push(InstanceInfo {
                                name: name.into(),
                                vcpus: 0.0,
                                memory_gb: 0.0,
                                hourly_price: price,
                            });
                        }
                    }
                    let Some(next) = data["NextPageLink"]
                        .as_str()
                        .filter(|next| !next.is_empty())
                    else {
                        break;
                    };
                    url = base
                        .join(next)
                        .map_err(|_| PricingError::HttpError("Invalid pricing next page".into()))?;
                }
                CloudProvider::Vultr => {
                    // Checked 2026-11-06: plans expose monthly_cost, not a complete
                    // region-specific hourly rate. A monthly cap is not an hourly quote.
                    // https://docs.vultr.com/support/platform/billing/how-am-i-billed-for-my-servers
                    return Ok(Vec::new());
                }
                CloudProvider::DigitalOcean => {
                    for row in rows(&data, "sizes")? {
                        let name = row["slug"].as_str().unwrap_or("");
                        if name.is_empty()
                            || sku.is_some_and(|expected| expected != name)
                            || row["available"] != true
                            || !row["regions"].as_array().is_some_and(|regions| {
                                regions.iter().any(|candidate| candidate == region)
                            })
                        {
                            continue;
                        }
                        if let Some(hourly_price) = number(&row["price_hourly"]) {
                            instances.push(InstanceInfo {
                                name: name.into(),
                                vcpus: number(&row["vcpus"]).unwrap_or(0.0) as f32,
                                memory_gb: (number(&row["memory"]).unwrap_or(0.0) / 1024.0) as f32,
                                hourly_price,
                            });
                        }
                    }
                    let Some(next) = data["links"]["pages"]["next"]
                        .as_str()
                        .filter(|next| !next.is_empty())
                    else {
                        break;
                    };
                    url = base
                        .join(next)
                        .map_err(|_| PricingError::HttpError("Invalid pricing next page".into()))?;
                }
                _ => unreachable!(),
            }
        }
        Ok(instances)
    }
}

fn number(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
        .filter(|price| price.is_finite() && *price > 0.0)
}

fn rows<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>> {
    value[field]
        .as_array()
        .ok_or_else(|| PricingError::HttpError(format!("Provider pricing {field} missing")))
}

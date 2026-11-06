//! Fresh provider-owned VM rates in USD per node-hour. Missing prices are never defaults.
//!
//! AWS Query pricing is supplied by the optional remote-provider AWS SDK client.
//! Azure: <https://learn.microsoft.com/rest/api/cost-management/retail-prices/azure-retail-prices>
//! DigitalOcean: <https://docs.digitalocean.com/reference/api/reference/sizes/>
//! Vultr: <https://api.vultr.com/v2/plans>

use crate::error::{PricingError, Result};
use crate::types::CloudProvider;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use url::Url;

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
    tokens: HashMap<CloudProvider, String>,
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
            endpoints: HashMap::new(),
        })
    }

    pub fn new_or_default() -> Self {
        Self::new().expect("secure provider pricing HTTP client")
    }

    pub fn with_token(mut self, provider: CloudProvider, token: String) -> Result<Self> {
        if token.trim().is_empty() || reqwest::header::HeaderValue::from_str(&token).is_err() {
            return Err(PricingError::Config(
                "Invalid provider pricing token".into(),
            ));
        }
        self.tokens.insert(provider, token);
        Ok(self)
    }

    /// A caller-owned API gateway; plaintext is restricted to loopback HTTP peers.
    pub fn with_endpoint(mut self, provider: CloudProvider, endpoint: &str) -> Result<Self> {
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
        self.endpoints.insert(provider, url);
        Ok(self)
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
            request = request.bearer_auth(token);
        }
        let mut response = request
            .send()
            .await
            .map_err(|e| PricingError::HttpError(e.without_url().to_string()))?;
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
        // As checked 2026-11-06, Compute machine types are not Billing SKU ids.
        // Pricing a GPU/core/RAM component as a whole VM would underquote it.
        // Source: https://cloud.google.com/billing/docs/reference/rest/v1/services.skus/list
        if matches!(provider, CloudProvider::AWS | CloudProvider::GCP) {
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
        self.instances(provider, region, None)
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

    async fn instances(
        &self,
        provider: CloudProvider,
        region: &str,
        sku: Option<&str>,
    ) -> Result<Vec<InstanceInfo>> {
        let base = self.endpoint(&provider)?;
        let path = match provider {
            CloudProvider::Azure => "/api/retail/prices".into(),
            CloudProvider::DigitalOcean => "/v2/sizes?per_page=200".into(),
            CloudProvider::Vultr => "/v2/plans?per_page=100".into(),
            _ => {
                return Err(PricingError::Config(
                    "No whole-instance pricing API configured".into(),
                ));
            }
        };
        let mut url = base
            .join(&path)
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
                            || product.to_ascii_lowercase().contains("windows")
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
                CloudProvider::DigitalOcean | CloudProvider::Vultr => {
                    let digitalocean = provider == CloudProvider::DigitalOcean;
                    for row in rows(&data, if digitalocean { "sizes" } else { "plans" })? {
                        let name = row[if digitalocean { "slug" } else { "id" }]
                            .as_str()
                            .unwrap_or("");
                        let regions =
                            row[if digitalocean { "regions" } else { "locations" }].as_array();
                        if name.is_empty()
                            || sku.is_some_and(|expected| expected != name)
                            || !regions.is_some_and(|regions| regions.iter().any(|r| r == region))
                            || (digitalocean && row["available"] != true)
                            || (!digitalocean && row["deploy_ondemand"] != true)
                        {
                            continue;
                        }
                        let price = if digitalocean {
                            number(&row["price_hourly"])
                        } else if row["location_cost"].get(region).is_some() {
                            number(&row["location_cost"][region]["hourly_cost"])
                        } else {
                            number(&row["hourly_cost"])
                        };
                        if let Some(price) = price {
                            instances.push(InstanceInfo {
                                name: name.into(),
                                vcpus: number(
                                    &row[if digitalocean { "vcpus" } else { "vcpu_count" }],
                                )
                                .unwrap_or(0.0) as f32,
                                memory_gb: (number(
                                    &row[if digitalocean { "memory" } else { "ram" }],
                                )
                                .unwrap_or(0.0)
                                    / 1024.0) as f32,
                                hourly_price: price,
                            });
                        }
                    }
                    let next = if digitalocean {
                        data["links"]["pages"]["next"].as_str()
                    } else {
                        data["meta"]["links"]["next"].as_str()
                    };
                    let Some(next) = next.filter(|next| !next.is_empty()) else {
                        break;
                    };
                    if digitalocean {
                        url = base.join(next).map_err(|_| {
                            PricingError::HttpError("Invalid pricing next page".into())
                        })?;
                    } else {
                        url = base
                            .join("/v2/plans")
                            .map_err(|_| PricingError::HttpError("Invalid pricing route".into()))?;
                        url.query_pairs_mut()
                            .append_pair("per_page", "100")
                            .append_pair("cursor", next);
                    }
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

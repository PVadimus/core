use primitives::{Asset, Device, Price, PriceAlertSubsriptions};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use storage::{models::PriceAlert, DatabaseClient};

#[allow(dead_code)]
pub struct PriceAlertClient {
    database: DatabaseClient,
}

#[derive(Clone, Debug)]
pub struct PriceAlertNotification {
    pub device: Device,
    pub asset: Asset,
    pub price: Price,
}

#[derive(Clone, Debug)]
pub struct PriceAlertRules {
    pub price_change_increase: f64,
}

impl PriceAlertClient {
    pub async fn new(database_url: &str) -> Self {
        let database = DatabaseClient::new(database_url);
        Self { database }
    }

    pub async fn get_price_alerts(&mut self, device_id: &str) -> Result<PriceAlertSubsriptions, Box<dyn Error>> {
        let device = self.database.get_device(device_id)?;
        let values = self
            .database
            .get_price_alerts_for_device_id(device.id)?
            .into_iter()
            .map(|x| x.as_primitive())
            .collect::<_>();
        Ok(values)
    }

    pub async fn add_price_alerts(&mut self, device_id: &str, subscriptions: PriceAlertSubsriptions) -> Result<usize, Box<dyn Error>> {
        let device = self.database.get_device(device_id)?;
        let values = subscriptions.into_iter().map(|x| PriceAlert::from_primitive(x, device.id)).collect::<_>();
        Ok(self.database.add_price_alerts(values)?)
    }

    pub async fn delete_price_alerts(&mut self, device_id: &str, subscriptions: PriceAlertSubsriptions) -> Result<usize, Box<dyn Error>> {
        let device = self.database.get_device(device_id)?;
        let asset_ids: Vec<_> = subscriptions.iter().map(|x| x.asset_id.as_str()).collect::<HashSet<_>>().into_iter().collect();

        Ok(self.database.delete_price_alerts(device.id, asset_ids)?)
    }

    pub async fn get_devices_to_alert(&mut self, rules: PriceAlertRules) -> Result<Vec<PriceAlertNotification>, Box<dyn Error>> {
        let now = chrono::Utc::now().naive_utc();
        let after_notified_at = now - chrono::Duration::hours(24);

        let prices = self.database.get_prices()?;
        let prices_assets = self.database.get_prices_assets()?;
        let prices_assets_map: HashMap<String, HashSet<String>> = prices_assets.into_iter().fold(HashMap::new(), |mut map, price_asset| {
            map.entry(price_asset.price_id.clone()).or_default().insert(price_asset.asset_id);
            map
        });

        let price_alerts = self.database.get_price_alerts(after_notified_at)?;

        let mut results: Vec<PriceAlertNotification> = Vec::new();
        let mut device_ids: HashSet<i32> = HashSet::new();

        for price in prices {
            if price.price_change_percentage_24h > rules.price_change_increase {
                if let Some(asset_ids) = prices_assets_map.get(&price.id) {
                    for price_alert in price_alerts.clone() {
                        if asset_ids.clone().contains(&price_alert.asset_id) {
                            device_ids.insert(price_alert.device_id);

                            let asset = self.database.get_asset(&price_alert.asset_id)?.as_primitive();
                            let device = self.database.get_device_by_id(price_alert.device_id)?.as_primitive();

                            let notification = PriceAlertNotification {
                                device,
                                asset,
                                price: price.as_price_primitive(),
                            };

                            results.push(notification);
                        }
                    }
                }
            }
        }
        self.database.update_price_alerts_set_notified_at(device_ids.into_iter().collect(), now)?;
        Ok(results)
    }
}

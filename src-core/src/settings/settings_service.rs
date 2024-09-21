// settings_service.rs

use crate::models::{Asset, ExchangeRate, NewSettings, Settings};
use crate::schema::assets::dsl::*;
use crate::schema::settings::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct SettingsService {
    settings_id: i32,
}

impl SettingsService {
    pub fn new() -> Self {
        SettingsService { settings_id: 1 }
    }

    pub fn get_settings(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Settings, diesel::result::Error> {
        settings.find(self.settings_id).first::<Settings>(conn)
    }

    pub fn update_settings(
        &self,
        conn: &mut SqliteConnection,
        new_setting: &NewSettings,
    ) -> Result<(), diesel::result::Error> {
        // First, try to update
        let rows_affected = diesel::update(settings.find(self.settings_id))
            .set(new_setting)
            .execute(conn)?;

        // Check if the update affected any rows
        if rows_affected == 0 {
            // If no rows were affected, perform an insert
            diesel::insert_into(settings)
                .values(new_setting)
                .execute(conn)?;
        }

        Ok(())
    }

    pub fn update_base_currency(
        &self,
        conn: &mut SqliteConnection,
        new_base_currency: &str,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(settings.find(self.settings_id))
            .set(base_currency.eq(new_base_currency))
            .execute(conn)?;
        Ok(())
    }

    pub fn get_exchange_rates(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<ExchangeRate>, diesel::result::Error> {
        let asset_rates: Vec<Asset> = assets
            .filter(asset_type.eq("Currency"))
            .load::<Asset>(conn)?;
        Ok(asset_rates
            .into_iter()
            .map(|asset| {
                let symbol_parts: Vec<&str> = asset.symbol.split('=').collect();
                ExchangeRate {
                    id: asset.id,
                    from_currency: symbol_parts[0][..3].to_string(),
                    to_currency: symbol_parts[0][3..].to_string(),
                    rate: asset.name.unwrap_or_default().parse().unwrap_or(1.0),
                    source: asset.data_source,
                }
            })
            .collect())
    }

    pub fn update_exchange_rate(
        &self,
        conn: &mut SqliteConnection,
        rate: &ExchangeRate,
    ) -> Result<ExchangeRate, diesel::result::Error> {
        let asset = Asset {
            id: rate.id.clone(),
            symbol: format!("{}{}=X", rate.from_currency, rate.to_currency),
            name: Some(rate.rate.to_string()),
            asset_type: Some("Currency".to_string()),
            data_source: rate.source.clone(),
            currency: rate.to_currency.clone(),
            updated_at: chrono::Utc::now().naive_utc(),
            ..Default::default()
        };

        diesel::update(assets.find(&asset.id))
            .set(&asset)
            .execute(conn)?;

        Ok(rate.clone())
    }
}

use anyhow::{Context, Result};
use log::{debug, error, info};
use sheets::{
    self,
    types::{
        ClearValuesRequest, DateTimeRenderOption, Dimension, ValueInputOption, ValueRange,
        ValueRenderOption,
    },
};
use yup_oauth2::{AccessToken, ApplicationSecret};

pub struct SheetsManager {
    client: sheets::spreadsheets::Spreadsheets,
    spreadsheet_id: String,
}

impl SheetsManager {
    pub fn new(
        secret: ApplicationSecret,
        token: AccessToken,
        spreadsheet_id: String,
    ) -> Result<Self> {
        let token_str = token
            .token()
            .ok_or_else(|| anyhow::anyhow!("Failed to get token string"))?;

        let sheet_client = sheets::Client::new(
            secret.client_id,
            secret.client_secret,
            secret.redirect_uris[0].clone(),
            token_str,
            token_str,
        );

        let client = sheets::spreadsheets::Spreadsheets {
            client: sheet_client,
        };

        Ok(SheetsManager {
            client,
            spreadsheet_id,
        })
    }

    pub async fn verify_spreadsheet_access(&self) -> Result<()> {
        let sc = self
            .client
            .get(&self.spreadsheet_id, false, &[])
            .await
            .with_context(|| format!("Failed to access spreadsheet {}", self.spreadsheet_id))?;

        info!("Connected to spreadsheet {}", sc.body.spreadsheet_id);
        Ok(())
    }

    pub async fn verify_sheet_exists(&self, sheet_name: &str) -> Result<()> {
        let sc = self
            .client
            .get(&self.spreadsheet_id, false, &[])
            .await
            .with_context(|| format!("Failed to access spreadsheet {}", self.spreadsheet_id))?;

        let sheet_exists = sc.body.sheets.iter().any(|sheet| {
            if let Some(props) = &sheet.properties {
                props.title == sheet_name
            } else {
                false
            }
        });

        if !sheet_exists {
            error!("Sheet '{}' not found in spreadsheet", sheet_name);
            return Err(anyhow::anyhow!(
                "Sheet '{}' not found in spreadsheet",
                sheet_name
            ));
        }

        Ok(())
    }

    pub async fn clear_range(&self, sheet_name: &str) -> Result<()> {
        let clear_range = format!("{}!A2:AX58", sheet_name);
        self.client
            .values_clear(&self.spreadsheet_id, &clear_range, &ClearValuesRequest {})
            .await
            .with_context(|| format!("Error clearing data in range {}", clear_range))?;

        info!("Cleared old data from {}", clear_range);
        Ok(())
    }

    pub async fn upload_data(&self, sheet_name: &str, matrix: Vec<Vec<String>>) -> Result<()> {
        let new_range = format!("{}!A2:AX{}", sheet_name, matrix.len() + 1);
        let update_body = ValueRange {
            values: matrix,
            major_dimension: Some(Dimension::Rows),
            range: new_range.clone(),
        };

        debug!("Updating range: {}", new_range);

        let update = self
            .client
            .values_update(
                &self.spreadsheet_id,
                &new_range,
                false,
                DateTimeRenderOption::FormattedString,
                ValueRenderOption::FormattedValue,
                ValueInputOption::UserEntered,
                &update_body,
            )
            .await
            .with_context(|| "Failed to upload data to spreadsheet")?;

        info!("Updated data: {}", update.status);
        Ok(())
    }
}
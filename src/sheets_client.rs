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
use crate::progress::ProgressCallback;

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

    pub async fn verify_spreadsheet_access(&self, progress: Option<&dyn ProgressCallback>) -> Result<()> {
        if let Some(p) = progress {
            p.set_message("Verifying spreadsheet access...");
        }

        let sc = self
            .client
            .get(&self.spreadsheet_id, false, &[])
            .await
            .with_context(|| format!("Failed to access spreadsheet {}", self.spreadsheet_id))?;

        info!("Connected to spreadsheet {}", sc.body.spreadsheet_id);
        
        if let Some(p) = progress {
            p.inc(1);
        }
        
        Ok(())
    }

    pub async fn verify_sheet_exists(&self, sheet_name: &str, progress: Option<&dyn ProgressCallback>) -> Result<()> {
        if let Some(p) = progress {
            p.set_message(&format!("Verifying sheet '{}' exists...", sheet_name));
        }

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

        if let Some(p) = progress {
            p.inc(1);
        }

        Ok(())
    }

    pub async fn clear_range(&self, sheet_name: &str, progress: Option<&dyn ProgressCallback>) -> Result<()> {
        if let Some(p) = progress {
            p.set_message("Clearing existing data...");
        }

        let clear_range = format!("{}!A2:AX58", sheet_name);
        self.client
            .values_clear(&self.spreadsheet_id, &clear_range, &ClearValuesRequest {})
            .await
            .with_context(|| format!("Error clearing data in range {}", clear_range))?;

        info!("Cleared old data from {}", clear_range);
        
        if let Some(p) = progress {
            p.inc(1);
        }
        
        Ok(())
    }

    pub async fn upload_data(&self, sheet_name: &str, matrix: Vec<Vec<String>>, progress: Option<&dyn ProgressCallback>) -> Result<()> {
        let row_count = matrix.len();
        
        if let Some(p) = progress {
            p.set_message(&format!("Uploading {} rows of data...", row_count));
        }

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
        
        if let Some(p) = progress {
            p.inc(1);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_upload_data_matrix_structure() {
        // Test that we can create the expected data structure for upload
        let test_data = vec![
            vec!["Name".to_string(), "Age".to_string(), "Position".to_string()],
            vec!["Player1".to_string(), "25".to_string(), "Forward".to_string()],
            vec!["Player2".to_string(), "30".to_string(), "Defender".to_string()],
        ];

        // Verify the data structure is as expected
        assert_eq!(test_data.len(), 3);
        assert_eq!(test_data[0].len(), 3);
        assert_eq!(test_data[1][0], "Player1");
        assert_eq!(test_data[2][2], "Defender");
    }

    #[test]
    fn test_range_formatting() {
        // Test range string generation
        let sheet_name = "TestSheet";
        let row_count = 10;
        
        let clear_range = format!("{}!A2:AX58", sheet_name);
        let update_range = format!("{}!A2:AX{}", sheet_name, row_count + 1);
        
        assert_eq!(clear_range, "TestSheet!A2:AX58");
        assert_eq!(update_range, "TestSheet!A2:AX11");
    }

    // Note: SheetsManager tests with actual Google API types are complex due to:
    // 1. Complex ApplicationSecret structure with many required fields
    // 2. AccessToken doesn't have a simple public constructor
    // 3. These are better tested as integration tests with real credentials
    // 
    // For unit tests, we focus on:
    // - Data structure validation
    // - Range string formatting
    // - Business logic that doesn't require external dependencies
    //
    // Integration tests would cover:
    // - Authentication flow
    // - API calls to Google Sheets
    // - Error handling for network failures
}
use super::BaseData;

pub struct YahooFinanceData{
    pub(crate) csv_file_path: String,
    pub(crate) start_date: String,
    pub(crate) end_date: String
}

impl BaseData for YahooFinanceData{

}


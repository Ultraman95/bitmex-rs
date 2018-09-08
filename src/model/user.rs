#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserWalletResponse {
    pub account: f64,
    pub currency: String,
    pub prevDeposited: f64,
    pub prevWithdrawn: f64,
    pub prevTransferIn: f64,
    pub prevTransferOut: f64,
    pub prevAmount: f64,
    pub prevTimestamp: String,
    pub deltaDeposited: f64,
    pub deltaWithdrawn: f64,
    pub deltaTransferIn: f64,
    pub deltaTransferOut: f64,
    pub deltaAmount: f64,
    pub deposited: f64,
    pub withdrawn: f64,
    pub transferIn: f64,
    pub transferOut: f64,
    pub amount: f64,
    pub pendingCredit: f64,
    pub pendingDebit: f64,
    pub confirmedDebit: f64,
    pub timestamp: String,
    pub addr: String,
    pub script: String,
    pub withdrawalLock: Vec<String>,
}

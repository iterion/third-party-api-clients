//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/**
* The status of the task
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Status::Completed => "completed",
            Status::Pending => "pending",
            Status::Noop => "",
            Status::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status::Noop
    }
}
impl Status {
    pub fn is_noop(&self) -> bool {
        matches!(self, Status::Noop)
    }
}

/// Description and status of an onboarding task.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TaskDescription {
    /**
    * Description and status of an onboarding task.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * Description and status of an onboarding task.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

/// A supported file
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct File {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub inserted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A supported file
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sub_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConflictResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TimeoffType {
    #[serde(rename = "bereavement")]
    Bereavement,
    #[serde(rename = "extended_leave")]
    ExtendedLeave,
    #[serde(rename = "in_lieu_time")]
    InLieuTime,
    #[serde(rename = "maternity_leave")]
    MaternityLeave,
    #[serde(rename = "military_leave")]
    MilitaryLeave,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "paid_time_off")]
    PaidTimeOff,
    #[serde(rename = "parental_leave")]
    ParentalLeave,
    #[serde(rename = "paternity_leave")]
    PaternityLeave,
    #[serde(rename = "public_holiday")]
    PublicHoliday,
    #[serde(rename = "sick_leave")]
    SickLeave,
    #[serde(rename = "unpaid_leave")]
    UnpaidLeave,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TimeoffType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TimeoffType::Bereavement => "bereavement",
            TimeoffType::ExtendedLeave => "extended_leave",
            TimeoffType::InLieuTime => "in_lieu_time",
            TimeoffType::MaternityLeave => "maternity_leave",
            TimeoffType::MilitaryLeave => "military_leave",
            TimeoffType::Other => "other",
            TimeoffType::PaidTimeOff => "paid_time_off",
            TimeoffType::ParentalLeave => "parental_leave",
            TimeoffType::PaternityLeave => "paternity_leave",
            TimeoffType::PublicHoliday => "public_holiday",
            TimeoffType::SickLeave => "sick_leave",
            TimeoffType::UnpaidLeave => "unpaid_leave",
            TimeoffType::Noop => "",
            TimeoffType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TimeoffType {
    fn default() -> TimeoffType {
        TimeoffType::Noop
    }
}
impl TimeoffType {
    pub fn is_noop(&self) -> bool {
        matches!(self, TimeoffType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Creator {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
* Payroll run product type
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProductType {
    #[serde(rename = "eor")]
    Eor,
    #[serde(rename = "global_payroll")]
    GlobalPayroll,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ProductType::Eor => "eor",
            ProductType::GlobalPayroll => "global_payroll",
            ProductType::Noop => "",
            ProductType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProductType {
    fn default() -> ProductType {
        ProductType::Noop
    }
}
impl ProductType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProductType::Noop)
    }
}

/**
* Status of the payroll
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PayrollRunStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "finalized")]
    Finalized,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "waiting_for_customer_approval")]
    WaitingForCustomerApproval,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PayrollRunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PayrollRunStatus::Completed => "completed",
            PayrollRunStatus::Finalized => "finalized",
            PayrollRunStatus::Preparing => "preparing",
            PayrollRunStatus::Processing => "processing",
            PayrollRunStatus::Rejected => "rejected",
            PayrollRunStatus::WaitingForCustomerApproval => "waiting_for_customer_approval",
            PayrollRunStatus::Noop => "",
            PayrollRunStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PayrollRunStatus {
    fn default() -> PayrollRunStatus {
        PayrollRunStatus::Noop
    }
}
impl PayrollRunStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, PayrollRunStatus::Noop)
    }
}

/**
* Payroll Run type
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "main")]
    Main,
    #[serde(rename = "one_off")]
    OneOff,
    #[serde(rename = "pro_forma")]
    ProForma,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Type::Main => "main",
            Type::OneOff => "one_off",
            Type::ProForma => "pro_forma",
            Type::Noop => "",
            Type::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Noop
    }
}
impl Type {
    pub fn is_noop(&self) -> bool {
        matches!(self, Type::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Company {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PayrollRun {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub customer_inputs_reviewed: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field_for_employment_matching: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inserted_at: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_editor: Option<Creator>,
    #[serde()]
    pub legal_entity: RemoteEntity,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub mapping_rules: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub net_pay_extraction_expression: String,
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub period_end: Option<chrono::NaiveDate>,
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub period_start: Option<chrono::NaiveDate>,
    /**
    * Payroll run product type
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    /**
    * Status of the payroll
    */
    #[serde(default, skip_serializing_if = "PayrollRunStatus::is_noop")]
    pub status: PayrollRunStatus,
    /**
    * Indicates if an Employer has completed the Payroll Inputs review flow
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub summarize_automatically: bool,
    /**
    * Payroll Run type
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validations: Option<Company>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Address {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_2: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<AddressCountry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_details: Option<Company>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmploymentType {
    #[serde(rename = "contractor")]
    Contractor,
    #[serde(rename = "employee")]
    Employee,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmploymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmploymentType::Contractor => "contractor",
            EmploymentType::Employee => "employee",
            EmploymentType::Noop => "",
            EmploymentType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmploymentType {
    fn default() -> EmploymentType {
        EmploymentType::Noop
    }
}
impl EmploymentType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmploymentType::Noop)
    }
}

/// Complete information of an employment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Employment {
    #[serde()]
    pub address_details: Company,
    #[serde()]
    pub administrative_details: Company,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub bank_account_details: Vec<String>,
    #[serde()]
    pub billing_address_details: Company,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_id: String,
    #[serde()]
    pub contract_details: Company,
    /**
    * A supported country on Remote
    */
    #[serde()]
    pub country: Country,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde()]
    pub emergency_contact_details: Company,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub files: Vec<File>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_title: String,
    /**
    * All tasks that need to be completed before marking the employment as ready
    */
    #[serde()]
    pub onboarding_tasks: OnboardingTasks,
    #[serde()]
    pub personal_details: Company,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub personal_email: String,
    /**
    * Selected type of payment.
    */
    #[serde()]
    pub pricing_plan_details: PricingPlanDetails,
    /**
    * Complete information of an employment
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub provisional_start_date: Option<chrono::NaiveDate>,
    /**
    * The status of employment
    */
    #[serde()]
    pub status: EmploymentStatus,
    #[serde(
        default,
        skip_serializing_if = "EmploymentType::is_noop",
        rename = "type"
    )]
    pub type_: EmploymentType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Data {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub current_page: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub employments: Vec<MinimalEmployment>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_pages: i64,
}

/// Response schema listing many employments
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListEmploymentsResponse {
    /**
    * Response schema listing many employments
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeoffResponseData {
    #[serde()]
    pub timeoff: Timeoff,
}

/// Timeoff response
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeoffResponse {
    #[serde()]
    pub data: TimeoffResponseData,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListTimeoffTypesResponseData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<TimeoffType>,
}

/// Time off types response
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListTimeoffTypesResponse {
    /**
    * Time off types response
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTimeoffTypesResponseData>,
}

/// Description of the required params to create an employment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmploymentBasicParams {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub personal_email: String,
    /**
    * Description of the required params to create an employment.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub provisional_start_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "EmploymentType::is_noop",
        rename = "type"
    )]
    pub type_: EmploymentType,
}

/// Complete information of an employment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmploymentResponse {
    /**
    * Complete information of an employment
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment: Option<Employment>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UpdateApprovedTimeoffParamsStatus {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UpdateApprovedTimeoffParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UpdateApprovedTimeoffParamsStatus::Approved => "approved",
            UpdateApprovedTimeoffParamsStatus::Cancelled => "cancelled",
            UpdateApprovedTimeoffParamsStatus::Noop => "",
            UpdateApprovedTimeoffParamsStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UpdateApprovedTimeoffParamsStatus {
    fn default() -> UpdateApprovedTimeoffParamsStatus {
        UpdateApprovedTimeoffParamsStatus::Noop
    }
}
impl UpdateApprovedTimeoffParamsStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, UpdateApprovedTimeoffParamsStatus::Noop)
    }
}

/// Update timeoff params
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateApprovedTimeoffParams {
    /**
    * Update timeoff params
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<serde_json::Value>,
    /**
    * Update timeoff params
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub approver_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cancel_reason: String,
    /**
    * Update timeoff params
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<TimeoffDocumentParams>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub edit_reason: String,
    /**
    * Update timeoff params
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub end_date: Option<chrono::NaiveDate>,
    /**
    * Update timeoff params
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notes: String,
    /**
    * Update timeoff params
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
    /**
    * Update timeoff params
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateApprovedTimeoffParamsStatus>,
    /**
    * Update timeoff params
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub timeoff_days: Vec<TimeoffDaysParams>,
    /**
    * Update timeoff params
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoff_type: Option<TimeoffType>,
    /**
    * Update timeoff params
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddressCountry {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub features: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
}

/**
* The status of employment
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmploymentStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "archived")]
    Archived,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "initiated")]
    Initiated,
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmploymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmploymentStatus::Active => "active",
            EmploymentStatus::Archived => "archived",
            EmploymentStatus::Created => "created",
            EmploymentStatus::Deleted => "deleted",
            EmploymentStatus::Initiated => "initiated",
            EmploymentStatus::Invited => "invited",
            EmploymentStatus::Pending => "pending",
            EmploymentStatus::Review => "review",
            EmploymentStatus::Noop => "",
            EmploymentStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmploymentStatus {
    fn default() -> EmploymentStatus {
        EmploymentStatus::Noop
    }
}
impl EmploymentStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmploymentStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UnprocessableEntityErrorResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<String>,
}

/// TimeoffDay schema
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeoffDay {
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub day: Option<chrono::NaiveDate>,
    /**
    * The current page among all of the total_pages
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hours: i64,
    /**
    * TimeoffDay schema
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_run: Option<Company>,
}

/// Holidays response
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct HolidaysResponse {
    /**
    * Holidays response
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<Holiday>,
}

/// Selected type of payment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PricingPlanDetails {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub frequency: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Timeoff {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub approver_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cancel_reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<File>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employment_id: String,
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notes: String,
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub timeoff_days: Vec<TimeoffDay>,
    #[serde()]
    pub timeoff_type: TimeoffType,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyManager {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

/// Timeoff document params
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeoffDocumentParams {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyManagerParams {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NotFoundResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// A supported country on Remote
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Country {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub country_subdivisions: Vec<CountrySubdivision>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Description of the basic required and onboarding tasks params to create an employment.
/// You do not need to include all onboarding tasks when creating or updating an employment.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmploymentFullParams {
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_details: Option<Company>,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_details: Option<Company>,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_details: Option<Company>,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address_details: Option<Company>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_id: String,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_details: Option<Company>,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact_details: Option<Company>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_title: String,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub manager_id: String,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<Company>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub personal_email: String,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_plan_details: Option<PricingPlanDetails>,
    /**
    * Description of the basic required and onboarding tasks params to create an employment.
    *  You do not need to include all onboarding tasks when creating or updating an employment.
    *
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub provisional_start_date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyManagersResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub company_managers: Vec<CompanyManager>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub current_page: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_pages: i64,
}

/// Response schema listing many company_managers
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyManagersResponse {
    /**
    * Response schema listing many company_managers
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<CompanyManagersResponseData>,
}

/// List of countries supported by Remote API
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CountriesResponse {
    /**
    * List of countries supported by Remote API
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<Country>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyManagerCreatedResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_manager: Option<CompanyManager>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RemoteEntity {
    #[serde()]
    pub address: Address,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_internal: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
}

/// Object with required and optional fields, its descriptions and suggested presentation
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CountryFormResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Company>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MinimalCompany {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TooManyRequestsResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Holiday {
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub day: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BadRequestResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UnauthorizedResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// A subdivision of a supported country on Remote
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CountrySubdivision {
    /**
    * A subdivision of a supported country on Remote
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A subdivision of a supported country on Remote
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdivision_type: String,
}

/// Timeoff creation params
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateTimeoffParams {
    /**
    * Timeoff creation params
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<TimeoffDocumentParams>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employment_id: String,
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub end_date: Option<chrono::NaiveDate>,
    /**
    * Timeoff creation params
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notes: String,
    /**
    * The end date the payroll run is for
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub timeoff_days: Vec<TimeoffDaysParams>,
    #[serde()]
    pub timeoff_type: TimeoffType,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

/// Timeoff days params
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeoffDaysParams {
    /**
    * Timeoff days params
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub day: Option<chrono::NaiveDate>,
    /**
    * Timeoff days params
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hours: i64,
}

/// All tasks that need to be completed before marking the employment as ready
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OnboardingTasks {
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub address_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub administrative_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub bank_account_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub billing_address_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub contract_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub emergency_contact_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub employment_document_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub personal_details: TaskDescription,
    /**
    * Description and status of an onboarding task.
    */
    #[serde()]
    pub pricing_plan_details: TaskDescription,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListTimeoffResponseData {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub current_page: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub timeoffs: Vec<Timeoff>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_pages: i64,
}

/// Response schema listing many timeoffs
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListTimeoffResponse {
    /**
    * Response schema listing many timeoffs
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTimeoffResponseData>,
}

/// Required params to update an employment in the Sandbox environment.
///
/// Currently only supports setting the Employment Status to `active`.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmploymentUpdateParams {
    /**
    * Required params to update an employment in the Sandbox environment.
    *  
    *  Currently only supports setting the Employment Status to `active`.
    *
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EmploymentStatus>,
}

/// Minimal information of an employment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MinimalEmployment {
    /**
    * A supported country on Remote
    */
    #[serde()]
    pub country: Country,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_title: String,
    /**
    * The status of employment
    */
    #[serde()]
    pub status: EmploymentStatus,
}

/**
* Filter time off by its status
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetIndexTimeoffStatus {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "cancel_requested")]
    CancelRequested,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "taken")]
    Taken,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetIndexTimeoffStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetIndexTimeoffStatus::Approved => "approved",
            GetIndexTimeoffStatus::CancelRequested => "cancel_requested",
            GetIndexTimeoffStatus::Cancelled => "cancelled",
            GetIndexTimeoffStatus::Declined => "declined",
            GetIndexTimeoffStatus::Requested => "requested",
            GetIndexTimeoffStatus::Taken => "taken",
            GetIndexTimeoffStatus::Noop => "",
            GetIndexTimeoffStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetIndexTimeoffStatus {
    fn default() -> GetIndexTimeoffStatus {
        GetIndexTimeoffStatus::Noop
    }
}
impl GetIndexTimeoffStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetIndexTimeoffStatus::Noop)
    }
}

/**
* Sort order
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OrderBy {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrderBy::Asc => "asc",
            OrderBy::Desc => "desc",
            OrderBy::Noop => "",
            OrderBy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrderBy {
    fn default() -> OrderBy {
        OrderBy::Noop
    }
}
impl OrderBy {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrderBy::Noop)
    }
}

/**
* Field to sort by
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SortBy {
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "timeoff_type")]
    TimeoffType,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SortBy::Status => "status",
            SortBy::TimeoffType => "timeoff_type",
            SortBy::Noop => "",
            SortBy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SortBy {
    fn default() -> SortBy {
        SortBy::Noop
    }
}
impl SortBy {
    pub fn is_noop(&self) -> bool {
        matches!(self, SortBy::Noop)
    }
}

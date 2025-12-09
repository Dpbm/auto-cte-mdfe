use crate::types::TagName;

pub const DANFE_FLAG:u8 = 0b00000001;
pub const RAZAO_SOCIAL_FLAG:u8 = 0b00000010;
pub const SHIPPING_COMPANY_FLAG:u8 = 0b00000100;
pub const LOAD_CUBICAGE_FLAG:u8 = 0b00001000;
pub const QUANTITY_FLAG:u8 = 0b00010000;
pub const ACCESS_KEY_FLAG:u8 = 0b00100000;

pub const RAZAO_SOCIAL_BACKTRACK_FLAG:u8 = 0b00000001;
pub const SHIPPING_COMPANY_BACKTRACK_FLAG:u8 = 0b00000010;

pub const DANFE_TAG:TagName = b"nFat";
pub const LOAD_CUBICAGE_TAG:TagName = b"infCpl";
pub const QUANTITY_TAG:TagName = b"qVol";
pub const ACCESS_KEY_TAG:TagName = b"chNFe";

pub const RAZAO_SOCIAL_FIRST_TAG:TagName = b"dest";
pub const SHIPPING_COMPANY_FIRST_TAG:TagName = b"transporta";

pub const X_NOME:&[u8] = b"xNome"; // used for Razao Social and Shipping company

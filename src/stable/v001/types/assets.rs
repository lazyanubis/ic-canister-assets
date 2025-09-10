use candid::CandidType;
use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

use super::HashDigest;

// ============================== 文件数据 ==============================

// 单个文件数据
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct AssetData {
    data: Vec<u8>, // 实际数据
}

impl AssetData {
    pub fn from(_hash: &HashDigest, data: Vec<u8>) -> Self {
        Self { data }
    }
    pub fn slice(&self, _hash: &HashDigest, data_size: u64, offset: usize, size: usize) -> std::borrow::Cow<'_, [u8]> {
        assert!(offset < data_size as usize);
        let offset_end = offset + size;
        assert!(offset_end <= data_size as usize);
        std::borrow::Cow::Borrowed(&self.data[offset..offset_end])
    }
}

// 对外的路径数据 指向文件数据
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct AssetFile {
    pub path: String,
    pub created: TimestampNanos,
    pub modified: TimestampNanos,
    pub headers: Vec<(String, String)>,
    pub hash: HashDigest,
    pub size: u64,
}

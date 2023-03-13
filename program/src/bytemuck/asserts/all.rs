use borsh::BorshSerialize;
use std::fmt::Display;

use crate::{
    bytemuck::{AssertType, Assertable, RuleV2, HEADER_SECTION, SIZE_U64},
    error::RuleSetError,
};

pub struct All<'a> {
    pub size: &'a u64,
    pub rules: Vec<RuleV2<'a>>,
}

impl<'a> All<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, RuleSetError> {
        let (size, data) = bytes.split_at(SIZE_U64);
        let size = bytemuck::from_bytes::<u64>(size);

        let mut rules = Vec::with_capacity(*size as usize);
        let mut offset = 0;

        for _ in 0..*size {
            let rule = RuleV2::from_bytes(&data[offset..])?;
            offset += rule.length();
            rules.push(rule);
        }

        Ok(Self { size, rules })
    }

    pub fn serialize(rules: &[&[u8]]) -> std::io::Result<Vec<u8>> {
        // length of the assert
        let length = (SIZE_U64
            + rules
                .iter()
                .map(|v| v.len())
                .reduce(|accum, item| accum + item)
                .ok_or(RuleSetError::DataIsEmpty)
                .unwrap()) as u32;

        let mut data = Vec::with_capacity(HEADER_SECTION + length as usize);

        // Header
        // - rule type
        let rule_type = AssertType::All as u32;
        BorshSerialize::serialize(&rule_type, &mut data)?;
        // - length
        BorshSerialize::serialize(&length, &mut data)?;

        // Assert
        // - size
        let size = rules.len() as u64;
        BorshSerialize::serialize(&size, &mut data)?;
        // - rules
        rules.iter().for_each(|x| data.extend(x.iter()));

        Ok(data)
    }
}

impl<'a> Assertable<'a> for All<'a> {
    fn assert_type(&self) -> AssertType {
        AssertType::All
    }
}

impl<'a> Display for All<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("All {rules: [")?;

        for i in 0..*self.size {
            if i > 0 {
                formatter.write_str(", ")?;
            }
            formatter.write_str(&format!("{}", self.rules[i as usize]))?;
        }

        formatter.write_str("]}")
    }
}

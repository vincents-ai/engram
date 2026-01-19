use super::{GateContext, QualityGateResult};
use crate::entities::progressive_config::GateLevel;

pub struct LevelSelector;

impl LevelSelector {
    pub fn select_level<'a>(
        context: &GateContext,
        available_levels: &'a [GateLevel],
    ) -> QualityGateResult<&'a GateLevel> {
        for level in available_levels {
            if Self::matches_threshold(context, &level.threshold) {
                return Ok(level);
            }
        }

        available_levels.first().ok_or_else(|| {
            super::QualityGateError::ConfigError("No gate levels available".to_string())
        })
    }

    fn matches_threshold(
        context: &GateContext,
        threshold: &crate::entities::progressive_config::ChangeThreshold,
    ) -> bool {
        let lines_changed = context.changed_files.len() as u32 * 50;
        let files_affected = context.changed_files.len() as u32;

        lines_changed <= threshold.max_lines_changed
            && files_affected <= threshold.max_files_affected
    }
}

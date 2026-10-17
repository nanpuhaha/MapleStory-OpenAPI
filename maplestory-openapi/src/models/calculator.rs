// src/models/calculator.rs
//! Expected cost and probability calculation engines for MapleStory Starforce and Cube systems.
//!
//! Provides mathematically rigorous absorbing Markov chain formulations and exact/empirical
//! percentiles for:
//! - **Starforce**: Equipment level-based meso cost formula, 0~25 star transition matrix
//!   (success, keep, drop, destroy), event modifiers (Starcatch, 1+1, 30% discount, 5/10/15 guarantee),
//!   and safeguard (파괴방지) options.
//! - **Cubes & Potential**: Grade-up probabilities (Rare -> Epic -> Unique -> Legendary)
//!   and official ceiling (천장 / Guarantee) models for Red, Black, Additional, Master Craftsman,
//!   Meister, and Meso Potential Resets. Also calculates expected attempts and percentiles
//!   for specific target options.

use crate::error::{MapleStoryError, Result};
use serde::{Deserialize, Serialize};

/// Starforce ruleset variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StarforceRuleset {
    /// Classic MapleStory rules:
    /// - 11~14 stars: drop on failure
    /// - 15 stars: checkpoint (no drop), destruction possible
    /// - 16~19 stars: drop on failure, destruction possible
    /// - 20 stars: checkpoint (no drop), destruction possible
    /// - 21~24 stars: drop on failure, destruction possible
    Classic,
    /// Modern MapleStory rules:
    /// - 15+ stars: no drop on failure (maintain or destroy)
    Current,
}

/// Options and event modifiers for Starforce enhancement calculations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StarforceOptions {
    /// Whether Starcatch mini-game bonus is enabled (multiplicative success boost).
    pub starcatch: bool,
    /// Multiplicative multiplier when Starcatch is enabled (default 1.05 = +5% multiplicative).
    pub starcatch_multiplier: f64,
    /// 1+1 Event: Enhancing successfully at 10 stars or below (0..=10) advances 2 stars instead of 1.
    pub event_1_plus_1: bool,
    /// Sunday Maple 30% meso discount.
    pub discount_30: bool,
    /// Custom meso discount rate (0.0 to 1.0). Overridden by `discount_30` if `discount_30` is true.
    pub custom_discount_rate: f64,
    /// Safeguard (파괴방지) for 15 and 16 stars. Costs 100% extra base meso, eliminates destruction rate.
    pub safeguard: bool,
    /// Sunday 5/10/15 event: 100% success rate at 5, 10, and 15 stars.
    pub sunday_5_10_15: bool,
    /// Shining Starforce: Combination of 30% discount and 5/10/15 guarantee.
    pub shining_starforce: bool,
    /// Cost in mesos to replace destroyed equipment with a spare (default 0).
    pub spare_cost: u64,
    /// Star level to which equipment is restored upon destruction (default 12).
    pub restore_star: u32,
    /// Starforce ruleset (Classic with drops, or Current without drops at 15+).
    pub ruleset: StarforceRuleset,
    /// Number of Monte Carlo iterations for empirical percentiles (default 10,000).
    pub simulation_samples: usize,
}

impl Default for StarforceOptions {
    fn default() -> Self {
        Self {
            starcatch: true,
            starcatch_multiplier: 1.05,
            event_1_plus_1: false,
            discount_30: false,
            custom_discount_rate: 0.0,
            safeguard: false,
            sunday_5_10_15: false,
            shining_starforce: false,
            spare_cost: 0,
            restore_star: 12,
            ruleset: StarforceRuleset::Classic,
            simulation_samples: 10_000,
        }
    }
}

impl StarforceOptions {
    /// Creates a new default StarforceOptions.
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggles the Starcatch success bonus (+5% multiplicative).
    pub fn with_starcatch(mut self, enabled: bool) -> Self {
        self.starcatch = enabled;
        self
    }

    /// Sets custom Starcatch multiplier (e.g. 1.05 for 5%).
    pub fn with_starcatch_multiplier(mut self, multiplier: f64) -> Self {
        self.starcatch_multiplier = multiplier;
        self
    }

    /// Toggles the 1+1 event (advances +2 stars on success at <= 10 stars).
    pub fn with_1_plus_1(mut self, enabled: bool) -> Self {
        self.event_1_plus_1 = enabled;
        self
    }

    /// Toggles the 30% meso discount event.
    pub fn with_discount_30(mut self, enabled: bool) -> Self {
        self.discount_30 = enabled;
        self
    }

    /// Sets custom discount rate (0.0 to 1.0).
    pub fn with_custom_discount(mut self, rate: f64) -> Self {
        self.custom_discount_rate = rate;
        self
    }

    /// Toggles Safeguard (파괴방지) for 15-16 stars.
    pub fn with_safeguard(mut self, enabled: bool) -> Self {
        self.safeguard = enabled;
        self
    }

    /// Toggles Sunday 5/10/15 100% success rate event.
    pub fn with_sunday_5_10_15(mut self, enabled: bool) -> Self {
        self.sunday_5_10_15 = enabled;
        self
    }

    /// Enables Shining Starforce (30% discount + 5/10/15 guarantee).
    pub fn with_shining_starforce(mut self) -> Self {
        self.shining_starforce = true;
        self.discount_30 = true;
        self.sunday_5_10_15 = true;
        self
    }

    /// Sets the meso cost of replacing destroyed equipment with a spare item.
    pub fn with_spare_cost(mut self, spare_cost: u64) -> Self {
        self.spare_cost = spare_cost;
        self
    }

    /// Sets the restored star level upon destruction (default 12).
    pub fn with_restore_star(mut self, restore_star: u32) -> Self {
        self.restore_star = restore_star;
        self
    }

    /// Sets the ruleset variant (Classic or Current).
    pub fn with_ruleset(mut self, ruleset: StarforceRuleset) -> Self {
        self.ruleset = ruleset;
        self
    }

    /// Sets the number of simulation samples for percentile estimation.
    pub fn with_simulation_samples(mut self, samples: usize) -> Self {
        self.simulation_samples = samples;
        self
    }
}

/// Transition probabilities for a single enhancement attempt at a given star level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StarforceProbabilities {
    /// Probability of advancing to the next star (or +2 stars if 1+1 event is active).
    pub success: f64,
    /// Probability of remaining at the current star level upon failure.
    pub failure_maintain: f64,
    /// Probability of dropping 1 star upon failure.
    pub failure_drop: f64,
    /// Probability of equipment destruction upon failure.
    pub destroy: f64,
}

/// Comprehensive outcome estimate for Starforce enhancement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarforceCostEstimate {
    /// Equipment required level (e.g. 150, 160, 200).
    pub item_level: u32,
    /// Initial star level.
    pub start_star: u32,
    /// Desired target star level.
    pub target_star: u32,
    /// Expected total meso cost, including spare replacement costs if `spare_cost` > 0.
    pub expected_meso_cost: f64,
    /// Pure enhancement meso cost (excluding spare item replacement costs).
    pub expected_pure_meso: f64,
    /// Expected number of total enhancement attempts.
    pub expected_attempts: f64,
    /// Expected number of equipment destructions (booms).
    pub expected_destructions: f64,
    /// Probability of reaching `target_star` without experiencing any destruction (0.0 to 1.0).
    pub no_destruction_probability: f64,
    /// 10th percentile cost (lucky run, 10% of players spend <= this amount).
    pub percentile_10_cost: u64,
    /// 50th percentile cost (median run, 50% of players spend <= this amount).
    pub percentile_50_cost: u64,
    /// 90th percentile cost (unlucky run, 90% of players spend <= this amount).
    pub percentile_90_cost: u64,
    /// Standard deviation of total cost.
    pub std_deviation_cost: f64,
}

/// Pure domain calculation engine for Starforce enhancement.
pub struct StarforceCalculator;

impl StarforceCalculator {
    /// Calculates the cost in mesos for a single enhancement attempt.
    pub fn cost_per_attempt(item_level: u32, current_star: u32, options: &StarforceOptions) -> u64 {
        let lvl = ((item_level / 10) * 10) as f64;
        let star = current_star as f64;

        let base = if current_star < 10 {
            (lvl.powi(3) * (star + 1.0)) / 2500.0
        } else if current_star == 10 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 40000.0
        } else if current_star == 11 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 22000.0
        } else if current_star == 12 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 15000.0
        } else if current_star == 13 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 11000.0
        } else if current_star == 14 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 7500.0
        } else if current_star == 15 || current_star == 16 || current_star == 20 || current_star >= 22 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 20000.0
        } else if current_star == 17 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 15000.0
        } else if current_star == 18 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 7000.0
        } else if current_star == 19 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 4500.0
        } else if current_star == 21 {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 12500.0
        } else {
            (lvl.powi(3) * (star + 1.0).powf(2.7)) / 20000.0
        };

        let base_meso = (base.round() + 10.0) * 100.0;

        let discount_rate = if options.discount_30 || options.shining_starforce {
            0.30
        } else {
            options.custom_discount_rate.clamp(0.0, 1.0)
        };
        let discount_multiplier = 1.0 - discount_rate;

        // Safeguard adds +100% of base meso on 15 and 16 stars (not discounted by Sunday discount)
        let is_safeguard_active = options.safeguard && (current_star == 15 || current_star == 16);

        let final_cost = if is_safeguard_active {
            base_meso * discount_multiplier + base_meso * 1.0
        } else {
            base_meso * discount_multiplier
        };

        ((final_cost / 100.0).round() * 100.0) as u64
    }

    /// Retrieves transition probabilities for a single attempt at `current_star`.
    pub fn transition_probabilities(
        current_star: u32,
        options: &StarforceOptions,
    ) -> StarforceProbabilities {
        let (base_success, base_destroy, is_checkpoint) = match current_star {
            0 => (0.95, 0.0, true),
            1 => (0.90, 0.0, true),
            2 => (0.85, 0.0, true),
            3 => (0.85, 0.0, true),
            4 => (0.80, 0.0, true),
            5 => (0.75, 0.0, true),
            6 => (0.70, 0.0, true),
            7 => (0.65, 0.0, true),
            8 => (0.60, 0.0, true),
            9 => (0.55, 0.0, true),
            10 => (0.50, 0.0, true),
            11 => (0.45, 0.0, false),
            12 => (0.40, 0.0, false),
            13 => (0.35, 0.0, false),
            14 => (0.30, 0.0, false),
            15 => (0.30, 0.021, true),
            16 => (0.30, 0.021, false),
            17 => (0.30, 0.021, false),
            18 => (0.30, 0.028, false),
            19 => (0.30, 0.028, false),
            20 => (0.30, 0.070, true),
            21 => (0.30, 0.070, false),
            22 => (0.03, 0.194, false),
            23 => (0.02, 0.196, false),
            24 => (0.01, 0.198, false),
            _ => (0.01, 0.200, false),
        };

        // 1. Guarantee 5/10/15 event
        if (options.sunday_5_10_15 || options.shining_starforce)
            && (current_star == 5 || current_star == 10 || current_star == 15)
        {
            return StarforceProbabilities {
                success: 1.0,
                failure_maintain: 0.0,
                failure_drop: 0.0,
                destroy: 0.0,
            };
        }

        // 2. Starcatch adjustment
        let success = if options.starcatch {
            let mult = if options.starcatch_multiplier > 0.0 {
                options.starcatch_multiplier
            } else {
                1.05
            };
            (base_success * mult).min(1.0)
        } else {
            base_success
        };

        // 3. Safeguard adjustment
        let safeguard_applies = options.safeguard && (current_star == 15 || current_star == 16);
        let destroy = if safeguard_applies {
            0.0
        } else if base_destroy > 0.0 && base_success < 1.0 {
            let destroy_ratio = base_destroy / (1.0 - base_success);
            (1.0 - success) * destroy_ratio
        } else {
            0.0
        };

        // 4. Failure split between maintain and drop
        let fail_total = (1.0 - success - destroy).max(0.0);

        let (failure_maintain, failure_drop) = match options.ruleset {
            StarforceRuleset::Current => {
                if current_star >= 15 || is_checkpoint {
                    (fail_total, 0.0)
                } else {
                    (0.0, fail_total)
                }
            }
            StarforceRuleset::Classic => {
                if is_checkpoint {
                    (fail_total, 0.0)
                } else {
                    (0.0, fail_total)
                }
            }
        };

        StarforceProbabilities {
            success,
            failure_maintain,
            failure_drop,
            destroy,
        }
    }

    /// Calculates expected costs, attempts, destructions, and percentiles to reach `target_star`.
    pub fn calculate(
        item_level: u32,
        start_star: u32,
        target_star: u32,
        options: &StarforceOptions,
    ) -> Result<StarforceCostEstimate> {
        if start_star >= target_star {
            return Err(MapleStoryError::InvalidParameter(
                "start_star must be strictly less than target_star".to_string(),
            ));
        }
        if target_star > 25 {
            return Err(MapleStoryError::InvalidParameter(
                "target_star must be <= 25".to_string(),
            ));
        }
        if item_level == 0 {
            return Err(MapleStoryError::InvalidParameter(
                "item_level must be greater than 0".to_string(),
            ));
        }

        // Determine lowest reachable state
        let s_min = if start_star < 10 {
            0
        } else {
            options.restore_star.min(10)
        };

        let n = (target_star - s_min) as usize;
        let star_to_idx = |s: u32| (s - s_min) as usize;

        // Transition matrix I - Q for transient states
        let mut a = vec![vec![0.0; n]; n];
        for (i, row) in a.iter_mut().enumerate() {
            row[i] = 1.0;
        }
        let mut cost_vec = vec![0.0; n];
        let mut dest_vec = vec![0.0; n];

        // Also build a0 and r_target for no_destruction_probability
        let mut a0 = vec![vec![0.0; n]; n];
        for (i, row) in a0.iter_mut().enumerate() {
            row[i] = 1.0;
        }
        let mut r_target = vec![0.0; n];

        for star in s_min..target_star {
            let i = star_to_idx(star);
            let probs = Self::transition_probabilities(star, options);
            let cost = Self::cost_per_attempt(item_level, star, options) as f64;
            cost_vec[i] = cost;
            dest_vec[i] = probs.destroy;

            // Success transition
            let next_star = if options.event_1_plus_1 && star <= 10 {
                star + 2
            } else {
                star + 1
            };

            if next_star < target_star {
                let j = star_to_idx(next_star);
                a[i][j] -= probs.success;
                a0[i][j] -= probs.success;
            } else {
                r_target[i] += probs.success;
            }

            // Maintain transition
            a[i][i] -= probs.failure_maintain;
            a0[i][i] -= probs.failure_maintain;

            // Drop transition
            if probs.failure_drop > 0.0 && star > s_min {
                let j = star_to_idx(star - 1);
                a[i][j] -= probs.failure_drop;
                a0[i][j] -= probs.failure_drop;
            }

            // Destroy transition: restores to restore_star
            if probs.destroy > 0.0 {
                let restore_s = options.restore_star.min(target_star - 1).max(s_min);
                let j = star_to_idx(restore_s);
                a[i][j] -= probs.destroy;
                // in a0, destroy is an absorbing failure state, so not in transient Q0
            }
        }

        let attempts = solve_linear_system(a.clone(), vec![1.0; n])?;
        let meso_costs = solve_linear_system(a.clone(), cost_vec)?;
        let destructions = solve_linear_system(a, dest_vec)?;
        let p_no_dest = solve_linear_system(a0, r_target)?;

        let start_idx = star_to_idx(start_star);
        let expected_attempts = attempts[start_idx];
        let expected_pure_meso = meso_costs[start_idx];
        let expected_destructions = destructions[start_idx];
        let expected_meso_cost = expected_pure_meso + expected_destructions * (options.spare_cost as f64);
        let no_destruction_probability = p_no_dest[start_idx].clamp(0.0, 1.0);

        // Monte Carlo simulation for percentiles & standard deviation
        let samples = if options.simulation_samples > 0 {
            options.simulation_samples
        } else {
            10_000
        };

        let mut rng = SimpleRng::new(42);
        let mut costs = Vec::with_capacity(samples);

        for _ in 0..samples {
            let mut star = start_star;
            let mut run_cost = 0u64;
            while star < target_star {
                let cost = Self::cost_per_attempt(item_level, star, options);
                run_cost += cost;
                let probs = Self::transition_probabilities(star, options);
                let r = rng.next_f64();
                if r < probs.success {
                    if options.event_1_plus_1 && star <= 10 {
                        star = (star + 2).min(target_star);
                    } else {
                        star += 1;
                    }
                } else if r < probs.success + probs.destroy {
                    run_cost += options.spare_cost;
                    star = options.restore_star.min(target_star - 1).max(s_min);
                } else if r < probs.success + probs.destroy + probs.failure_drop {
                    if star > s_min {
                        star -= 1;
                    }
                } else {
                    // maintain
                }
            }
            costs.push(run_cost);
        }

        costs.sort_unstable();
        let percentile_10_cost = costs[(samples * 10 / 100).min(samples - 1)];
        let percentile_50_cost = costs[(samples * 50 / 100).min(samples - 1)];
        let percentile_90_cost = costs[(samples * 90 / 100).min(samples - 1)];

        let mean = costs.iter().map(|&c| c as f64).sum::<f64>() / (samples as f64);
        let variance = costs
            .iter()
            .map(|&c| {
                let diff = (c as f64) - mean;
                diff * diff
            })
            .sum::<f64>()
            / (samples as f64);
        let std_deviation_cost = variance.sqrt();

        Ok(StarforceCostEstimate {
            item_level,
            start_star,
            target_star,
            expected_meso_cost,
            expected_pure_meso,
            expected_attempts,
            expected_destructions,
            no_destruction_probability,
            percentile_10_cost,
            percentile_50_cost,
            percentile_90_cost,
            std_deviation_cost,
        })
    }
}

/// Convenience standalone function for Starforce expected cost calculation.
pub fn calculate_starforce_cost(
    item_level: u32,
    start_star: u32,
    target_star: u32,
    options: &StarforceOptions,
) -> Result<StarforceCostEstimate> {
    StarforceCalculator::calculate(item_level, start_star, target_star, options)
}

/// Potential grades for MapleStory equipment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PotentialGrade {
    /// Rare (레어)
    Rare = 1,
    /// Epic (에픽)
    Epic = 2,
    /// Unique (유니크)
    Unique = 3,
    /// Legendary (레전드리)
    Legendary = 4,
}

impl PotentialGrade {
    /// Parses a potential grade from Korean or English strings.
    pub fn parse_korean(s: &str) -> Option<Self> {
        match s.trim() {
            "레어" | "Rare" | "rare" => Some(PotentialGrade::Rare),
            "에픽" | "Epic" | "epic" => Some(PotentialGrade::Epic),
            "유니크" | "Unique" | "unique" => Some(PotentialGrade::Unique),
            "레전드리" | "Legendary" | "legendary" => Some(PotentialGrade::Legendary),
            _ => None,
        }
    }

    /// Returns the Korean representation of the potential grade.
    pub fn as_korean(&self) -> &'static str {
        match self {
            PotentialGrade::Rare => "레어",
            PotentialGrade::Epic => "에픽",
            PotentialGrade::Unique => "유니크",
            PotentialGrade::Legendary => "레전드리",
        }
    }
}

/// Types of cubes or potential reset mechanisms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CubeType {
    /// Red Cube (레드 큐브, Cash 1,200 KRW)
    Red,
    /// Black Cube (블랙 큐브, Cash 2,200 KRW)
    Black,
    /// Additional Cube (에디셔널 큐브, Cash 2,400 KRW)
    Additional,
    /// Master Craftsman's Cube (장인의 큐브, in-game meso / crafting)
    MasterCraftsman,
    /// Meister's Cube (명장의 큐브, in-game meso / crafting)
    Meister,
    /// Occult Cube (수상한 큐브, in-game drop)
    Occult,
    /// Karma Cube / Meso Potential Reset (카르마 큐브 / 메소 잠재능력 재설정)
    KarmaMeso,
}

/// Intermediate step estimate for a single grade-up transition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CubeGradeStepEstimate {
    /// Starting grade for this transition step.
    pub from_grade: PotentialGrade,
    /// Ending grade for this transition step.
    pub to_grade: PotentialGrade,
    /// Probability of grading up on each attempt.
    pub probability: f64,
    /// Expected number of cubes required for this step.
    pub expected_cubes: f64,
    /// Ceiling guarantee threshold (maximum attempts before guaranteed success), if applicable.
    pub ceiling_cubes: Option<u32>,
    /// Cost per cube attempt (meso or cash).
    pub cost_per_attempt: u64,
    /// Expected cost for this step.
    pub expected_cost: f64,
}

/// Overall estimate for potential grade-up via cubes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CubeGradeUpEstimate {
    /// Cube type used.
    pub cube_type: CubeType,
    /// Starting potential grade.
    pub from_grade: PotentialGrade,
    /// Desired target potential grade.
    pub target_grade: PotentialGrade,
    /// Expected total number of cubes required.
    pub expected_cubes: f64,
    /// Expected total cost (Cash KRW or Mesos depending on `is_cash`).
    pub expected_cost: f64,
    /// True if currency is Cash (KRW), false if Mesos.
    pub is_cash: bool,
    /// Cumulative ceiling cubes (maximum guaranteed attempts across all steps).
    pub ceiling_cubes: Option<u32>,
    pub ceiling_cost: Option<u64>,
    /// 10th percentile cubes (lucky attempts).
    pub percentile_10_cubes: u32,
    /// 50th percentile cubes (median attempts).
    pub percentile_50_cubes: u32,
    /// 90th percentile cubes (unlucky attempts).
    pub percentile_90_cubes: u32,
    /// Detailed breakdown per grade transition step.
    pub step_details: Vec<CubeGradeStepEstimate>,
}

/// Cost and attempt estimate for rolling a specific target potential option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetOptionCostEstimate {
    /// Target option appearance probability per roll.
    pub target_probability: f64,
    /// Expected number of attempts (1 / p).
    pub expected_attempts: f64,
    /// Expected total cost.
    pub expected_cost: f64,
    /// 10th percentile attempts (lucky rolls).
    pub percentile_10_attempts: u32,
    /// 50th percentile (median) attempts.
    pub percentile_50_attempts: u32,
    /// 90th percentile attempts.
    pub percentile_90_attempts: u32,
    /// 99th percentile attempts.
    pub percentile_99_attempts: u32,
    /// Cost at 10th percentile attempts.
    pub cost_at_percentile_10: u64,
    /// Cost at 50th percentile attempts.
    pub cost_at_percentile_50: u64,
    /// Cost at 90th percentile attempts.
    pub cost_at_percentile_90: u64,
}

/// Pure domain calculation engine for MapleStory Cube and Potential systems.
pub struct CubeCalculator;

impl CubeCalculator {
    /// Retrieves official grade-up probability for `cube_type` from `from_grade`.
    pub fn grade_up_probability(cube_type: CubeType, from_grade: PotentialGrade) -> Option<f64> {
        match (cube_type, from_grade) {
            (CubeType::Red, PotentialGrade::Rare) => Some(0.06),
            (CubeType::Red, PotentialGrade::Epic) => Some(0.018),
            (CubeType::Red, PotentialGrade::Unique) => Some(0.003),

            (CubeType::Black, PotentialGrade::Rare) => Some(0.15),
            (CubeType::Black, PotentialGrade::Epic) => Some(0.035),
            (CubeType::Black, PotentialGrade::Unique) => Some(0.014),

            (CubeType::Additional, PotentialGrade::Rare) => Some(0.047619),
            (CubeType::Additional, PotentialGrade::Epic) => Some(0.019608),
            (CubeType::Additional, PotentialGrade::Unique) => Some(0.007),

            (CubeType::MasterCraftsman, PotentialGrade::Rare) => Some(0.047619),
            (CubeType::MasterCraftsman, PotentialGrade::Epic) => Some(0.011858),
            (CubeType::MasterCraftsman, PotentialGrade::Unique) => None,

            (CubeType::Meister, PotentialGrade::Rare) => Some(0.079994),
            (CubeType::Meister, PotentialGrade::Epic) => Some(0.016959),
            (CubeType::Meister, PotentialGrade::Unique) => Some(0.001996),

            (CubeType::Occult, PotentialGrade::Rare) => Some(0.009901),
            (CubeType::Occult, PotentialGrade::Epic) => None,
            (CubeType::Occult, PotentialGrade::Unique) => None,

            (CubeType::KarmaMeso, PotentialGrade::Rare) => Some(0.15),
            (CubeType::KarmaMeso, PotentialGrade::Epic) => Some(0.035),
            (CubeType::KarmaMeso, PotentialGrade::Unique) => Some(0.014),

            _ => None,
        }
    }

    /// Retrieves official ceiling (guaranteed grade-up attempt limit) for `cube_type`.
    pub fn ceiling(cube_type: CubeType, from_grade: PotentialGrade) -> Option<u32> {
        match (cube_type, from_grade) {
            (CubeType::Red, PotentialGrade::Rare) => Some(25),
            (CubeType::Red, PotentialGrade::Epic) => Some(83),
            (CubeType::Red, PotentialGrade::Unique) => Some(500),

            (CubeType::Black, PotentialGrade::Rare) => Some(10),
            (CubeType::Black, PotentialGrade::Epic) => Some(42),
            (CubeType::Black, PotentialGrade::Unique) => Some(107),

            (CubeType::Additional, PotentialGrade::Rare) => Some(31),
            (CubeType::Additional, PotentialGrade::Epic) => Some(76),
            (CubeType::Additional, PotentialGrade::Unique) => Some(214),

            (CubeType::KarmaMeso, PotentialGrade::Rare) => Some(10),
            (CubeType::KarmaMeso, PotentialGrade::Epic) => Some(42),
            (CubeType::KarmaMeso, PotentialGrade::Unique) => Some(107),

            _ => None,
        }
    }

    /// Retrieves default cost per cube attempt and whether it is Cash or Meso.
    pub fn default_cost_per_attempt(cube_type: CubeType) -> (u64, bool) {
        match cube_type {
            CubeType::Red => (1200, true),
            CubeType::Black => (2200, true),
            CubeType::Additional => (2400, true),
            CubeType::MasterCraftsman => (5_000_000, false),
            CubeType::Meister => (10_000_000, false),
            CubeType::Occult => (500_000, false),
            CubeType::KarmaMeso => (25_000_000, false),
        }
    }

    /// Calculates expected cubes, costs, and ceiling limits to advance from `from_grade` to `target_grade`.
    pub fn calculate(
        cube_type: CubeType,
        from_grade: PotentialGrade,
        target_grade: PotentialGrade,
        custom_cost_per_attempt: Option<u64>,
    ) -> Result<CubeGradeUpEstimate> {
        if from_grade >= target_grade {
            return Err(MapleStoryError::InvalidParameter(
                "from_grade must be strictly lower than target_grade".to_string(),
            ));
        }

        let (default_cost, is_cash) = Self::default_cost_per_attempt(cube_type);
        let cost_per_cube = custom_cost_per_attempt.unwrap_or(default_cost);

        let mut current = from_grade;
        let mut step_details = Vec::new();
        let mut total_expected_cubes = 0.0;
        let mut total_expected_cost = 0.0;
        let mut total_ceiling_cubes: Option<u32> = Some(0);

        while current < target_grade {
            let next = match current {
                PotentialGrade::Rare => PotentialGrade::Epic,
                PotentialGrade::Epic => PotentialGrade::Unique,
                PotentialGrade::Unique => PotentialGrade::Legendary,
                PotentialGrade::Legendary => break,
            };

            let prob = Self::grade_up_probability(cube_type, current).ok_or_else(|| {
                MapleStoryError::InvalidParameter(format!(
                    "{:?} cannot grade up from {:?} to {:?}",
                    cube_type, current, next
                ))
            })?;
            let ceil = Self::ceiling(cube_type, current);

            // E[X] for geometric with ceiling C: (1 - (1 - p)^C) / p
            let exp_cubes = match ceil {
                Some(c) => (1.0 - (1.0 - prob).powi(c as i32)) / prob,
                None => 1.0 / prob,
            };
            let exp_cost = exp_cubes * (cost_per_cube as f64);

            if let (Some(total_c), Some(c)) = (total_ceiling_cubes, ceil) {
                total_ceiling_cubes = Some(total_c + c);
            } else {
                total_ceiling_cubes = None;
            }

            total_expected_cubes += exp_cubes;
            total_expected_cost += exp_cost;

            step_details.push(CubeGradeStepEstimate {
                from_grade: current,
                to_grade: next,
                probability: prob,
                expected_cubes: exp_cubes,
                ceiling_cubes: ceil,
                cost_per_attempt: cost_per_cube,
                expected_cost: exp_cost,
            });

            current = next;
        }

        let total_ceiling_cost = total_ceiling_cubes.map(|c| (c as u64) * cost_per_cube);

        // Calculate percentiles across steps by convolving PMFs
        let (percentile_10_cubes, percentile_50_cubes, percentile_90_cubes) =
            calculate_cube_percentiles(&step_details);

        Ok(CubeGradeUpEstimate {
            cube_type,
            from_grade,
            target_grade,
            expected_cubes: total_expected_cubes,
            expected_cost: total_expected_cost,
            is_cash,
            ceiling_cubes: total_ceiling_cubes,
            ceiling_cost: total_ceiling_cost,
            percentile_10_cubes,
            percentile_50_cubes,
            percentile_90_cubes,
            step_details,
        })
    }

    /// Calculates expected cost and attempt percentiles for rolling a target potential option.
    pub fn calculate_target_option(
        target_probability: f64,
        cost_per_attempt: u64,
    ) -> Result<TargetOptionCostEstimate> {
        if target_probability <= 0.0 || target_probability > 1.0 {
            return Err(MapleStoryError::InvalidParameter(
                "target_probability must be in the range (0.0, 1.0]".to_string(),
            ));
        }

        let expected_attempts = 1.0 / target_probability;
        let expected_cost = expected_attempts * (cost_per_attempt as f64);

        let q_fail = 1.0 - target_probability;
        let percentile_10_attempts = if q_fail > 0.0 {
            (0.9f64.ln() / q_fail.ln()).ceil().max(1.0) as u32
        } else {
            1
        };
        let percentile_50_attempts = if q_fail > 0.0 {
            (0.5f64.ln() / q_fail.ln()).ceil().max(1.0) as u32
        } else {
            1
        };
        let percentile_90_attempts = if q_fail > 0.0 {
            (0.1f64.ln() / q_fail.ln()).ceil().max(1.0) as u32
        } else {
            1
        };
        let percentile_99_attempts = if q_fail > 0.0 {
            (0.01f64.ln() / q_fail.ln()).ceil().max(1.0) as u32
        } else {
            1
        };

        let cost_at_percentile_10 = (percentile_10_attempts as u64) * cost_per_attempt;
        let cost_at_percentile_50 = (percentile_50_attempts as u64) * cost_per_attempt;
        let cost_at_percentile_90 = (percentile_90_attempts as u64) * cost_per_attempt;

        Ok(TargetOptionCostEstimate {
            target_probability,
            expected_attempts,
            expected_cost,
            percentile_10_attempts,
            percentile_50_attempts,
            percentile_90_attempts,
            percentile_99_attempts,
            cost_at_percentile_10,
            cost_at_percentile_50,
            cost_at_percentile_90,
        })
    }
}

/// Computes the exact 10th, 50th, and 90th percentiles for multi-step cube transitions via PMF convolution.
fn calculate_cube_percentiles(steps: &[CubeGradeStepEstimate]) -> (u32, u32, u32) {
    if steps.is_empty() {
        return (0, 0, 0);
    }

    // Step PMF constructor: P(X = k) for k in 1..=max_k
    let build_pmf = |step: &CubeGradeStepEstimate| -> Vec<f64> {
        let p = step.probability;
        let max_k = step.ceiling_cubes.unwrap_or_else(|| {
            // If no ceiling, truncate at 99.9% probability
            ((0.001f64.ln() / (1.0 - p).ln()).ceil().max(1.0) as u32).min(5000)
        }) as usize;

        let mut pmf = vec![0.0; max_k + 1]; // 1-indexed
        let mut fail_pow = 1.0;
        for item in pmf[1..max_k].iter_mut() {
            *item = fail_pow * p;
            fail_pow *= 1.0 - p;
        }
        // At max_k (ceiling or truncated tail), absorb remaining probability
        if step.ceiling_cubes.is_some() {
            pmf[max_k] = fail_pow;
        } else {
            pmf[max_k] = fail_pow * p;
        }
        pmf
    };

    // Convolve PMFs across all steps
    let mut current_pmf = build_pmf(&steps[0]);
    for step in &steps[1..] {
        let next_pmf = build_pmf(step);
        let new_len = current_pmf.len() + next_pmf.len() - 1;
        let mut convolved = vec![0.0; new_len];
        for i in 1..current_pmf.len() {
            if current_pmf[i] == 0.0 {
                continue;
            }
            for j in 1..next_pmf.len() {
                convolved[i + j] += current_pmf[i] * next_pmf[j];
            }
        }
        current_pmf = convolved;
    }

    // Find 10th, 50th, and 90th percentile from CDF
    let mut cdf = 0.0;
    let mut p10 = 1;
    let mut p50 = 1;
    let mut p90 = 1;
    let mut found_10 = false;
    let mut found_50 = false;
    let mut found_90 = false;

    for (k, &prob) in current_pmf.iter().enumerate() {
        cdf += prob;
        if !found_10 && cdf >= 0.1 {
            p10 = k as u32;
            found_10 = true;
        }
        if !found_50 && cdf >= 0.5 {
            p50 = k as u32;
            found_50 = true;
        }
        if !found_90 && cdf >= 0.9 {
            p90 = k as u32;
            found_90 = true;
            break;
        }
    }

    if !found_10 {
        p10 = 1;
    }
    if !found_50 {
        p50 = current_pmf.len().saturating_sub(1) as u32;
    }
    if !found_90 {
        p90 = current_pmf.len().saturating_sub(1) as u32;
    }

    (p10, p50, p90)
}

/// Gaussian elimination with partial pivoting to solve A x = b.
#[allow(clippy::needless_range_loop)]
fn solve_linear_system(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>> {
    let n = b.len();
    for i in 0..n {
        let mut max_row = i;
        let mut max_val = a[i][i].abs();
        for k in (i + 1)..n {
            let val = a[k][i].abs();
            if val > max_val {
                max_val = val;
                max_row = k;
            }
        }
        if max_val < 1e-12 {
            return Err(MapleStoryError::InvalidParameter(
                "Singular matrix in Markov chain system".to_string(),
            ));
        }
        if max_row != i {
            a.swap(i, max_row);
            b.swap(i, max_row);
        }

        let pivot = a[i][i];
        for k in (i + 1)..n {
            let factor = a[k][i] / pivot;
            a[k][i] = 0.0;
            for j in (i + 1)..n {
                a[k][j] -= factor * a[i][j];
            }
            b[k] -= factor * b[i];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }
    Ok(x)
}

/// Lightweight, deterministic 64-bit pseudo-random number generator for simulations.
struct SimpleRng(u64);

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0xdeadbeefcafe } else { seed })
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }
}

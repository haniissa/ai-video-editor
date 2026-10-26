pub mod edl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EditSegment {
    pub start: f64,
    pub end: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EditDecisionList {
    pub segments: Vec<EditSegment>,
}

#[derive(Debug, PartialEq)]
pub struct TimelineSegment {
    pub source_start: f64,
    pub source_end: f64,
    pub output_start: f64,
    pub output_end: f64,
}

impl EditSegment {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.start < 0.0 {
            anyhow::bail!("Segment start cannot be negative");
        }
        if self.end <= self.start {
            anyhow::bail!("Segment end must be greater than start");
        }

        if !self.start.is_finite() || !self.end.is_finite() {
            anyhow::bail!("Segment timestamp must be finite numbers");
        }

        Ok(())
    }
}

impl EditDecisionList {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.segments.is_empty() {
            anyhow::bail!("Edit decision list cannot be empty");
        }
        for segment in &self.segments {
            segment.validate()?;
        }

        //Sort references by start time without changing the original list
        let mut sorted_segments: Vec<&EditSegment> = self.segments.iter().collect();

        sorted_segments.sort_by(|a, b| a.start.total_cmp(&b.start));

        //Then, check that segments do not overlap
        for pair in sorted_segments.windows(2) {
            let current = &pair[0];
            let next = &pair[1];

            if next.start < current.end {
                anyhow::bail!("Edit segments cannot overlap");
            }
        }
        Ok(())
    }
    pub fn total_duration(&self) -> f64 {
        self.segments
            .iter()
            .map(|segment| segment.end - segment.start)
            .sum()
    }
    pub fn build_timeline(&self) -> anyhow::Result<Vec<TimelineSegment>> {
        self.validate()?;

        let mut segments: Vec<&EditSegment> = self.segments.iter().collect();
        segments.sort_by(|a, b| a.start.total_cmp(&b.start));

        let mut output_time = 0.0;
        let mut timeline = Vec::new();

        for segment in segments {
            let duration = segment.end - segment.start;

            timeline.push(TimelineSegment {
                source_start: segment.start,
                source_end: segment.end,
                output_start: output_time,
                output_end: output_time + duration,
            });
            output_time += duration;
        }
        Ok(timeline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_segment_passes() {
        let segment = EditSegment {
            start: 0.0,
            end: 5.0,
        };

        assert!(segment.validate().is_ok());
    }

    #[test]
    fn segment_with_end_before_start_fails() {
        let segment = EditSegment {
            start: 10.0,
            end: 5.0,
        };
        assert!(segment.validate().is_err());
    }

    #[test]
    fn empty_edit_list_fails() {
        let list = EditDecisionList { segments: vec![] };

        assert!(list.validate().is_err());
    }

    #[test]
    fn valid_edit_list_passes() {
        let list = EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
                EditSegment {
                    start: 10.0,
                    end: 15.0,
                },
            ],
        };

        assert!(list.validate().is_ok());
    }

    #[test]
    fn overlapping_segments_fail() {
        let list = EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
                EditSegment {
                    start: 4.0,
                    end: 8.0,
                },
            ],
        };
        assert!(list.validate().is_err());
    }

    #[test]
    fn out_of_order_non_overlapping_segments_pass() {
        let list = EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 10.0,
                    end: 15.0,
                },
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
            ],
        };

        assert!(list.validate().is_ok());
    }
    #[test]
    fn out_of_order_overlapping_segments_fail() {
        let list = EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 5.0,
                    end: 12.0,
                },
                EditSegment {
                    start: 0.0,
                    end: 8.0,
                },
            ],
        };
        assert!(list.validate().is_err());
    }

    #[test]
    fn total_duration_calculates_selected_segments() {
        let list = EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
                EditSegment {
                    start: 10.0,
                    end: 15.0,
                },
            ],
        };
        assert_eq!(list.total_duration(), 10.0);
    }

    #[test]
    fn total_duration_excludes_gaps_between_segments() {
        let list = EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
                EditSegment {
                    start: 10.0,
                    end: 15.0,
                },
                EditSegment {
                    start: 20.0,
                    end: 23.0,
                },
            ],
        };
        assert_eq!(list.total_duration(), 13.0);
    }

    #[test]
    fn timeline_removes_gaps_between_segments() {
        let list = EditDecisionList {
            segments: vec![
                EditSegment {
                    start: 10.0,
                    end: 15.0,
                },
                EditSegment {
                    start: 0.0,
                    end: 5.0,
                },
            ],
        };
        let timeline = list.build_timeline().unwrap();

        assert_eq!(timeline.len(), 2);
        assert_eq!(timeline[0].source_start, 0.0);
        assert_eq!(timeline[0].output_start, 0.0);
        assert_eq!(timeline[0].output_end, 5.0);
        assert_eq!(timeline[1].source_start, 10.0);
        assert_eq!(timeline[1].output_start, 5.0);
        assert_eq!(timeline[1].output_end, 10.0);
    }
}

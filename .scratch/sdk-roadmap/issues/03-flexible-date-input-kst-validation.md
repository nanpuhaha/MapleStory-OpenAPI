# 03: Support flexible date inputs (Into<MapleDate>) and enforce MIN_DATE / KST boundaries

**What to build:** All date-accepting SDK methods accept an ergonomic `Into<MapleDate>` parameter accommodating raw `&str`, `chrono::NaiveDate`, and `chrono::DateTime<Utc>`, while locally enforcing `MIN_DATE (2023-12-21)` and NEXON's 01:00 AM KST daily refresh boundary to fail fast before hitting the remote API.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] Define a `MapleDate` type that implements `From<&str>`, `From<String>`, `From<NaiveDate>`, and `From<DateTime<Tz>>`.
- [x] Provide helper factories like `MapleDate::latest_available()` which calculates whether yesterday or the day before yesterday is available based on current KST time and the 01:00 AM refresh window.
- [x] `MapleStoryClient::validate_date` enforces `date >= MIN_DATE` and rejects future dates in KST, producing a structured `MapleStoryError::DateOutOfRange`.
- [x] Update client methods taking `date: Option<&str>` to accept `impl Into<Option<MapleDate>>` (or `impl Into<MapleDate>`) while preserving backward compatibility for string slices.
- [x] Unit tests verify conversions from diverse date types, `MIN_DATE` boundary checks, and KST time calculations.

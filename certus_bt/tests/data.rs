use certus_bt::data::HistoricBarConsolidationModel;
use certus_core::data::{Bar, MarketData};
use chrono::{NaiveDate, NaiveDateTime};

fn create_bar(date: NaiveDateTime, open: f64, high: f64, low: f64, close: f64, volume: f64) -> MarketData {
    MarketData::Bar(Bar { date, open, high, low, close, volume })
}

#[test]
fn test_normal_consolidation_1_to_5_min() {
    let model = HistoricBarConsolidationModel::new(1, 5);
    let data = vec![
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap(), 100.0, 101.0, 99.0, 100.5, 1000.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,1,0).unwrap(), 100.5, 102.0, 100.0, 101.5, 1100.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,2,0).unwrap(), 101.5, 103.0, 101.0, 102.5, 1200.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,3,0).unwrap(), 102.5, 104.0, 102.0, 103.5, 1300.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,4,0).unwrap(), 103.5, 105.0, 103.0, 104.5, 1400.0),
    ];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 1);
    if let MarketData::Bar(bar) = &consolidated[0] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.high, 105.0);
        assert_eq!(bar.low, 99.0);
        assert_eq!(bar.close, 104.5);
        assert_eq!(bar.volume, 6000.0);
    }
}

#[test]
fn test_consolidation_with_missing_bars() {
    let model = HistoricBarConsolidationModel::new(1, 5);
    // Missing 9:1 and 9:3
    let data = vec![
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap(), 100.0, 101.0, 99.0, 100.5, 1000.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,2,0).unwrap(), 101.5, 103.0, 101.0, 102.5, 1200.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,4,0).unwrap(), 103.5, 105.0, 103.0, 104.5, 1400.0),
    ];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 1);
    if let MarketData::Bar(bar) = &consolidated[0] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.high, 105.0);
        assert_eq!(bar.low, 99.0);
        assert_eq!(bar.close, 104.5);
        assert_eq!(bar.volume, 3600.0);
    }
}

#[test]
fn test_consolidation_late_start() {
    let model = HistoricBarConsolidationModel::new(1, 5);
    // Starts at 9:2, not 9:0
    let data = vec![
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,2,0).unwrap(), 101.5, 103.0, 101.0, 102.5, 1200.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,3,0).unwrap(), 102.5, 104.0, 102.0, 103.5, 1300.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,4,0).unwrap(), 103.5, 105.0, 103.0, 104.5, 1400.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,5,0).unwrap(), 104.5, 106.0, 104.0, 105.5, 1500.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,6,0).unwrap(), 105.5, 107.0, 105.0, 106.5, 1600.0),
    ];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 2); // Buckets at 9:0 and 9:5
    if let MarketData::Bar(bar) = &consolidated[0] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        assert_eq!(bar.open, 101.5);
        assert_eq!(bar.high, 105.0);
        assert_eq!(bar.low, 101.0);
        assert_eq!(bar.close, 104.5);
        assert_eq!(bar.volume, 3900.0);
    }
    if let MarketData::Bar(bar) = &consolidated[1] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,5,0).unwrap());
        assert_eq!(bar.open, 104.5);
        assert_eq!(bar.high, 107.0);
        assert_eq!(bar.low, 104.0);
        assert_eq!(bar.close, 106.5);
        assert_eq!(bar.volume, 3100.0);
    }
}

#[test]
fn test_consolidation_multiple_buckets() {
    let model = HistoricBarConsolidationModel::new(1, 5);
    let data = vec![
        // First bucket 9:0-9:4
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap(), 100.0, 101.0, 99.0, 100.5, 1000.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,1,0).unwrap(), 100.5, 102.0, 100.0, 101.5, 1100.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,2,0).unwrap(), 101.5, 103.0, 101.0, 102.5, 1200.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,3,0).unwrap(), 102.5, 104.0, 102.0, 103.5, 1300.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,4,0).unwrap(), 103.5, 105.0, 103.0, 104.5, 1400.0),
        // Second bucket 9:5-9:9
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,5,0).unwrap(), 104.5, 106.0, 104.0, 105.5, 1500.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,6,0).unwrap(), 105.5, 107.0, 105.0, 106.5, 1600.0),
    ];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 2);
    // First bar
    if let MarketData::Bar(bar) = &consolidated[0] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.close, 104.5);
    }
    // Second bar
    if let MarketData::Bar(bar) = &consolidated[1] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,5,0).unwrap());
        assert_eq!(bar.open, 104.5);
        assert_eq!(bar.close, 106.5);
    }
}

#[test]
fn test_consolidation_empty_data() {
    let model = HistoricBarConsolidationModel::new(1, 5);
    let data: Vec<MarketData> = vec![];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 0);
}

#[test]
fn test_consolidation_single_bar() {
    let model = HistoricBarConsolidationModel::new(1, 5);
    let data = vec![
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap(), 100.0, 101.0, 99.0, 100.5, 1000.0),
    ];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 1);
    if let MarketData::Bar(bar) = &consolidated[0] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.high, 101.0);
        assert_eq!(bar.low, 99.0);
        assert_eq!(bar.close, 100.5);
        assert_eq!(bar.volume, 1000.0);
    }
}

#[test]
fn test_consolidation_unordered_bars() {
    let model = HistoricBarConsolidationModel::new(1, 5);
    let data = vec![
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,4,0).unwrap(), 103.5, 105.0, 103.0, 104.5, 1400.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap(), 100.0, 101.0, 99.0, 100.5, 1000.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,2,0).unwrap(), 101.5, 103.0, 101.0, 102.5, 1200.0),
    ];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 1);
    if let MarketData::Bar(bar) = &consolidated[0] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        assert_eq!(bar.open, 100.0); // First in sorted order
        assert_eq!(bar.close, 104.5); // Last in sorted order
    }
}

#[test]
fn test_consolidation_different_timeframes() {
    let model = HistoricBarConsolidationModel::new(5, 15); // 5-min to 15-min
    let data = vec![
        // First 15-min bucket: 9:0 to 9:14
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap(), 100.0, 101.0, 99.0, 100.5, 1000.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,5,0).unwrap(), 100.5, 102.0, 100.0, 101.5, 1100.0),
        create_bar(NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,10,0).unwrap(), 101.5, 103.0, 101.0, 102.5, 1200.0),
    ];
    let consolidated = model.consolidate_bars(&data);
    assert_eq!(consolidated.len(), 1);
    if let MarketData::Bar(bar) = &consolidated[0] {
        assert_eq!(bar.date, NaiveDate::from_ymd_opt(2023,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.high, 103.0);
        assert_eq!(bar.low, 99.0);
        assert_eq!(bar.close, 102.5);
        assert_eq!(bar.volume, 3300.0);
    }
}

#[test]
#[should_panic(expected = "output_minutes (7) must be a multiple of input_minutes (5)")]
fn test_invalid_timeframe_multiples() {
    HistoricBarConsolidationModel::new(5, 7);
}
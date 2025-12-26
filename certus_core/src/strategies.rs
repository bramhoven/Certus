
/// trait for trading strategies
/// init and next methods should be implemented
pub trait Strategy {
    /// Initializes a strategy and allows for setting indicators up
    fn init(&mut self);

    /// Will be called for every new bar or tick, when tick data is supplied
    fn next(&mut self);
}
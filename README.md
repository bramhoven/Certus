# Certus

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/certus/ci.yml)](https://github.com/yourusername/certus/actions)

A high-performance, event-driven trading execution and backtesting engine written in Rust. Certus empowers traders and developers to test, optimize, and validate trading strategies in a simulated environment before deploying them live, ensuring reliability and performance.

> **Note:** This project is a work in progress (W.I.P.). Features and APIs may change.

## 🚀 Features

- **Event-Driven Architecture**: Built on an asynchronous event loop for handling market data, order execution, and strategy signals in real-time.
- **Backtesting Engine**: Simulate trading strategies against historical data with high fidelity.
- **Live Execution**: Seamlessly transition validated strategies to live trading environments.
- **Performance Optimized**: Leveraging Rust's zero-cost abstractions for low-latency execution.
- **Modular Design**: Composed of independent crates for core functionality, backtesting, and live trading.
- **Extensible**: Easy to integrate custom data sources, indicators, and execution handlers.

## 🏗️ Architecture

Certus is organized into three main crates:

- **`certus_core`**: The foundational library providing core types, event handling, and shared utilities.
- **`certus_bt`**: Backtesting functionality for simulating strategies against historical market data.
- **`certus_live`**: Live trading execution module for connecting to brokers and executing orders in real-time.

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


---

*Certus: Precision in Trading, Powered by Rust.*
# Nekoflow

Nekoflow is a lightweight, extensible Rust library for building and running data pipelines. It provides a simple builder-pattern API to compose sources, processors, and destinations into reusable and testable pipelines.

## Features

- Declarative pipeline construction with `PipelineBuilder`.
- Async execution powered by Tokio.
- Custom sources, processors, and destinations.
- Context propagation across pipeline stages.
- Macro-generated support for pipelines with up to 16 processors.

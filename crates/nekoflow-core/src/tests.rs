use crate::context::Context;
use crate::destination::Destination;
use crate::error::Result;
use crate::executor::ExecutorTrait;
use crate::pipeline::PipelineBuilder;
use crate::processor::{FnProcessor, Processor};
use crate::source::Source;
struct TestSource {}
impl Source for TestSource {
  type Send = String;
  async fn get(&mut self, _ctx: Context) -> Result<Option<String>> {
    Ok(Some(String::from("Hello, nekoflow!")))
  }
}

struct TestDestination {}
impl Destination for TestDestination {
  type Recv = String;
  async fn recv(&mut self, payload: Self::Recv, _ctx: Context) -> Result {
    println!("{}", payload);
    Ok(())
  }
}

#[derive(Clone)]
struct TestProcessor {}

impl Processor for TestProcessor {
  type Recv = String;
  type Send = String;
  async fn process(&self, payload: Self::Recv, _ctx: Context) -> Result<Self::Send> {
    Ok(payload)
  }
}

#[test]
fn test_pipeline() {
  let source = TestSource {};
  let destination = TestDestination {};
  let processor = TestProcessor {};
  let pipeline = PipelineBuilder::new("test")
    .source(source)
    .processor(processor.clone())
    .processor(processor)
    .destination(destination)
    .build();
  let mut executor = crate::executor::Executor::new(pipeline, Context::new());
  executor.execute();
}

#[test]
fn test_flow() {
  let source = TestSource {};
  let destination = TestDestination {};
  let processor = TestProcessor {};
  let pipeline = PipelineBuilder::new("test")
    .source(source)
    .processor(FnProcessor::new(|x| x))
    .processor(processor)
    .destination(destination)
    .build();
  let mut flow = crate::flow::Flow::new();
  flow.add_pipeline(pipeline);
  flow.run();
}

use crate::context::Context;
use crate::destination::Destination;
use crate::error::Result;
use crate::pipeline::PipelineBuilder;
use crate::processor::Processor;
use crate::source::Source;
struct TestSource {}
impl Source for TestSource {
  type Send = String;
  fn get(&mut self, _ctx: Context) -> Result<String> {
    Ok(String::from("test"))
  }
}

struct TestDestination {}
impl Destination for TestDestination {
  type Recv = String;
  fn send(&mut self, payload: Self::Recv, _ctx: Context) -> Result {
    assert_eq!(payload, "test");
    Ok(())
  }
}

#[derive(Clone)]
struct TestProcessor {}

impl Processor for TestProcessor {
  type Recv = String;
  type Send = String;
  fn process(&self, payload: Self::Recv, _ctx: Context) -> Result<Self::Send> {
    Ok(payload)
  }
}

#[test]
fn test_pipeline() {
  let source = TestSource {};
  let destination = TestDestination {};
  let processor = TestProcessor {};

  let pipeline = PipelineBuilder::new()
    .source(source)
    .processor(processor.clone())
    .processor(processor)
    .destination(destination)
    .build();
}

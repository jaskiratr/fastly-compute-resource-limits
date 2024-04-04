//! Compute resource limit tests
use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use serde_json::json;
use std::time::{Duration, Instant};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
  // Filter request methods...
  match req.get_method() {
    // Allow GET and HEAD requests.
    &Method::GET | &Method::HEAD => (),
    // Deny anything else.
    _ => {
      return Ok(
        Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
          .with_header(header::ALLOW, "GET, HEAD")
          .with_body_text_plain("This method is not allowed\n"),
      )
    }
  };
  // Initialize the logger with the one endpoint
  // Notice we are echoing to stdout, so we don't need separate println! for log-tailing
  log_fastly::Logger::builder()
    .max_level(log::LevelFilter::Debug)
    .default_endpoint("my_endpoint")
    .echo_stdout(true)
    .init();
  // Set endpoint for panics
  fastly::log::set_panic_endpoint("my_endpoint")?;

  // Pattern match on the path...
  match req.get_path() {
    // If request is to the `/` path...
    "/" => {
      // Get some request data to log
      let ts = chrono::Utc::now();
      let record = json!({
         "timestamp": ts.format("%F %T%.6f %Z").to_string(),
         "trace_id": std::env::var("FASTLY_TRACE_ID").unwrap_or_else(|_| String::new()),
         "client_ip": req.get_client_ip_addr().unwrap().to_string(),
         "host": req.get_header_str("Host"),
         "request_method": req.get_method_str(),
         "url": req.get_url_str(),
      });
      // Send the logs
      // note we didn't specify a target so it goes to `my_endpoint`, which we set as the default
      // We could have also specified the target log::info!(target: "my_endpoint", "{}", record.to_string())
      log::info!("{}", record.to_string());
      // Send a default synthetic response.
      Ok(
        Response::from_status(StatusCode::OK)
          .with_content_type(mime::TEXT_HTML_UTF_8)
          .with_body(include_str!("welcome-to-compute.html")),
      )
    }

    "/panic" => {
      log::info!("Testing panic");
      simulate_panic();
      log::info!("Finished panic test.");

      Ok(
        Response::from_status(StatusCode::OK)
        .with_body_text_plain("panic test\n"),
      )
    }

    "/test_memory_limit" => {
      log::info!("Starting memory consumption limit test...");
      consume_500mb_of_ram();
      log::info!("Finished memory consumption limit test.");
     
      Ok(
        Response::from_status(StatusCode::OK)
        .with_body_text_plain("memory consumption limit test\n"),
      )
    }

    "/test_time_limit" => {
      log::info!("Starting execution time limit test...");
      run_for_5_minutes();
      log::info!("Finished execution time limit test.");

      Ok(
        Response::from_status(StatusCode::OK)
        .with_body_text_plain("execution time limit test\n"),
      )
    }

    // Not enforced
    "/test_vcpu_limit" => {
      log::info!("Starting vCPU consumption limit test...");
      simulate_vcpu_usage();
      log::info!("Finished vCPU consumption limit test.");

      Ok(
        Response::from_status(StatusCode::OK)
        .with_body_text_plain("vCPU consumption limit test\n"),
      )
    }

    // Catch all other requests and return a 404.
    _ => Ok(
      Response::from_status(StatusCode::NOT_FOUND)
        .with_body_text_plain("The page you requested could not be found\n"),
    ),
  }
}

fn simulate_panic() {
  panic!()
}

fn simulate_vcpu_usage() {
  let start_time = Instant::now();
  let duration_limit = Duration::from_millis(100); // 100 milliseconds

  // Simulate CPU usage by performing some computation
  let mut result = 0;
  while Instant::now().duration_since(start_time) < duration_limit {
    result += 1;
  }

  // Printing result just to avoid the computation being optimized away
  println!("Result: {}", result);
}

fn consume_500mb_of_ram() {
  let num_bytes = 500 * 1024 * 1024; // 500MB in bytes
  let mut data: Vec<u8> = Vec::with_capacity(num_bytes);

  // Fill the vector with some data to actually consume memory
  for _ in 0..num_bytes {
    data.push(0);
  }

  println!("Data size: {} bytes", data.len());
}

fn run_for_5_minutes() {
  let start_time = Instant::now();
  let duration_limit = Duration::from_secs(5 * 60); // 5 minutes in seconds

  loop {
    // Do some work here

    // Check if 5 minutes have elapsed
    if Instant::now().duration_since(start_time) >= duration_limit {
      break;
    }
  }
}

/// <reference types="@fastly/js-compute" />

import { Logger } from 'fastly:logger';

addEventListener('fetch', (event) => event.respondWith(handleRequest(event)));

async function handleRequest(event) {
  const logger = new Logger('my_endpoint');
  let req = event.request;
  let url = new URL(req.url);

  switch (url.pathname) {
    case '/':
      logger.log(
        JSON.stringify({
          method: event.request.method,
          url: event.request.url
        })
      );
      return new Response('OK');
      break;
    case '/test_memory_limit':
      logger.log('Starting memory consumption test');
      consumeMemory(); // Consume memory
      return new Response('OK');
      break;
    case '/test_runtime_limit':
      logger.log('Starting to execute for 5 min');
      executeForDuration(5 * 60 * 1000); // Execute for 5 minutes (300000 milliseconds)
      break;
    case '/test_log_levels':
      logger.log('Generating logs ...');
      generateLogs();
      return new Response('OK');
      break;
    default:
      return new Response('OK');
      break;
  }
}

// Ttry different log levels
async function generateLogs(request) {
  try {
    // Generate test logs
    console.error('This is an error log');
    console.warn('This is a warning log');
    console.info('This is an info log');

    // Simulate an exception
    throw new Error('Simulated exception');
  } catch (error) {
    // Log the exception and close the request
    console.error('Exception occurred:', error.message);
    request.send(500, 'Internal Server Error');
  }
}

// Function to consume memory
function consumeMemory() {
  let arr = 'Hello';
  while (true) {
    arr.concat('o');
  }
}

// Function to execute for a specified duration
async function executeForDuration(durationInMilliseconds) {
  const startTime = Date.now();
  while (Date.now() - startTime < durationInMilliseconds) {
    // Perform some task
    await new Promise((resolve) => setTimeout(resolve, 1000)); // Wait for 1 second (simulated task)
  }
}

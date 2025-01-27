# Structure

* Intro (5, 5)
  * This talk (2) - meme + list of advantages, It is all on GitHub - QR Code
  * me (1)
  * Rust in Edhouse (2) - LeanCat WE
* Example (1, 6)
  * General diagram (1) - what is SSE, Console output from GET /events (1)
* Part 1: Fearless Concurrency (6, 12)
  * Avoiding communication via shared memory (2) - what the compiler prevents?
  * Channels at a glance (1) - just simple snippet, Sender, Receiver
  * Processes interacting via channels diagram (1)
  * Snippet from example: Sending message to pump? (1)
  * Snippet from example: Pump? (1)
* Part 2: Strong ecosystem and community (4, 16)
  * Axum and Tokio, crates (1)
  * Snippet from Example (1) - SSE implementation `tokio::sync::broadcast`, `publish`
  * Snippet from Example (1) - SSE implementation `Stream`, `get_events`
  * Cargo (1)
* Part 3: Powerful macros and generics (3, 18)
  * Snippet from Example (1) - `EventDto::with_json_payload` generic function
  * Snippet from Example (1) - `Serialize` derive macro from `Serde`
* Part 4: Error Handling (4, 22)
  * Test (1)
  * How it works (1)
  * Diagram (1)
  * vs. Exceptions (1)
* Conclusion (2, 24)

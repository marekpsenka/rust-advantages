# Structure

* Intro (5, 5)
  * me (1)
  * Rust in Edhouse (2) - LeanCat WE
  * This talk (2) - meme + list of advantages, It is all on GitHub - QR Code
* Example (1, 6)
  * General diagram (1) - what is SSE, Console output from GET /events (1)
* Part 1: Fearless Concurrency (6, 12)
  * Avoiding communication via shared memory (2) - what the compiler prevents?
  * Processes interacting via channels diagram (1)
  * Channels at a glance (1) - just simple snippet, Sender, Receiver
  * Snippet from example: Sending message to pump? (1)
  * Snippet from example: Pump? (1)
* Part 2: Strong ecosystem and community (4?, 15)
  * Axum and Tokio, crates (1)
  * Cargo ?, (1)
  * Snippet from Example (1) - SSE implementation `tokio::sync::mpsc::Broadcast`, `publish`
  * Snippet from Example (1) - SSE implementation `Stream`, `get_events`
* Part 3: Powerful macros and generics (3, 19)
  * Snippet from Example (1) - `Serialize` derive macro from `Serde`
  * Types of macros (1)
  * Snippet from Example (1) - `EventDto::with_json_payload` generic function
* Part 4: Error Handling (4, 23)
  * Test (1)
  * How it works (1)
  * Diagram (1)
  * vs. Exceptions (1)
* Conclusion (2, 25)

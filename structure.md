# Structure

* Intro (5, 5)
  * me (1)
  * Rust in Edhouse (2) - LeanCat WE
  * This talk (2) - meme + list of advantages
* Example (2-3?, 8)
  * ? It is all on GitHub (1?) - QR Code
  * General diagram (1) - what is SSE, channels highlighted
  * Console output from GET /events (1)
* Part 1: Fearless Concurrency (5, 13)
  * Processes interacting via channels diagram (1)
  * Channels at a glance (1) - just simple snippet, Sender, Receiver
  * Avoiding communication via shared memory (1) - what the compiler prevents?
  * Snippet from example: Sending message to pump? (1)
  * Snippet from example: Pump? (1)
* Part 2: Strong ecosystem and community (4?, 17)
  * Axum and Tokio, crates (1)
  * Cargo ?, (1)
  * Snippet from Example (1) - SSE implementation `tokio::sync::mpsc::Broadcast`, `publish`
  * Snippet from Example (1) - SSE implementation `Stream`, `get_events`
* Part 3: Powerful macros and generics (3, 20)
  * Snippet from Example (1) - `Serialize` derive macro from `Serde`
  * Types of macros (1)
  * Snippet from Example (1) - `EventDto::with_json_payload` generic function
* Part 4: Error Handling (4, 24)
  * Test (1)
  * How it works (1)
  * Diagram (1)
  * vs. Exceptions (1)
* Conclusion (2, 26)

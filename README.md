# Data Processing With Actors

To run the app, call:

```bash
cargo run -- --from 2020-07-03T12:00:09Z  --symbols LYFT,MSFT,AAPL,UBER,LYFT,AMD,GOOG
```

To access the data with curl, do in another terminal:

```bash
http://localhost:8080/tail/10
```
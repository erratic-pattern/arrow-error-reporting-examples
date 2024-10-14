
Just use `cargo run` to run the example


```console
$ cargo run --example object_store
   Compiling arrow-error-reporting-example v0.1.0 (/Users/adam/Code/arrow-error-reporting-examples)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/object_store`
Error:
   0: Generic S3 error: Error after 10 retries in 5.723990834s, max_retries:10, retry_timeout:180s, source:error sending request for url (http://169.254.169.254/latest/api/token)
   1: Error after 10 retries in 5.723990834s, max_retries:10, retry_timeout:180s, source:error sending request for url (http://169.254.169.254/latest/api/token)
   2: error sending request for url (http://169.254.169.254/latest/api/token)
   3: client error (Connect)
   4: tcp connect error: Host is down (os error 64)
   5: Host is down (os error 64)

Location:
   examples/object_store.rs:11

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```



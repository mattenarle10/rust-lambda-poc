# AWS Rust Lambda POC

A simple proof-of-concept using **Rust**, **AWS Lambda**, and the **AWS SDK for Rust**.  
Beginner-friendly. Uses a basic Lambda function that lists your S3 buckets.
This repo is a small POC for starting to learn Rust and AWS Lambda, inspired by the recent support for the AWS SDK for Rust.

---

## Features

- AWS Lambda function written in Rust
- Uses AWS SDK for Rust
- Demonstrates listing S3 buckets
- Very small Lambda binary (2–4 MB)
- Extremely fast cold starts

## References

- [AWS Lambda with Rust](https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html)
- [AWS SDK for Rust – Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html)
- [Building serverless applications with Rust on AWS Lambda](https://aws.amazon.com/blogs/compute/building-serverless-applications-with-rust-on-aws-lambda/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/ch00-00-introduction.html)


## Deploy with Cargo Lambda

Assumes AWS CLI is configured with credentials.

1. (Optional) pick profile/region:

   ```bash
   export AWS_PROFILE=your-profile
   ```

2. Build and deploy:

   ```bash
   cargo lambda build --release
   cargo lambda deploy rust-lambda-poc
   ```

3. Invoke to test (remote Lambda):

   ```bash
   # hello example
   cargo lambda invoke rust-lambda-poc --remote \
     --data-ascii '{"path":"/items","id":123}'

   # simple health check
   cargo lambda invoke rust-lambda-poc --remote \
     --data-ascii '{"path":"/health","command":"health"}'
   ```


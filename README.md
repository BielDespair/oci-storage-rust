



# OCI Client Rust

A Rust HTTP Client for accessing **Oracle Cloud Infrastructure (OCI) Object Storage** with RSA SHA256 signature authentication.

Inspired by OCI Objectstorage .NET package.

## Features

- Automatic authentication using private PEM key and `~/.oci/config`.
- Generates `Authorization` header according to OCI standards.
- Asynchronous function to fetch objects (`get_object`).
- Uses `reqwest::Client`, which is thread-safe.
- Returns object as bytes (`Vec<u8>`), ready to save or process.



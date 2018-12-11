# MozDef-Proxy Design Specification

MozDef-Proxy is intended to provide one simple service. That is, it accepts
POST requests containing event data encoded as JSON and will queue that
event data for consumption by MozDef.  In order to safely act as a public
interface to a potentially private MozDef instance, MozDef-Proxy will queue
events into [Amazon SQS](https://aws.amazon.com/sqs/).

## API Specification

MozDef-Proxy's REST API will accept requests with any path, provided that it
is a POST request with a `Content-Type` of `application/json` and that the
body of the request contains some required information.  Status codes are
used to indicate success or failure in processing a request, with plaintext
messages in the body to provide error messages.

### Uploading event data

```
POST /
```

#### Request

| Name | Type | Description | Example |
| ---- | ---- | ----------- | ------- |
| category | string | A descriptive name/type for the data | vulnerability |
| hostname | string | Fully-qualified domain name of the sender | server.example.com |
| severity | string | One of DEBUG, INFO, NOTICE, WARNING, ERROR, CRITICAL, ALERT, EMERGENCY | WARNING |
| process  | string | The name of the process sending the log | patchesscanner |
| summary  | string | A human-readable explanation of the event data | "results of vuln scan" |
| tags     | []string | An array of string tags to associate with the event | `["vulnerability", "clair"]` |
| details  | object | Any JSON-serialized data associated with the event | `{"vulnerabilities": [...]}` |

#### Response

Status codes indicate success/failure. The body of all responses will contain
a human-readable plaintext message detailing the cause of any errors encountered.

* 200 - Event data has been queued successfully.
* 400 - Client request is missing required data.
* 429 - Client is making too many requests and must slow down.
* 500 - Unable to process the request at this time.

## Configuration

The use of configuration files should be avoided in favour of command-line arugments.
This keeps the process of deploying and managing deployments of this service simple,
and makes use easier.

## Logging

All logs will be output to `stderr`.

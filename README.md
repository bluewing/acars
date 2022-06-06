# ACARS

<p align="center">
<img src="./assets/logo.png" alt="ACARS logo" width="400">
</p>

_ACARS_ is a simple but opinionated GitHub Action designed for reporting CI/CD failures to Slack—for example, if PHPUnit tests fail on a commit. It is named after the [Aircraft Communications Addressing and Reporting System](https://en.wikipedia.org/wiki/ACARS), of the same acronym. It is written in Rust, and when used as a GitHub action, runs inside of an [Alpine Linux-derived Rust container](https://github.com/rust-lang/docker-rust/blob/186e4d580b5581861907c015d6b58919c809e375/1.61.0/alpine3.14/Dockerfile).

## Usage

```yaml
uses: bluewing/acars@v1
env:
  WEBHOOK_URL: ${{ secrets.WEBHOOK_URL }}
```

## Inputs

| Name          | Details                                                                                                                                                     |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `WEBHOOK_URL` | **Required**. This is the URL for the Webhook you've configured for your slack organization. Provide this as an environment variable in your workflow step. |

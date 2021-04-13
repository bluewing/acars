# ACARS

<p align="center">
<img src="./assets/logo.png" alt="ACARS logo" width="400">
</p>

_ACARS_ is a simple but opinionated GitHub Action designed for reporting CI/CD failures to Slack—for example, if PHPUnit tests fail on a commit. It is named after the [Aircraft Communications Addressing and Reporting System](https://en.wikipedia.org/wiki/ACARS), of the same acronym. It is written in Rust and runs inside a linux docker container.

## Usage

```yaml
uses: bluewing/acars@v1
env:
  WEBHOOK_URL: ${{ secrets.WEBHOOK_URL }}
```

## Inputs

| Name | Details |
| --- | --- |
| `WEBHOOK_URL` | **Required**. This is the URL for the Webhook you've configured for your slack organization. Provide this as an environment variable in your workflow step.


# Development Guide

## Running CircleCI locally

### Step 1: Install CircleCI

If you wish to run CircleCI locally, start by installing it:

- macOS
```
brew install circleci
```

- Linux (via Snap)
```
sudo snap install docker circleci
sudo snap connect circleci:docker docker
```

- Windows (via Chocolatey)
```
choco install circleci-cli -y
```

### Step 2: Run CircleCI

The jobs live in `.circleci/continue_config.yml` (`.circleci/config.yml` is only
the setup config that selects the `pipeline-scope` parameter). To run a job,
export the config to `process.yml`, and specify it when executing:
```shell
circleci config process --pipeline-parameters "pipeline-scope: full" .circleci/continue_config.yml > process.yml
circleci local execute -c process.yml --job JOB_NAME
```

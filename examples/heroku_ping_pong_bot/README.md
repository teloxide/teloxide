# Heroku example

This is an example project on how to deploy `webhook_ping_pong_bot` to heroku.

You will need to configure the buildpack for heroku. We will be using [Heroku rust buildpack](https://github.com/emk/heroku-buildpack-rust). Configuration was done by using `heroku` CLI.

If you're creating a new Heroku application, run this command inside example
```
heroku create --buildpack emk/rust
```

To set buildpack for existing applicaton:
```
heroku buildpacks:set emk/rust
```

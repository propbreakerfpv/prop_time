# prop-time
a little program that outputs the time and date for my starship prompt

# Usage
## install
`cargo install prop-time`
## starship config
```toml
[custom.prop-time]
format = "$output"
description = "propbreakers custom time command"
command = "prop-time"
when = true
```

## What command I executed
- command

## What error message I got
```zsh
> npm run start

> rust-webpack-template@0.1.0 start
> rimraf dist pkg && webpack-dev-server --open -d

/Users/***/****/my-init-wasm-project/node_modules/webpack-cli/bin/utils/prompt-command.js:46
        return func(...args);
               ^

TypeError: Class constructor ServeCommand cannot be invoked without 'new'
    at runWhenInstalled (/Users/***/****/my-init-wasm-project/node_modules/webpack-cli/bin/utils/prompt-command.js:46:9)
    at promptForInstallation (/Users/***/****/my-init-wasm-project/node_modules/webpack-cli/bin/utils/prompt-command.js:140:10)
    at /Users/***/****/my-init-wasm-project/node_modules/webpack-cli/bin/cli.js:32:43
    at Object.<anonymous> (/Users/***/****/my-init-wasm-project/node_modules/webpack-cli/bin/cli.js:366:3)
    at Module._compile (node:internal/modules/cjs/loader:1356:14)
    at Module._extensions..js (node:internal/modules/cjs/loader:1414:10)
    at Module.load (node:internal/modules/cjs/loader:1197:32)
    at Module._load (node:internal/modules/cjs/loader:1013:12)
    at Module.require (node:internal/modules/cjs/loader:1225:19)
    at require (node:internal/modules/helpers:177:18)

Node.js v18.19.0
```


## What I did to solve the problem
- this issue is caused to delete vulnerabilities so I accepted the vulnerabilities and the issue was solved.


## What command I executed
- `npm run start`

## What error message I got

```zsh
> rust-webpack-template@0.1.0 start
> rimraf dist pkg && webpack-dev-server --open -d

The command moved into a separate package: @webpack-cli/serve
Would you like to install serve? (That will run npm install -D @webpack-cli/serve) (yes/NO) : y
npm ERR! code ERESOLVE
npm ERR! ERESOLVE unable to resolve dependency tree
npm ERR! 
npm ERR! While resolving: rust-webpack-template@0.1.0
npm ERR! Found: webpack@4.47.0
npm ERR! node_modules/webpack
npm ERR!   dev webpack@"^4.42.0" from the root project
npm ERR! 
npm ERR! Could not resolve dependency:
npm ERR! peer webpack@"5.x.x" from @webpack-cli/serve@2.0.5
npm ERR! node_modules/@webpack-cli/serve
npm ERR!   dev @webpack-cli/serve@"*" from the root project
npm ERR! 
npm ERR! Fix the upstream dependency conflict, or retry
npm ERR! this command with --force or --legacy-peer-deps
npm ERR! to accept an incorrect (and potentially broken) dependency resolution.
npm ERR! 
npm ERR! 
npm ERR! For a full report see:
npm ERR! /Users/***/.npm/_logs/2024-03-20T04_49_15_163Z-eresolve-report.txt

npm ERR! A complete log of this run can be found in: /Users/***/.npm/_logs/2024-03-20T04_49_15_163Z-debug-0.log
undefined
```

```js
'Log files:
/Users/***/.npm/_logs/2024-03-20T04_49_15_163Z-debug-0.log

# npm resolution error report

While resolving: rust-webpack-template@0.1.0
Found: webpack@4.47.0
node_modules/webpack
  dev webpack@"^4.42.0" from the root project

Could not resolve dependency:
peer webpack@"5.x.x" from @webpack-cli/serve@2.0.5
node_modules/@webpack-cli/serve
  dev @webpack-cli/serve@"*" from the root project

Fix the upstream dependency conflict, or retry
this command with --force or --legacy-peer-deps
to accept an incorrect (and potentially broken) dependency resolution.
```

## What I did to solve the problem
- `npm install -D @webpack-cli/serve --legacy-peer-deps`

## What I learned
- legacy-peer-deps: [config | npm Docs](https://docs.npmjs.com/cli/v7/using-npm/config#legacy-peer-deps)
- peerDependencies: [package.json | npm Docs](https://docs.npmjs.com/cli/v10/configuring-npm/package-json#peerdependencies)



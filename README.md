# pixi-activate

Activate an environment that has been globally registered with pixi-register.



## Dev setup

This project is setup using pixi! In order to run this locally you will need to:

1. build/run the project with pixi + cargo

### 1: Run the project

Run the project directly with pixi

Or, if you want to run the binary you can add the target build directory to path, 
and build using pixi.  This way, you can also run the project as a pixi extension.

```
$ export PATH=$PATH:$PWD/target/debug
$ pixi run build
$ pixi register --help
```

## Try it out

Register an environment with pixi-register. For example, how about this one. From the root of this project.
By default it will use the cwd.

```
$ pixi register --name pixi-register

$ $ cat ~/.pixi/register/environments.json
[
  {
    "name": "pixi-register",
    "path": "<full/path/to>/pixi-register"
  }
]
```

Then activate it!
```
$ pixi activate --name pixi-register
```
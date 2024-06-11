# How to compile the GUI version

!! **Disclaimer :** this might not work on your steam deck, a pre-compiled binary or Flatpak version will be distributed in the future !!

## Requirement

You need the [base](https://github.com/ALEZ-DEV/Babylonia-terminal?tab=readme-ov-file#requirement) requirement  
You will need [Flutter](https://flutter.dev/) in order to compile the GUI version  
Install [Rinf](https://github.com/cunarist/rinf) with **Cargo** :

```bash
cargo install rinf
```

And check if all the requirement are installed successfully :

```bash
rustc --version
flutter doctor
```

## Compile

First generate all the necessary messages : 

```bash
rinf message
```

Then compile the project : 

```bash
flutter run
```

# How to compile the GUI version

!! **Disclaimer :** this might not work on your steam deck, a Flatpak version is distributed, [see instruction on how to install it!](https://github.com/ALEZ-DEV/Babylonia-terminal/wiki/Installation#installing-via-flatpak)

## Requirement

You need the [base](https://github.com/ALEZ-DEV/Babylonia-terminal/wiki/Compilation-requirement) requirement  
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
Generate all the necessary linux file for compilation :

```bash
flutter create --platform=linux .
```

Then compile the project : 

```bash
flutter run
```

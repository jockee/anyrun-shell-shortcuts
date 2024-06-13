# Anyrun Shell Shortcuts
A simple Anyrun Plugin that allows you to add Results that execute Commands.

## Usage
Add your Results to the `shortcuts` configuration field.

## Configuration
```ron
// <Anyrun config dir>/shell-shortcuts.ron
Config(
  shortcuts: {
    "Shutdown": (
      command: "shutdown now",
      icon: "system-shutdown"
    ),
  },
  // Override the shell used to launch the command
  shell: None,
)
```

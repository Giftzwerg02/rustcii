<div align="center">
  <h1><code>rustcii</code></h1>

  <p>
    <strong>A tool to convert old and outdated "characters" into the superior Rustcii-Encoding.</strong>
  </p>

  <strong>Speak your mind. Blazingly (🦀) fast (🚀).</strong>

  <h3>
    <a href="https://github.com/Giftzwerg02/rustcii">Github</a>
    <span> | </span>
    <a href="https://crates.io/crates/rustcii">crates.io</a>
  </h3>
</div>

## Installation
```sh
cargo install rustcii
```
## Usage
### Encode
```sh
rustcii encode <input>
```
### Decode
```sh
rustcii decode <input>
```

### Scripting Examples
#### Bash
> This example script is also available in this repository.
```sh
# Load in the contents of your file which stil uses pathetic, old and outdated "characters".
input=$(cat example.txt)

# Convert the content into the new, exciting, blazingly (🦀) fast (🚀) and - most important of all - superior *Rustcii-Encoding*.
better=$(rustcii encode "$input")

# Save the superior content into a new file. The file format <file-name>.superior.<extension> is not required - but recommended since everyone should know at one glance whether or not a file contains the new, exciting, blazingly (🦀) fast (🚀) and - most important of all - superior *Rustcii-Encoding* or just some pathetic, old and outdated "characters".
# Note: Once operating systems adapt to this modern change and allows the Rustcii-Encoding to be used in filenames, it is recommended to change the filename into the Rustcii-Encoding as well.
echo $better > example.superior.txt
```
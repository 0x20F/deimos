<h1 align="center">deimos</h1>

<p align="center">Fast, cross-platform, snippets or text expander. Whatever you wanna call it.</p>


<br><br><br>

Imagine typing something weird in an input field somewhere, like `hello` and all of a sudden the text gets replaced with `hello darkness my old friend`.

That's the gist of it at least, creating simple keys that expand into bigger, more complex things.


## Configuration (`deimos.toml`)
You define your snippets into a file called `deimos.toml` and place that file into your home directory.
- Windows -> `C:\Users\Username\deimos.toml`
- Linux -> `~/deimos.toml`

The contents of the file are as follows:
```toml
[snippets]
one = "1"
two = "2"
long = """
This is a paragraph
of random text that I want
to have on hand in case an
opportunity to use it arises.
"""
```


As easy as that. Under the header `[snippets]` you write some key value pairs, where the key is what you want to type with your hands and the value is what you want to insert in its place.

## Building
At the moment there are no actual releases since the projet is still in the "this is a fun gimmick" phase so you'll have to build it yourself if you want to use it.

Building deimos, however, is very easy.

1. Clone the repository
`git clone https://github.com/0x20F/deimos`

2. CD into the directory
`cd deimos`

3. Build using cargo
`cargo build --release`

4. Your new binary should now be available in `deimos/target/release`


## Contributing
Any contributions are welcome!<br>
- If you have random ideas that might work well with something like this, you're more than welcome to open up a discussion and we'll talk about it.<br>
- If something's broken and you'd like it fix open an issue!<br>
- If something's broken and you've fixed it yourself, even better, open a Pull Request!<br>
- If you feel lonely, or bored, open up a discussion and we'll see what happens!<br>

Everybody's welcome!

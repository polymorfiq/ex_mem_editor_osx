# MemEditor

**This is for research only. This is not a finished tool of any sort**

This is a library made purely for fun that combines Rust and Elixir to allow for memory editing and injection of MacOS applications.

In order for this tool to work, you need to:
- Boot into Recovery Mode on your Mac
- Run `csrutil disable` and then `csrutil enable --without debug` (this could open your mac up to viruses)
- (Possibly) Run this tool as `sudo` such as through `iex -S mix` in the project directory

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `mem_editor` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:mem_editor, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/mem_editor>.


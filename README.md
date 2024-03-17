# `bench`

Do benchmarking at the `main` function.

## Examples

### Cache line

An example to find out the size of the cache lines of your machine.

Steps:

1.  Install the `dfplot` tool by running:
    ```bash
    cargo install --git https://github.com/Banyc/dfplot.git
    ```
1.  Execute the example by running:
    ```bash
    cargo run --example cache_line
    ```
1.  Identify the cache size by observing the scatter plot.
1.  Uninstall the `dfplot` tool by running:
    ```bash
    cargo uninstall dfplot
    ```

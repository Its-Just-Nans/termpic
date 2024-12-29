# termpic - FORK of [termpix](https://github.com/hopey-dishwasher/termpix) [![crates.io version](https://img.shields.io/crates/v/termpic)](https://crates.io/crates/termpic) ![crates.io downloads](https://img.shields.io/crates/d/termpic)

Draw images in an ANSI terminal! Requires that your terminal can show ANSI colours, and uses a font that can show the 'bottom half block' character (`▄`).

## Usage

```sh
cargo install termpic
termpic <file> [--width <width>] [--height <height>] [--max-width <max-width>] [--max-height <max-height>] [--true-color|--true-colour] [--output <file>]
```

<details>

<summary>Example</summary>

![buzz](https://cloud.githubusercontent.com/assets/4640028/13073384/9d46b2e2-d4f2-11e5-9218-09f1a05bf296.png)

`file` can be any image file readable by the [rust 'image' library](https://github.com/PistonDevelopers/image).
I've seen it fail with some unusual jpeg files ("Marker SOF2 is not supported.")

By default, it will fill as much of the terminal as possible, while keping the aspect ratio of the input image.
Use --width or --height to override this, specifying the number of terminal rows or columns to fit to (Or both to specify an absolute size).

![buzz smaller](https://cloud.githubusercontent.com/assets/4640028/13073404/b60d1410-d4f2-11e5-85c1-ccb6dc967eae.png)

If your terminal supports it, you can use the full 24-bit colour palette with the `--true-colour` flag:
![lena looks good](https://cloud.githubusercontent.com/assets/4640028/13419797/fa51cb88-dfd4-11e5-87c3-f8620cd67557.png)

In low-colour mode, high-contrast, colourful images tend to work better than flatter images. Skin tones and shades of brown are particularly poorly represented in the ANSI colour palette.
![lena looks bad](https://cloud.githubusercontent.com/assets/4640028/13073360/705a85b0-d4f2-11e5-917a-fdb91e5e45b9.png)

</details>

## License

Apache 2.0 license

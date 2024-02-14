# giuda

A tool for creating a CV in LaTeX

You have heard of Infrastructure as Code, now get ready for CV as Code!

Starting from a yaml file, create a LaTeX file for a beautiful CV.

[Output example](./fred.pdf)

Compile the rust program and run `giuda.exe my_cv.yaml output` or `giuda my_cv.yaml output` to parse the cv definition. It will create a file called `output.tex` and another called `altacv.cls`.

If you don't have LaTeX installed and you don't want to install it, you can use [Overleaf](https://www.overleaf.com/). Just upload the two files, along with your picture, and overleaf will do the rest.

There's an example for the definition in `./giuda.yaml`, but there is also a schema that can help you while you make your own CV.

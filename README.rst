Language comparison
===================

At the example of the Mandelbrot set
------------------------------------

This is a comparison of an algorithm implementation into different languages 
that are commonly used in science (and I am able to use), or should be.

The algorithm is the well known Mandelbrot set, which visualizes a fractal on
the complex plane by coloring each pixel according to the amount of 
iterations needed to prove that the complex representation is not part of the
Mandelbrot set. For any black pixel, the membership could not be disproved 
within the given limit of iterations. The iterations are limited by the value
range of the used bitdepth for the result image. For an 8 Bit PNG the 
iterations will be limited to 255.

The iteration is defined as:

.. math::

    z_{n+1} = z_n^2 + c,

with :math:`z, c \in \mathbb{C}`


Most of the Rust example is taken from:

  *"Programming Rust by Jim Blandy and Jason Orendorff (o'Reilly). Copyright 
  2018 Jim Blandy and Jason Orendorff, 978-491-92728-1"*
    
and is also available on `GitHub <https://github.com/ProgrammingRust>`_.

Notes
-----

* The implementations in different languages were written to be comparable.
  Thus, unusual function or variable name norms might have been used.



TODOs
-----

* Implement FORTRAN, C, Octave
* There is a horrible loop in mandelbrot.R marked by a TODO comment. This can
  surely be done better.
* Make the application more verbose
* add 16 or 32 bit option for coloring (this will heavily increase the
  calculation time)
* Make everything multicore
"""
Calculate and plot the Mandelbrot set using Python
"""
import png


def write_image(filename, buffer, bounds):
    """Write the image file

    Save the given list of pixel values (img) to filename.
    Bounds is a tuple specifying the resolution.

    Parameters
    ----------
    filename : str
        If filename already exists, it will be overwritten
    buffer : list
        list of pixel values
    bounds : tuple
        resolution of the image in pixel.

    """
    with open(filename, 'wb') as f:
        png_file = png.Writer(width=bounds[0], height=bounds[1], bitdepth=8,
                              greyscale=True)
        png_file.write_array(f, buffer)


def render(bounds, upper_left, lower_right):
    """Render the Mandelbrot set

    Applies the actual calculation function to the complex representation of
    each pixel

    Parameters
    ----------
    bounds  : tuple
        Image resolution in pixels (width, height).
    upper_left : complex
        Upper left corner of the complex plane considered for the
        Mandelbrot set.
    lower_right : complex
        Lower right corner of the complex plane considered for the
        Mandelbrot set.

    Returns
    -------
    mandelbrot : list
        Mandelbrot set of buffer, arranged in the same manner.

    """
    # complex transformer
    def calc(x, y):
        # calculate result
        res = is_member(
            pixel_to_point(bounds, (x, y), upper_left, lower_right), 255
        )
        if res is None:
            return 0
        else:
            return 255 - res

    # transform
    complex_buffer = [calc(x, y) for y in range(bounds[1])
                      for x in range(bounds[0])]

    return complex_buffer


def pixel_to_point(bounds, pixel, upper_left, lower_right):
    """Map between Cartesian and Complex plane

    Given the bounds of the image and the bounding edges of the Complex plane
    (upper_left and lower_right) the image pixel location (pixel) will be
    transformed to its corresponding point on the Comlex plane

    Parameters
    ----------
    bounds : tuple
        Image resolution in pixels. (width, height).
    pixel : tuple
        Pixel position as (x, y), where x < width and y < height.
    upper_left : complex
        Upper left corner of the complex plane considered for the
        Mandelbrot set.
    lower_right : complex
        Lower right corner of the complex plane considered for the
        Mandelbrot set.

    Returns
    -------
    location : complex
        The pixel representation on the Complex plane

    """
    # calculate width and height of the complex plane
    width = lower_right.real - upper_left.real
    height = upper_left.imag - lower_right.imag

    # transform
    location = complex(
        real=upper_left.real + pixel[0] * width / bounds[0],
        imag=upper_left.imag - pixel[1] * height / bounds[1]
    )

    # return
    return location


def is_member(c, limit):
    """Member of Mandelbrot set

    Determines if c is member of the Mandelbrot set.
    Cumulates the product of complex number z * z + c.
    If ||z|| > 2 then z will converge towards +/- inf and
    is therefore in the set.
    Otherwise the number of iterations needed to proof z is
    not in the set is returned.

    Parameters
    ----------
    c : comlpex number
        Number to be added to z on each iteration
    limit : int
        Iteration limit

    Returns
    -------
    iterations : int
        Number of iterations needed until z

    """
    z = 0.0 + 0.0j

    for i in range(limit):
        z = z*z + c

        if abs(z)**2 > 4.:
            return i

    # if loop ended, return None
    return None


if __name__ == '__main__':
    import sys
#    import argparse
#    # setup CLI
#    parser = argparse.ArgumentParser(description="Mandelbrot set plotter")
#    parser.add_argument('filename', metavar='FILE', type=str,
#                        help="Filename for the Mandelbrot set. "
#                             "Existing files will be overwritten.")
#    parser.add_argument('bounds', metavar='SIZE', type=int, nargs=2,
#                        help="Image resolution as <width>x<height>. "
#                             "Example: 1000x750.")
#    parser.add_argument('upper_left', metavar='UPPERLEFT', type=complex,
#                        help="Upper left corner of the Mandelbrot set. "
#                             "Syntax is to mark the imaginary part by a 'j' "
#                             "like -1.2+0.35j for complex(re=-1.2, im=0.35).")
#    parser.add_argument('lower_right', metavar='LOWERRIGHT', type=complex,
#                        help="Lower right corner of the Mandelbrot set. "
#                             "Syntax is to mark the imaginary part by a 'j' "
#                             "like -1+0.2j for complex(re=-1.0, im: 0.2).")
    # parse
#    args = parser.parse_args()
    try:
        filename = sys.argv[1]
        _b = sys.argv[2].split('x')
        bounds = (int(_b[0]), int(_b[1]))
        assert len(bounds) == 2
        upper_left = complex(sys.argv[3])
        lower_right = complex(sys.argv[4])
    except:
        print("Usage: mandelbrot.py FILENAME SIZE UPPERLEFT LOWERRIGHT")
        print("Example: python mandelbrot.py mandelbrot.png 1000x750 "
              "-1.2+0.35j -1+0.2j")
        sys.exit()

    # render the set. TODO: here the time function
    mandelbrot = render(bounds=bounds,
                        upper_left=upper_left,
                        lower_right=lower_right
                        )

    # write the result
    write_image(filename=filename, buffer=mandelbrot, bounds=bounds)

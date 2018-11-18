"""
Calculate and plot the Mandelbrot set using Python
"""


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
    print("(1.0 + 1.0i):", is_member(1.+1.j, 500))
    print("(0.1 - 0.3i):", is_member(.1-.3j, 500))

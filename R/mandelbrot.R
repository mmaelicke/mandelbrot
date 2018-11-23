# Title     : mandelbrot.R
# Objective : Calculate the Mandelbrot set for a given extract of the complex
#             plane. Saves the result to a file.
# Created by: mirko
# Created on: 22.11.18
require(testthat)
require(png)


is_member <- function(c, limit) {
    #' determine if c is part of the Mandelbrot set.
    #'
    #' At most limit iterations the complex number 0+0i will be multiplied by
    #' the sum of z and c. If the squre of the norm of z is larger than 4,
    #' c is a member
    z <- complex(real=0.0, imaginary=0.0)

    for (i in 0:limit) {
        z <- z*z + c
        if (Mod(z)^2 > 4) {
            return(i)
        }
    }
    return(NULL)
}
# some unittest
test_that("is_member 1.0+1.0j", {
    expect_equal(
        is_member(complex(real=1.0, imaginary=1.0), 100),
        1
    )
})
test_that("is_member 1.0+1.0j", {
    expect_equal(
        is_member(complex(real=0.1, imaginary=-0.3), 100),
        NULL
    )
})

pixel_to_point <- function(bounds, pixel, upper_left, lower_right) {
    #' Map between Cartesian and Complex plane
    #'
    #' Given the bounds of the image and the bounding edges of the complex
    #' plane (upper_left, loweer_right) the image pixel location is
    #' transformed to the corresponding point on the complex plane
    width <- Re(lower_right) - Re(upper_left)
    height <- Im(upper_left) - Im(lower_right)

    z <- complex(
        real=Re(upper_left) + pixel[1] * width / bounds[1],
        imaginary=Im(upper_left) - pixel[2] * height / bounds[2]
    )
}
test_that("transform pixel", {
    expect_equal(
        pixel_to_point(c(100, 100), c(25, 75),
                       complex(real=-1.0, imaginary=1.0),
                       complex(real=1.0, imaginary=-1.0)),
        complex(real=-0.5, imaginary=-0.5)
    )
})


# helper function
calc <- function(x, y, bounds, upper_left, lower_right) {
    res <- is_member(pixel_to_point(bounds, c(x, y), upper_left, lower_right),255)
    if (is.null(res)){
        return(0)
    } else {
        return(255 - res)
    }
}
render <- function(bounds, upper_left, lower_right) {
    #' Render the Mandelbrot set
    #'
    #' Applies the actual calculation function to the complex representation
    #' of each pixel on the complex plane
    buffer <- rep(0, bounds[1] * bounds[2])

    # TODO: don't know enough about R to do this better
    i <- 1
    for (x in 1:bounds[1]) {
        for (y in 1:bounds[2]) {
            buffer[i] <- calc(x,y,bounds, upper_left, lower_right)
            i <- i + 1
        }
    }

    return(buffer)
}

write_image <- function(filename, buffer, bounds){
    #' Write the image file
    #'
    #' Reshape the given pixel buffer to a matrix using bounds and transform
    #' the 8-bit greyscale values to 0..1 values.
    img <- matrix(buffer, nrow=bounds[2], byrow=F) / 255.

    writePNG(img, target=filename)
}


# ------------------------------------------------------------------------------
# Parse command line arguments
args <- commandArgs(trailingOnly=T)
if (length(args) < 4) {
    stop(
    "Usage:  Rscript mandelbrot.R FILE SIZE UPPERLEFT LOWERRIGHT
    Example: Rscript mandelbrot.R mandelbrot.png 1000x750, -1.2+0.35i -1+0.2i"
    )
} else {
    filename <- args[1]
    bounds <- as.numeric(strsplit(args[2], 'x')[[1]])
    upper_left <- as.complex(args[3])
    lower_right <- as.complex(args[4])
}

# render
t1 <- Sys.time()
mandelbrot <- render(bounds, upper_left, lower_right)
print(sprintf("Calculation: %.3f seconds.", as.numeric(Sys.time() - t1)))

write_image(filename, mandelbrot, bounds)



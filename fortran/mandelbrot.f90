! mandelbrot.f90
! Calculates the Mandelbrot set for a given extract of the complex plane

program mandelbrot
    use calc

    implicit none
    complex                             :: upper_left, lower_right
    integer, dimension(2)               :: bounds
    integer, dimension(:), allocatable  :: buffer
    integer :: int

    ! set vars
    upper_left = (-1.0, 1.0)
    lower_right = (1.0, -1.0)
    bounds = (/ 10, 10 /)

    ! allocate enough space for a
    allocate(buffer(1:bounds(1) * bounds(2)), stat=int)

    call render(buffer, bounds, upper_left, lower_right)
    print*, buffer

    ! free the buffer
    deallocate(buffer, stat=int)

end program mandelbrot

module calc
contains
    ! render the Mandelbrot set
    ! This function does the actual calculation and is the only function
    ! that needds to be benchmarked and optimized
    subroutine render(buffer, bounds, upper_left, lower_right)
        implicit none

        integer, intent(inout)                 :: buffer(:)
        integer,dimension(2), intent(in)    :: bounds
        complex, intent(in)                 :: upper_left
        complex, intent(in)                 :: lower_right
        integer     :: row, column, iterations , i
        complex     :: poi

        ! TODO: put the fortran equiv of a assert here
        i = 1
        do row = 1, bounds(2)
            do column = 1, bounds(1)
                ! get the point on the complex plane
                poi = pixel_to_point(bounds, (/ column, row /), upper_left, lower_right)

                ! check if it is in the Mandelbrot set
                iterations = is_member(poi, 255)

                ! save to the buffer
                buffer(i) = 255 - iterations
                i = i + 1
            end do
        end do

    end subroutine render

    ! Map between Cartesian and complex plane
    complex function pixel_to_point (bounds, pixel, upper_left, lower_right)
        implicit none
        integer, dimension(2), intent(in) :: bounds
        integer, dimension(2), intent(in) :: pixel
        complex, intent(in) :: upper_left
        complex, intent(in) :: lower_right
        real :: width
        real :: height
        real :: re
        real :: im

        ! calculate
        width = real(lower_right) - real(upper_left)
        height = aimag(upper_left) - aimag(lower_right)

        ! return complex number
        re = real(upper_left) + pixel(1) * width / bounds(1)
        im = aimag(upper_left) - pixel(2) * height / bounds(2)
        pixel_to_point = complex(re, im)
    end function pixel_to_point

    ! is_member: actual algorithm implementation
    integer function is_member(c, limit)
        implicit none
        complex, intent(in) :: c
        integer, intent(in) :: limit
        integer :: i
        complex :: z

        z = (0, 0)
        do i = 0, limit
            z = z * z + c
            if (abs(z)**2 > 4) then
                is_member = i
                return
            end if
        end do
        ! if the loop ended, set i to None
        is_member = 0
    end function is_member

end module calc

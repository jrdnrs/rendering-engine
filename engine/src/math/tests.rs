mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::math::{mat4f::Mat4f, vec3f::Vec3f};

    #[test]
    fn mat4_mul_test() {
        let a = Mat4f::from_rows(&[
            [1.0, 3.0, 5.0, 7.0],
            [6.0, 5.0, 4.0, 3.0],
            [1.0, 2.0, 3.0, 4.0],
            [9.0, 8.0, 7.0, 6.0],
        ]);

        let b = Mat4f::from_rows(&[
            [9.0, 11.0, 13.0, 15.0],
            [1.0, 2.0, 3.0, 4.0],
            [10.0, 11.0, 12.0, 13.0],
            [1.0, 3.0, 5.0, 7.0],
        ]);

        let answer = Mat4f::from_rows(&[
            [69.0, 93.0, 117.0, 141.0],
            [102.0, 129.0, 156.0, 183.0],
            [45.0, 60.0, 75.0, 90.0],
            [165.0, 210.0, 255.0, 300.0],
        ]);

        assert_eq!(answer.as_slice(), (a * b).as_slice());
    }

    #[test]
    fn mat4_mul_assign_test() {
        let a = Mat4f::from_rows(&[
            [1.0, 3.0, 5.0, 7.0],
            [6.0, 5.0, 4.0, 3.0],
            [1.0, 2.0, 3.0, 4.0],
            [9.0, 8.0, 7.0, 6.0],
        ]);

        let mut b = Mat4f::from_rows(&[
            [9.0, 11.0, 13.0, 15.0],
            [1.0, 2.0, 3.0, 4.0],
            [10.0, 11.0, 12.0, 13.0],
            [1.0, 3.0, 5.0, 7.0],
        ]);

        b *= a;

        let answer = Mat4f::from_rows(&[
            [69.0, 93.0, 117.0, 141.0],
            [102.0, 129.0, 156.0, 183.0],
            [45.0, 60.0, 75.0, 90.0],
            [165.0, 210.0, 255.0, 300.0],
        ]);

        assert_eq!(answer.as_slice(), b.as_slice());
    }

    #[test]
    fn index_rowcol_test() {
        let a = Mat4f::from_rows(&[
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 7.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(7.0, a[(2, 2)]);
    }

    #[test]
    fn index_test() {
        let a = Mat4f::from_rows(&[
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 7.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(7.0, a[10]);
    }

    #[test]
    fn vec3_cross_test() {
        let a = Vec3f::new(27.0, 45.0, 7.0);
        let b = Vec3f::new(4.0, 21.0, 83.0);

        assert_eq!((3588.0, -2213.0, 387.0), a.cross(b).as_tuple())
    }

    #[test]
    fn vec3_dot_test() {
        let a = Vec3f::new(27.0, 45.0, 7.0);
        let b = Vec3f::new(4.0, 21.0, 83.0);

        assert_eq!(1634.0, a.dot(b))
    }
}

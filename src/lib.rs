pub mod cvector;

#[cfg(test)]
mod tests {
    use crate::cvector::CVector;

    #[test]
    fn scaling_test() {
        let mut test_vec = CVector::new(100, 150);

        assert_eq!(
            test_vec.mag(),
            ((100.0_f64 * 100.0) + (150.0 * 150.0)).sqrt()
        );

        test_vec.normalise();
        assert_eq!(test_vec.mag(), 1.0_f64);

        test_vec.set_mag(10);
        let expected_mag = 10.0;
        let epsilon = 0.000001;
        assert!(f64::abs(test_vec.mag() - expected_mag) < epsilon);

        test_vec.mult_mag(3.3);
        let expected_mag = 33.0;
        assert!(f64::abs(test_vec.mag() - expected_mag) < epsilon);
    }

    #[test]
    fn arithmetic_test() {
        let mut test_vec0 = CVector::new(10, 5);
        let test_vec1 = CVector::new(20, 10);

        test_vec0.add(&test_vec1);
        assert_eq!(test_vec0.x(), 30.0);
        assert_eq!(test_vec0.y(), 15.0);
    }

    #[test]
    fn generic_creation_test() {
        let cv0 = CVector::new(10, 10);
        assert_eq!(cv0.x(), 10.0);
        assert_eq!(cv0.y(), 10.0);

        let cv1 = CVector::new(-10, -10);
        assert_eq!(cv1.x(), -10.0);
        assert_eq!(cv1.y(), -10.0);

        let cv2 = CVector::new(10.0, 10.0);
        assert_eq!(cv2.x(), 10.0);
        assert_eq!(cv2.y(), 10.0);

        let cv3 = CVector::new(-10.0, -10.0);
        assert_eq!(cv3.x(), -10.0);
        assert_eq!(cv3.y(), -10.0);
    }
}

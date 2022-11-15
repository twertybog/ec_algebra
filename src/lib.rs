#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
struct ECPoint {
    x: i128,
    y: i128,
}

impl ECPoint {
    fn BasePointGGet() -> ECPoint {
        ECPoint { x: 0, y: 0 }
    }

    fn ECPointGen(x: i128, y: i128) -> ECPoint {
        ECPoint { x, y }
    }

    fn IsOnCurveCheck(point: ECPoint, a: i128, b: i128) -> bool {
        if point.y.pow(2) == (point.x.pow(3) + a * point.x + b){
            return true;
        }
        else {
            return false;
        }
    }

    fn AddECPoints(a: ECPoint, b: ECPoint) -> ECPoint {
        let m = (a.y - b.y) / (a.x - b.x);
        let x_r = m.pow(2) - a.x - b.x;
        let y_r = a.y + m * (x_r - a.x);
        ECPoint { x: x_r, y: y_r }
    }

    fn DoubleECPoints(point: ECPoint, a: i128) -> ECPoint {
        let m = (3 * point.x.pow(2) + a) / (2 * point.y);
        let x_3 = m.pow(2) - 2 * point.x;
        let y_3 = m*(point.x - x_3) - point.y;
        ECPoint{
            x: x_3,
            y: y_3
        }
    }

    fn ScalarMult(point: ECPoint, k: i128, a: i128) -> ECPoint {
        if k > 1{
            let mut last = ECPoint::DoubleECPoints(point, a);
            for _ in 2..k {
                last = ECPoint::AddECPoints(point, last);
            }
            return last
        }
        else {
            return point;
        }
    }

    fn ECPointToString(point: ECPoint) -> String {
        format!("x = {}; y = {}", point.x, point.y)
    }

    fn PrintECPoint(point: ECPoint) {
        println!("{}", ECPoint::ECPointToString(point));
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn add_ec_test(){
        //P(202;478);Q(196;292)
        assert_eq!(ECPoint::ECPointGen(563, 11669), 
                ECPoint::AddECPoints(ECPoint::ECPointGen(202, 478), 
                ECPoint::ECPointGen(196, 292)));
    }

    #[test]
    fn double_ec_test(){
        //y^2 = x^3 + 3x + 3
        //P(196;125)
        assert_eq!(ECPoint::ECPointGen(212129, -97701238),
        ECPoint::DoubleECPoints(ECPoint::ECPointGen(196, 125), 3));
        //y^2 = x^3 + 4x + 3
        //P(6;-4)
        assert_eq!(ECPoint::ECPointGen(184, 2496),
        ECPoint::DoubleECPoints(ECPoint::ECPointGen(6, -4), 4));
    }

    #[test]
    fn scalar_mult_test(){
        //y^2 = x^3 + 4x + 3
        //P(6;-4)
        //k = 3
        assert_eq!(ECPoint::ECPointGen(6, -4),
        ECPoint::ScalarMult(ECPoint::ECPointGen(6, -4), 3, 4));
    }

    #[test]
    fn is_on_curve_test(){
        //y^2 = x^3 + x + 1
        //P(0; 1)
        let a = 1;
        let b = 1;
        let point = ECPoint::ECPointGen(0, 1);
        assert!(ECPoint::IsOnCurveCheck(point, a, b));
    }
}
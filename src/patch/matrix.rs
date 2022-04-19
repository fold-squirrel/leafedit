use lopdf::Object;

#[derive(Clone, Copy)]
pub struct Matrix {
    cm: [f64; 6],
}

impl Matrix {
    pub fn new(mut array: Vec<Object>) -> Option<Matrix> {
        let mut cm_option = vec![];
        for _ in 0..6 {
            cm_option.push(match array.remove(0) {
                Object::Integer(num) => Some(num as f64),
                Object::Real(num) => Some(num),
                _ => None,
            })
        }
        if !cm_option.contains(&None) {
            let mut cm = [1f64, 0f64, 0f64, 1f64, 0f64, 0f64];
            for i in 0..6 {
                cm[i] = cm_option[i].unwrap()
            }
            Some(Matrix { cm })
        } else {
            None
        }
    }

    pub fn defualt() -> Matrix {
        Matrix { cm: [1f64, 0f64, 0f64, 1f64, 0f64, 0f64], }
    }

    pub fn multiply(&mut self, old: Matrix) {
        let matrix_a = [
            [old.cm[0], old.cm[1], old.cm[4]],
            [old.cm[2], old.cm[3], old.cm[5]],
            [     0f64,      0f64,      1f64]
        ];
        let matrix_b = [
            [self.cm[0], self.cm[1], self.cm[4]],
            [self.cm[2], self.cm[3], self.cm[5]],
            [      0f64,       0f64,       1f64]
        ];
        let mut out = [
            [ 0f64, 0f64, 0f64],
            [ 0f64, 0f64, 0f64],
            [ 0f64, 0f64, 0f64]
        ];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    out[i][j] += matrix_a[i][k] * matrix_b[k][j];
                }
            }
        }

        self.cm = [out[0][0], out[0][1], out[1][0], out[1][1], out[0][2], out[1][2]];
    }


    pub fn inverse(cm_vec: &mut Vec::<Matrix>) -> [Object; 6] {
        let mut matrix = Matrix::defualt();
        for x in cm_vec {
            x.multiply(matrix);
            matrix = *x;
        }

        let m = [
            [matrix.cm[0], matrix.cm[1], matrix.cm[4]],
            [matrix.cm[2], matrix.cm[3], matrix.cm[5]],
            [        0f64,         0f64,         1f64]
        ];

        let mut determinant = 0f64;
        for i in 0..3 {
            determinant +=  m[0][i]*(m[1][(i+1)%3]*m[2][(i+2)%3] - m[1][(i+2)%3]*m[2][(i+1)%3]);
        }

        let mut out = [
            [ 0f64, 0f64, 0f64],
            [ 0f64, 0f64, 0f64],
            [ 0f64, 0f64, 0f64]
        ];

        (0..3).for_each(|i| {
            for j in 0..3 {
                let n = m[(j+1)%3][(i+1)%3] * m[(j+2)%3][(i+2)%3];
                let l = m[(j+1)%3][(i+2)%3] * m[(j+2)%3][(i+1)%3];
                let o: f64 = n - l;

                if o.eq(&0f64) {
                    out[i][j] = 0f64;
                    continue;
                }

                out[i][j] = o / determinant;
            }
        });

        matrix.cm = [out[0][0], out[0][1], out[1][0], out[1][1], out[0][2], out[1][2]];

        [
            Object::Real(matrix.cm[0]),
            Object::Real(matrix.cm[1]),
            Object::Real(matrix.cm[2]),
            Object::Real(matrix.cm[3]),
            Object::Real(matrix.cm[4]),
            Object::Real(matrix.cm[5])
        ]
    }

}



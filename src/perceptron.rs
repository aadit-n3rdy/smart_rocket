use crate::matrix::Matrix;

fn activate(inp: f64) -> f64 {
    return 2.0 / (1.0 + (1.0 / inp.exp())) + 1.0;
}

#[derive(Debug, Clone)]
pub struct Perceptron {
    wts: Vec<Matrix>,
    shape: Vec<usize>,
}

impl Perceptron {
    pub fn create(shape_param: Vec<usize>) -> Perceptron {
        let mut ptron = Perceptron {
            wts: Vec::new(),
            shape: Vec::new(),
        };
        ptron.shape = shape_param;
        for i in 0..ptron.shape.len() - 1 {
            ptron.wts.push(Matrix::create_randoms(
                ptron.shape[i + 1],
                ptron.shape[i] + 1,
            ));
        }
        return ptron;
    }
    pub fn get_wts(&self) -> Vec<Matrix> {
        return self.wts.clone();
    }
    pub fn set_wts(&mut self, new_wts: Vec<Matrix>) {
        self.wts = new_wts;
    }

    pub fn calculate(&self, input: &Matrix) -> Matrix {
        let mut temp: Matrix = input.clone();
        let unit_matrix = Matrix::create_init(1, 1, vec![1.0]);
        for i in 0..self.wts.len() {
            temp.append_rows(&unit_matrix);
            temp = self.wts[i].clone() * temp;
            for j in 0..temp.rows() {
                temp.set(j, 0, activate(temp.get(j, 0)));
            }
        }
        return temp.clone();
    }
}


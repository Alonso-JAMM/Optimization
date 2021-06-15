# Optimization

This library is a small collection of unconstrained optimization algorithms.
This library also includes some automatic differentiation techniques using dual number and hyperdual numbers.
These number systems are useful in creating gradients and hessians of objective functions.
The optimization algorithms are based on the book "Numerical Optimization second edition" written by Jorge Nocedal and Stephen J. Wright.

*The library is marked as "stable" since I think it is mature enough to be used in my use case.*


## Motivation for this library
My goal is to write a geometric constraint solver for [FreeCAD](https://github.com/Alonso-JAMM/FreeCAD_Assembly4).
The core problem to be solved by a geometric constraint solver is to minimize an objective function.
The minimization process can be done by an optimization algorithm so that is the main reason I needed an optimization library.

There are many optimization libraries; however, I wanted to learn more about writing code to solve problems using numerical methods.
I got interested in Rust and its speed compared with python which is why I chose to make my own optimization library in Rust.
The first constraint solver I wrote used scipy optimization library in order to solve the problem; however, it is relatively slow.
I tested my implementations of the optimization algorithms versus scipy optimization algorithms and found out that the rust version is at least 100x faster which makes me happy.

I also wanted to make the library somewhat generic so it could be used on other optimization problems (not just geometric constraint solvers).
So the way of writing objective functions for the optimization algorithms in this library is through traits which helps to interface arbitrary objective functions with the optimization algorithms.

**The current implementations of the algorithms and number systems (dual and hyperdual) are heavily influenced by my use case of geometric constraint solver; however it should not be too hard to modify the current implementations in order to be used by more general problems (the dual and hyperdual number systems only support basic math functions so far).**

## Algorithms
So far only a couple of basic optimization algorithm are implemented.
These algorithms are the ones I used the most on the geometric constraint solver I wrote.
In the future I plan to add more optimization algorithms.
Here is the list of optimization algorithms implemented so far:

1. **Steepest Descent**. This algorithm is the simplest algorithm of all, however is the lest efficient.
2. **Nonlinear Conjugate Gradient (NCG)**. This algorithm performs better and it is also simple.
3. **BFGS**. This algorithm performs better than Steepest Descent and NCG and doesn't need a hessian matrix from the objective function.
4. **Trust Newton Conjugate Gradient (Trust-NCG)**. This algorithm performs really well with fast convergence and precision; however it requires hessian matrix from the objective function which may be costly.

The Trust-NCG algorithm performed the best in my case for the geometric constraint solver.
I may implement more algorithms in the future and compare them with the Trust-NCG.


## Objective Function - Problem
The way to interface with the optimization algorithms is through traits so that the objective function can calculate the gradient or hessian in any form and just give back arrays representing the gradient or hessian matrices.
This library uses heavily the Rust ndarray crate in order to do calculations.
The traits used by objective functions (problems) are the following:

1. **Objective** This is the main trait for an objective functions.
It contains methods for evaluating the objective function and to update the x-value.
This trait is heavily influenced in how I used the optimization algorithms in scipy.
I had a custom class that internally contained a list of hyperdual numbers representing the variables of the objective function.
At each iteration step I would update the internal hyperdual numbers using the input array and then I would evaluate the objective function and store the resulting real value, gradient, and hessian internally.
Finally, I would give these values back to the optimization algorithm when needed.

2. **Gradient** This trait contains the methods needed for calculating a gradient or a univariate first derivative (useful in line-search algorithms).
3. **Hessian** This trait contains the method needed for calculating the hessian.

For more in-depth description of these traits you can look at **/src/proble.rs** where the traits are defined.
Also, you can look at the **/examples/** to see how traits could be implemented in real-world scenarios.


## Example
Here is an example of an objective function problem and the Trust-NCG algorithm used to solve the problem (taken by **examples/trust_ncg_example.rs**):

```Rust
    use ndarray::{arr1, Array1, Array2};
    use optimization::number_system::HyperDualScalar as HDual;
    use optimization::problem::{Objective, Gradient, Hessian};
    use optimization::TrustNCG;


    // Objective function. In this case it only contains a vector of HyperDualScalars
    // representing the variables. Note that it could also contain a vector of
    // error functions in a least-squares problem.
    pub struct ProblemObjective {
        x: Vec<HDual>,
    }


    // Implementation of the Objective trait. The method `eval` is the method that
    // actually evaluates the objective function. The methods `eval_re`, `grad`,
    // and `diff` will call `eval` in order to evaluate the function and then pick
    // the re or dual value of the result.
    impl Objective for ProblemObjective {
        type Output = HDual;
        fn eval(&self) -> HDual {
            let x1 = &self.x[0];
            let x2 = &self.x[1];
            let x3 = &self.x[2];

            let u1 = x1.cos()*x2.sin() - 0.05;
            let u2 = x2.sin() - 0.2;
            let u3 = x3.powi(2) - 2.56;

            u1.powi(2) + u2.powi(2) + u3.powi(2)
        }

        fn eval_real(&self) -> f64 {
            self.eval().re
        }

        fn update_x(&mut self, x: &Array1<f64>) {
            for i in 0..self.x.len() {
                self.x[i].re = x[i];
                self.x[i].e1 = 0.0; // make sure we are removing any dual part
            }
        }

        fn move_step(&mut self, x: &Array1<f64>, p: &Array1<f64>, alpha: f64) {
            let a = HDual{re: alpha, e1: 1.0, e2: 1.0, e1e2: 0.0};
            for i in 0..self.x.len() {
                self.x[i] = x[i] + a*p[i];
            }
        }
    }

    // Implementation of the gradient trait. Note that both `grad` and `diff` call
    // `eval` in order to obtain the corresponding derivatives.
    impl Gradient for ProblemObjective {
        fn grad(&mut self, output: &mut Array1<f64>) {
            for i in 0..self.x.len() {
                self.x[i].e1 = 1.0;
                output[i] = self.eval().e1;
                self.x[i].e1 = 0.0;
            }
        }

        fn diff(&self) -> f64 {
            self.eval().e1
        }
    }


    // Implementation of the hessian trait. This implementation tries to avoid
    // performing redundant operations since the hessian is a symmetric matrix
    impl Hessian for ProblemObjective {
        fn hess(&mut self, output: &mut Array2<f64>) {
            let mut eval: f64;
            for i in 0..self.x.len() {
                for j in i..self.x.len() {
                    self.x[i].e1 = 1.0;
                    self.x[j].e2 = 1.0;

                    eval = self.eval().e1e2;
                    output[[i,j]] = eval;
                    output[[j,i]] = eval;

                    self.x[i].e1 = 0.0;
                    self.x[j].e2 = 0.0;
                }
            }
        }
    }



    fn main() {
        let x0 = arr1(&[1.0, 1.0, 1.0]);

        let a = HDual{re:1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
        let b = HDual{re:1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
        let c = HDual{re:1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

        let mut x = Vec::new();
        x.push(a);
        x.push(b);
        x.push(c);

        let mut problem = ProblemObjective{x};
        let mut min = TrustNCG::new();

        let sol = min.minimize(&x0, &mut problem);

        println!("Solution succeeded?: {}, iterations: {}, function evaluations: {}, \
        gradient evaluations: {}", sol.success, sol.iter_num, sol.f_evals, sol.f_grad_evals);
        println!("solution x: {}", sol.x);
    }

```

This example gives the following result:
```
Solution succeeded?: true, iterations: 10, function evaluations: 10, gradient evaluations: 10
solution x: [1.3181161455941164, 0.20135792735957758, 1.600000000800614]
```

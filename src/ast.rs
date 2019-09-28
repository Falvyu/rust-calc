

mod ast {

    use std::rc::Rc;

    #[derive(Debug)]
    pub enum CalcError {
        NotEnoughArguments,
        UnknownError,
    }
    
    pub fn calc(op: Rc<dyn Operation>) -> Result<i32, CalcError> {
        let mut stack: Vec<Rc<dyn Operation>> = Vec::new();
        let mut infix: Vec<Rc<dyn Operation>> = Vec::new();

        stack.push(op);
        while !stack.is_empty() {
            match stack.pop() {
                Some(op) => {
                    op.push_op(&mut infix);
                    infix.push(Rc::clone(&op));
                },
                None => return Err(CalcError::UnknownError)
            }
        }

        let mut num_stack = Vec::new();
        let mut r = 0;
        for op in infix {
            match op.calc(&mut num_stack) {
                Ok(x) => {
                    r = x;
                },
                Err(err) => {
                    return Err(err)
                }                
            }
        }
        Ok(r)
    }
    
    pub trait Operation {
        fn calc(&self, args: &mut Vec<i32>) -> Result<i32, CalcError>;

        fn push_op(&self, infix: &mut Vec<Rc<dyn Operation>>);                   
    }

    pub struct Add {
        left: Rc<dyn Operation>,
        right: Rc<dyn Operation>
    }

    pub struct Sub {
        left: Rc<dyn Operation>,
        right: Rc<dyn Operation>
    }

    pub struct Mult {
        left: Rc<dyn Operation>,
        right: Rc<dyn Operation>
    }

    pub struct Number {
        x: i32,
    }


    
    impl Operation for Add {
        fn calc(&self, args: &mut Vec<i32>) -> Result<i32, CalcError> {

            // Get 2 numbers from the top of the stack
            match (args.pop(), args.pop()) {
                (Some(x), Some(y)) => {
                    let r = x+y;
                    args.push(r);
                    Ok(r)
                },
                _ =>  Err(CalcError::NotEnoughArguments)            
            }
        }

        fn push_op(&self, infix: &mut Vec<Rc<dyn Operation>>) {
            infix.push(Rc::clone(&self.left));
            infix.push(Rc::clone(&self.right));
        }
    }
    
    impl Operation for Sub {
        fn calc(&self, args: &mut Vec<i32>) -> Result<i32, CalcError> {

            match (args.pop(), args.pop()) {
                (Some(x), Some(y)) => {
                    let r = x - y;
                    args.push(r);
                    Ok(r)                                 
                },
                _ => Err(CalcError::NotEnoughArguments)
            }
        }

        fn push_op(&self, infix: &mut Vec<Rc<dyn Operation>>) {
            infix.push(Rc::clone(&self.left));
            infix.push(Rc::clone(&self.right));
        }
    }


    impl Operation for Mult {
        fn calc(&self, args: &mut Vec<i32>) -> Result<i32, CalcError> {

            match (args.pop(), args.pop()) {
                (Some(x), Some(y)) => {
                    let r = x*y;
                    args.push(y);
                    Ok(r)
                },
                _ => Err(CalcError::NotEnoughArguments)                
            }
        }

        fn push_op(&self, infix: &mut Vec<Rc<dyn Operation>>) {
            infix.push(Rc::clone(&self.left));
            infix.push(Rc::clone(&self.right));
        }
    }

    impl Operation for Number {
        fn calc(&self, args: &mut Vec<i32>) -> Result<i32, CalcError> {
            args.push(self.x);
            Ok(self.x)
        }

        fn push_op(&self, infix: &mut Vec<Rc<dyn Operation>>) {
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn assert_calc(op: Rc<dyn Operation>, expected: i32) {
            let result = calc(op);
            assert_eq!(result.is_ok(), true);
            let r = result.unwrap();
            assert_eq!(r, expected);
        }
        
        #[test]
        fn test_calc() {

            let n1 = Rc::new(Number {x: 1});
            assert_calc(n1, 1);        
            
            let n2 = Rc::new(Number {x: 2});
            assert_calc(n2, 2);
            
            let n3 = Rc::new(Number {x: 3});
            assert_calc(n3, 3);
            
            let a1 = Rc::new(Add {
                left: Rc::clone(&n1),
                right: Rc::clone(&n2)
            });
            assert_calc(a1, 3);
            
            let m1 = Rc::new(Mult {
                left: Rc::clone(&a1),
                right: Rc::clone(&n3)
            });
            assert_calc(m1, 9);
            
        }
    }

}


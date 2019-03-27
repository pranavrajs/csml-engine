//TODO: make a better error system

use std::io::{Error, ErrorKind, Result};
use crate::parser::ast::*;
use crate::interpreter::{
    builtins::*,
    message::*,
    csml_rules::*,
    json_to_rust::*,
};

// use std::collections::HashMap;

// struct StepInfo<'a> {
//     name: &'a str,
//     var: HashMap<&'a str, &'a Expr>,
//     retry: i32
// }

// // return Result<struct, error>
// pub fn match_flowstarter(ident: &Ident, list: &[Expr]) {
//     println!("{:?} - {:?}", ident, list);
// }

pub struct AstInterpreter<'a> {
    pub context: &'a JsContext,
    pub event: &'a Option<Event>,
}

impl<'a> AstInterpreter<'a>
{
    // fn match_var(&self, var: &Ident, action: &Expr) -> Result<MessageType> {
    //     match var {
    //         Ident(arg) if arg == "event"   => Ok(MessageType::Msg(Message::new(action, "Text".to_string()))),
    //         Ident(_arg)                    => Err(Error::new(ErrorKind::Other, "Error no builtin found")),
    //     }
    // }

    fn match_builtin(&self, builtin: &Ident, args: &Expr) -> Result<MessageType> {
        match builtin {
            Ident(arg) if arg == "Typing"=> Ok(MessageType::Msg(Message::new(typing(args), arg.to_string()))),
            Ident(arg) if arg == "Wait"  => Ok(MessageType::Msg(Message::new(wait(args), arg.to_string()))),
            Ident(arg) if arg == "Text"  => Ok(MessageType::Msg(Message::new(text(args), arg.to_string()))),
            Ident(arg) if arg == "Url"   => Ok(MessageType::Msg(Message::new(url(args), arg.to_string()))),
            Ident(arg) if arg == "OneOf" => Ok(MessageType::Msg(Message::new(one_of(args), "Text".to_string()))),
            Ident(arg) if arg == "Button"=> Ok(button(args)),
            Ident(_arg)                  => Err(Error::new(ErrorKind::Other, "Error no builtin found")),
        }
    }

    fn match_action(&self, action: &Expr) -> Result<MessageType> {
        match action {
            Expr::Action { builtin, args }  => self.match_builtin(builtin, args),
            Expr::LitExpr(_literal)         => Ok(MessageType::Msg(Message::new(action, "Text".to_string()))),
            Expr::Empty                     => Ok(MessageType::Empty),
            _                               => Err(Error::new(ErrorKind::Other, "Error must be a valid action")),
        }
    }

    fn match_reserved(&self, reserved: &Ident, arg: &Expr) -> Result<MessageType> {
        match reserved {
            Ident(ident) if ident == "say"      => {
                self.match_action(arg)
            }
            Ident(ident) if ident == "ask"      => {
                // TMP implementation of block for an action
                if let Expr::VecExpr(block) = arg {
                    match self.match_block(block) { 
                        Ok(root)  => Ok(MessageType::Msgs(root.message)),
                        Err(e)    => Err(e)
                    }
                } else {
                    //check for info
                    self.match_action(arg)
                    //check if retry > 1
                }
            }
            Ident(ident) if ident == "retry"    => {
                // check nbr
                // save new option if exist
                self.match_action(arg)
            }
            _                                   => {
                Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
            }
        }
    }

    // Can be rm if we want to have multiple ask in the same Step
    fn match_reserved_if(&self, reserved: &Ident, arg: &Expr) -> Result<MessageType> {
        match reserved {
            Ident(ident) if ident == "say"      => {
                self.match_action(arg)
            }
            Ident(ident) if ident == "retry"    => {
                // check nbr
                // save new option if exist 
                self.match_action(arg)
            }
            _                                   => {
                Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
            }
        }
    }

    // TMP implementation of block for an action
    fn check_valid_step(&self, step: &[Expr]) -> bool {
        let mut nbr = 0;

        for expr in step {
            if let Expr::Reserved { fun, .. } = expr {
                match fun {
                    Ident(ident) if ident == "ask"  => nbr += 1,
                    _                               => {}
                }
            }
        }
        nbr < 2
    }

    fn cmp_lit(&self, infix: &Infix, lit1: &Literal, lit2: &Literal) -> bool {
        match infix {
            Infix::Equal                => lit1 == lit2,
            Infix::GreaterThanEqual     => lit1 >= lit2,
            Infix::LessThanEqual        => lit1 <= lit2,
            Infix::GreaterThan          => lit1 > lit2,
            Infix::LessThan             => lit1 < lit2,
            Infix::And                  => true,
            Infix::Or                   => true,
        }
    }

    fn cmp_bool(&self, infix: &Infix, b1: bool, b2: bool) -> bool {
        match infix {
            Infix::Equal                => b1 == b2,
            Infix::GreaterThanEqual     => b1 >= b2,
            Infix::LessThanEqual        => b1 <= b2,
            Infix::GreaterThan          => b1 > b2,
            Infix::LessThan             => b1 < b2,
            Infix::And                  => b1 && b2,
            Infix::Or                   => b1 || b2,
        }
    }

    fn get_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::InfixExpr(infix, exp1, exp2)  => {
                if let Ok(var) = self.evaluate_condition(infix, exp1, exp2) {
                    return var;
                }
                // TODO: return error 
                false
            },
            Expr::LitExpr(_lit)                 => true,
            Expr::IdentExpr(_ident)             => false, // search if var exist
            _                                   => false, // ret error
        }
    }

    fn gen_literal_form_event(&self) -> Result<Literal> {
        match self.event {
            Some(event)        => {
                match event.payload {
                    PayLoad{content_type: ref t, content: ref c} if t == "text" => Ok( Literal::StringLiteral(c.text.clone()) ),
                    _                                                           => Err(Error::new(ErrorKind::Other, "event type is unown")),
                }
            },
            None               => Err(Error::new(ErrorKind::Other, "no event is received"))
        }
    }

    fn ger_var(&self, name: &Ident) -> Result<Literal> {
        match name {
            Ident(var) if var == "event"       => self.gen_literal_form_event(),
            _                                  => Err(Error::new(ErrorKind::Other, "unown variable")),
        }
    }

    // TODO: return Result<&Literal>
    fn gen_literal_form_exp(&self, exp: &Expr) -> Result<Literal> {
        match exp {
            Expr::LitExpr(lit)      => Ok(lit.clone()),
            Expr::IdentExpr(ident)  => self.ger_var(ident),
            _                       => Err(Error::new(ErrorKind::Other, "Expression must be a literal or an identifier"))
        }
    }

    fn evaluate_condition(&self, infix: &Infix, expr1: &Expr, expr2: &Expr) -> Result<bool> {
        match (expr1, expr2) {
            (Expr::LitExpr(l1), Expr::LitExpr(l2))          => {
                Ok(self.cmp_lit(infix, l1, l2))
            },
            (Expr::IdentExpr(id1), Expr::IdentExpr(id2))    => {
                match (self.ger_var(id1), self.ger_var(id2)) {
                    (Ok(l1), Ok(l2))   => Ok(self.cmp_lit(infix, &l1, &l2)),
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between ident and ident"))
                }
            },
            (Expr::IdentExpr(ident), Expr::LitExpr(l2))     => {
                match self.ger_var(ident) {
                    Ok(l1) => Ok(self.cmp_lit(infix, &l1, l2)),
                    _      => Err(Error::new(ErrorKind::Other, "error in evaluation between ident and lit"))
                }
            },
            (Expr::LitExpr(l1), Expr::IdentExpr(ident))     => {
                match self.ger_var(ident) {
                    Ok(l2) => Ok(self.cmp_lit(infix, l1, &l2)),
                    _      => Err(Error::new(ErrorKind::Other, "error in evaluation between lit and ident"))
                }
            },
            (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp1, exp2))      => {
                match (self.evaluate_condition(i1, ex1, ex2), self.evaluate_condition(i2, exp1, exp2)) {
                    (Ok(l1), Ok(l2))   => Ok(self.cmp_bool(infix, l1, l2)),
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between InfixExpr and InfixExpr"))
                }
            },
            (Expr::InfixExpr(i1, ex1, ex2), e)                        => {
                match (self.evaluate_condition(i1, ex1, ex2), self.gen_literal_form_exp(e)) {
                    (Ok(e1), Ok(_)) => {
                        match infix {
                            Infix::And                  => Ok(e1),
                            Infix::Or                   => Ok(true),
                            _                           => Err(Error::new(ErrorKind::Other, "error need && or ||"))
                        }
                    },
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between InfixExpr and expr"))
                }
            },
            (e, Expr::InfixExpr(i1, ex1, ex2))                              => {
                match (self.gen_literal_form_exp(e), self.evaluate_condition(i1, ex1, ex2)) {
                    (Ok(_), Ok(e2)) => {
                        match infix {
                            Infix::And                  => Ok(e2),
                            Infix::Or                   => Ok(true),
                            _                           => Err(Error::new(ErrorKind::Other, "error need && or ||"))
                        }
                    },
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between expr and InfixExpr"))
                }
            }
            (_, _)                                        => Err(Error::new(ErrorKind::Other, "error in evaluate_condition function")),
        }
    }

    fn valid_condition(&self, expr: &Expr) -> bool {
        match expr {
            Expr::InfixExpr(inf, exp1, exp2)    => {
                match self.evaluate_condition(inf, exp1, exp2) {
                    Ok(rep) => rep,
                    Err(e) =>{
                        println!("error {:?}", e);
                        false
                    }
                }
            },
            Expr::LitExpr(_lit)                 => true,
            Expr::IdentExpr(ident)              => {
                match self.ger_var(ident) {
                    Ok(_)   => true,
                    Err(_)  => false, // error
                }
            },
            _                                   => false, // return error
        }
    }

    fn match_ifexpr(&self, cond: &Expr, consequence: &[Expr]) -> Result<RootInterface> {
       println!("> condition > {:?}", self.valid_condition(cond));
        if self.valid_condition(cond) {
            let mut root = RootInterface {remember: None, message: vec![], next_flow: None , next_step: None};
            for expr in consequence {
                if root.next_step.is_some() {
                    return Ok(root)
                }

                match expr {
                    Expr::Reserved { fun, arg }         => {
                        match self.match_reserved_if(fun, arg) {
                            Ok(msg)   => {
                                match msg {
                                    MessageType::Msg(msg)   => root.add_message(msg),
                                    MessageType::Msgs(msgs) => root.message.extend(msgs),
                                    MessageType::Empty      => {},
                                }
                            },
                            Err(err)  => return Err(err)
                        }
                    },
                    Expr::IfExpr { cond, consequence }  => {
                        match self.match_ifexpr(cond, consequence) {
                            Ok(msg)   => root = root + msg,
                            Err(err)  => return Err(err)
                        }
                    },
                    Expr::Goto(Ident(ident))            => {root.add_next_step(ident); break},
                    _                                   => return Err(Error::new(ErrorKind::Other, "Error in If block")),
                }
            }
            Ok(root)
        } else {
            Err(Error::new(ErrorKind::Other, "error in if condition, it does not reduce to a boolean expression "))
        }
    }

    // TMP implementation of block for an action
    pub fn match_block(&self, actions: &[Expr]) -> Result<RootInterface> {
        self.check_valid_step(actions);
        let mut root = RootInterface {remember: None, message: vec![], next_flow: None , next_step: None};

        for action in actions {
            //TODO: check ask
            if root.next_step.is_some() {
                return Ok(root)
            }

            match action {
                Expr::Reserved { fun, arg } => {
                    match self.match_reserved(fun, arg) {
                        Ok(action)  => {
                            match action {
                                MessageType::Msg(msg)   => root.add_message(msg),
                                MessageType::Msgs(msgs) => root.message.extend(msgs),
                                MessageType::Empty      => {},
                            }
                        },
                        Err(err)    => return Err(err)
                    }
                },
                Expr::IfExpr { cond, consequence }  => {
                    match self.match_ifexpr(cond, consequence) {
                        Ok(action)  => root = root + action,
                        Err(err)    => return Err(err)
                    }
                },
                Expr::Goto(Ident(ident))    => root.add_next_step(ident),
                _                           => return Err(Error::new(ErrorKind::Other, "Block must start with a reserved keyword")),
            };
        }
        Ok(root)
    }
}
use lib_ruby_parser_ast::{Blob, SingleLinkedIntrusiveList, Writer};

#[cfg(feature = "onig")]
use onig::{Regex, RegexOptions};

use core::convert::TryInto;
use core::fmt::Write;

use std::collections::HashMap;

#[allow(unused_imports)]
use crate::nodes::*;
use crate::Loc;
use crate::{
    Bytes, CurrentArgStack, Lexer, MaxNumparamStack, Node, SharedContext, StaticEnvironment, Token,
    VariablesStack,
};
use crate::{Diagnostic, DiagnosticMessage, ErrorLevel};

#[derive(Debug, PartialEq)]
pub(crate) enum LoopType {
    While,
    Until,
}

#[derive(Debug, PartialEq)]
pub(crate) enum KeywordCmd {
    Break,
    Defined,
    Next,
    Redo,
    Retry,
    Return,
    Super,
    Yield,
    Zsuper,
}

enum MethodCallType {
    Send,
    CSend,
}

#[derive(Debug, PartialEq)]
pub(crate) enum LogicalOp {
    And,
    Or,
}

#[derive(Debug)]
pub(crate) enum PKwLabel<'b> {
    PlainLabel(&'b Token<'b>),
    QuotedLabel((&'b Token<'b>, &'b NodeList<'b>, &'b Token<'b>)),
}

#[derive(Debug, Clone)]
pub(crate) enum ArgsType<'b> {
    Args(Option<&'b Node<'b>>),
    Numargs(u8),
}

#[derive(Debug)]
pub(crate) struct Builder<'b> {
    static_env: &'b StaticEnvironment<'b>,
    context: &'b SharedContext,
    current_arg_stack: &'b CurrentArgStack<'b>,
    max_numparam_stack: &'b MaxNumparamStack<'b>,
    pattern_variables: &'b VariablesStack<'b>,
    pattern_hash_keys: &'b VariablesStack<'b>,
    diagnostics: &'b SingleLinkedIntrusiveList<'b, Diagnostic<'b>>,
    blob: &'b Blob<'b>,
}

impl<'b> Builder<'b> {
    pub(crate) fn new(
        static_env: &'b StaticEnvironment<'b>,
        context: &'b SharedContext,
        current_arg_stack: &'b CurrentArgStack<'b>,
        max_numparam_stack: &'b MaxNumparamStack<'b>,
        pattern_variables: &'b VariablesStack<'b>,
        pattern_hash_keys: &'b VariablesStack<'b>,
        diagnostics: &'b SingleLinkedIntrusiveList<'b, Diagnostic<'b>>,
        blob: &'b Blob<'b>,
    ) -> Self {
        Self {
            static_env,
            context,
            current_arg_stack,
            max_numparam_stack,
            pattern_variables,
            pattern_hash_keys,
            diagnostics,
            blob,
        }
    }

    //
    // Literals
    //

    // Singletons

    pub(crate) fn nil(&self, nil_t: &'b Token<'b>) -> &'b Node<'b> {
        Nil::new_in(self.blob, |nil| {
            nil.expression_l = self.loc(nil_t);
        })
    }

    pub(crate) fn true_(&self, true_t: &'b Token<'b>) -> &'b Node<'b> {
        True::new_in(self.blob, |true_| {
            true_.expression_l = self.loc(true_t);
        })
    }

    pub(crate) fn false_(&self, false_t: &'b Token<'b>) -> &'b Node<'b> {
        False::new_in(self.blob, |false_| {
            false_.expression_l = self.loc(false_t);
        })
    }

    // Numerics

    pub(crate) fn integer(&self, integer_t: &'b Token<'b>) -> &'b Node<'b> {
        Int::new_in(self.blob, |int| {
            int.value = integer_t.token_value;
            int.operator_l = None;
            int.expression_l = self.loc(integer_t);
        })
    }

    pub(crate) fn float(&self, float_t: &'b Token<'b>) -> &'b Node<'b> {
        Float::new_in(self.blob, |float| {
            float.value = float_t.token_value;
            float.operator_l = None;
            float.expression_l = self.loc(float_t);
        })
    }

    pub(crate) fn rational(&self, rational_t: &'b Token<'b>) -> &'b Node<'b> {
        Rational::new_in(self.blob, |rational| {
            rational.value = rational_t.token_value;
            rational.operator_l = None;
            rational.expression_l = self.loc(rational_t);
        })
    }

    pub(crate) fn complex(&self, complex_t: &'b Token<'b>) -> &'b Node<'b> {
        Complex::new_in(self.blob, |complex| {
            complex.value = complex_t.token_value;
            complex.operator_l = None;
            complex.expression_l = self.loc(complex_t);
        })
    }

    pub(crate) fn unary_num(&self, unary_t: &'b Token<'b>, numeric: &'b Node<'b>) -> &'b Node<'b> {
        let new_operator_l = self.loc(unary_t);
        let sign = unary_t.token_value;

        let join_sign_and_value_in =
            |sign: &'b Bytes<'b>, value: &'b Bytes<'b>, out: &'b Bytes<'b>| -> &'b Bytes<'b> {
                let mut mem = [0; 100];
                let mut writer = Writer::new(&mut mem);
                write!(
                    &mut writer,
                    "{}{}",
                    sign.as_whole_string().unwrap(),
                    value.as_whole_string().unwrap()
                )
                .unwrap();
                let s = writer.as_str().unwrap();
                let s = self.blob.push_str(s);
                out.append_borrowed(s, self.blob);
                out
            };

        match numeric {
            Node::Int(Int {
                value,
                expression_l,
                operator_l,
                ..
            }) => Int::new_in(self.blob, |int| {
                join_sign_and_value_in(sign, value, int.value);
                int.expression_l = new_operator_l.join(*expression_l);
                int.operator_l = Some(new_operator_l);
            }),
            Node::Float(Float {
                value,
                expression_l,
                ..
            }) => Float::new_in(self.blob, |float| {
                join_sign_and_value_in(sign, value, float.value);
                float.expression_l = new_operator_l.join(*expression_l);
                float.operator_l = Some(new_operator_l);
            }),
            Node::Rational(Rational {
                value,
                expression_l,
                ..
            }) => Rational::new_in(self.blob, |rational| {
                join_sign_and_value_in(sign, value, rational.value);
                rational.expression_l = new_operator_l.join(*expression_l);
                rational.operator_l = Some(new_operator_l);
            }),
            Node::Complex(Complex {
                value,
                expression_l,
                ..
            }) => Complex::new_in(self.blob, |complex| {
                join_sign_and_value_in(sign, value, complex.value);
                complex.expression_l = new_operator_l.join(*expression_l);
                complex.operator_l = Some(new_operator_l);
            }),
            _ => unreachable!(),
        }
    }

    pub(crate) fn __line__(&self, line_t: &'b Token<'b>) -> &'b Node<'b> {
        Line::new_in(self.blob, |line| {
            line.expression_l = self.loc(line_t);
        })
    }

    // Strings

    pub(crate) fn str_node(
        &self,
        begin_t: Option<&'b Token<'b>>,
        value: &'b Bytes<'b>,
        parts: &'b NodeList<'b>,
        end_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        if self.is_heredoc(begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(begin_t, &parts, end_t);

            Heredoc::new_in(self.blob, |heredoc| {
                heredoc.parts = parts;
                heredoc.heredoc_body_l = heredoc_body_l;
                heredoc.heredoc_end_l = heredoc_end_l;
                heredoc.expression_l = expression_l;
            })
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(
                begin_t.as_ref().map(|t| t.loc),
                &parts,
                end_t.as_ref().map(|t| t.loc),
            );

            Str::new_in(self.blob, |str_| {
                str_.value = value;
                str_.begin_l = begin_l;
                str_.end_l = end_l;
                str_.expression_l = expression_l;
            })
        }
    }

    pub(crate) fn string_internal(&self, string_t: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(string_t);
        Str::new_in(self.blob, |str_| {
            str_.value = string_t.token_value;
            str_.begin_l = None;
            str_.end_l = None;
            str_.expression_l = expression_l;
        })
    }

    pub(crate) fn string_compose(
        &self,
        begin_t: Option<&'b Token<'b>>,
        parts: &'b NodeList<'b>,
        end_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        if parts.is_empty() {
            return self.str_node(begin_t, self.blob.alloc_ref(), parts, end_t);
        }
        if parts.len() == 1 {
            let part = parts.iter().next().unwrap();

            match part {
                Node::Str(_) | Node::Dstr(_) | Node::Heredoc(_)
                    if begin_t.is_none() && end_t.is_none() =>
                {
                    return part;
                }

                Node::Str(Str { value, .. }) => {
                    return self.str_node(begin_t, value, parts, end_t);
                }

                Node::Dstr(_) | Node::Heredoc(_) => {
                    unreachable!()
                }

                _ => {}
            }
        }

        if self.is_heredoc(begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(begin_t, &parts, end_t);

            Heredoc::new_in(self.blob, |heredoc| {
                heredoc.parts = parts;
                heredoc.heredoc_body_l = heredoc_body_l;
                heredoc.heredoc_end_l = heredoc_end_l;
                heredoc.expression_l = expression_l;
            })
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(
                begin_t.as_ref().map(|t| t.loc),
                &parts,
                end_t.as_ref().map(|t| t.loc),
            );

            Dstr::new_in(self.blob, |dstr| {
                dstr.parts = parts;
                dstr.begin_l = begin_l;
                dstr.end_l = end_l;
                dstr.expression_l = expression_l;
            })
        }
    }

    pub(crate) fn character(&self, char_t: &'b Token<'b>) -> &'b Node<'b> {
        let str_loc = self.loc(char_t);

        let begin_l = Some(str_loc.with_end(str_loc.begin() + 1));
        let end_l = None;
        let expression_l = str_loc;

        Str::new_in(self.blob, |str_| {
            str_.value = char_t.token_value;
            str_.begin_l = begin_l;
            str_.end_l = end_l;
            str_.expression_l = expression_l;
        })
    }

    pub(crate) fn __file__(&self, file_t: &'b Token<'b>) -> &'b Node<'b> {
        File::new_in(self.blob, |file| {
            file.expression_l = self.loc(file_t);
        })
    }

    // Symbols

    fn validate_sym_value(&self, value: &Bytes, loc: Loc) {
        if !value.is_valid_utf8() {
            self.error(DiagnosticMessage::InvalidSymbol { symbol: "UTF-8" }, loc)
        }
    }

    pub(crate) fn symbol(&self, start_t: &'b Token<'b>, value_t: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(start_t).join(self.loc(value_t));
        let begin_l = Some(self.loc(start_t));
        self.validate_sym_value(value_t.token_value, expression_l);
        Sym::new_in(self.blob, |sym| {
            sym.name = value_t.token_value;
            sym.begin_l = begin_l;
            sym.end_l = None;
            sym.expression_l = expression_l;
        })
    }

    pub(crate) fn symbol_internal(&self, symbol_t: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(symbol_t);
        self.validate_sym_value(symbol_t.token_value, expression_l);
        Sym::new_in(self.blob, |sym| {
            sym.name = symbol_t.token_value;
            sym.begin_l = None;
            sym.end_l = None;
            sym.expression_l = expression_l;
        })
    }

    pub(crate) fn symbol_compose(
        &self,
        begin_t: &'b Token<'b>,
        parts: &'b NodeList<'b>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        if parts.len() == 1 {
            let part = parts.iter().next().unwrap();
            match part {
                Node::Str(Str { value, .. }) => {
                    let CollectionMap {
                        begin_l,
                        end_l,
                        expression_l,
                    } = self.collection_map(
                        Some(begin_t.loc),
                        self.blob.alloc_ref(),
                        Some(end_t.loc),
                    );

                    self.validate_sym_value(&value, expression_l);

                    return Sym::new_in(self.blob, |sym| {
                        sym.name = value;
                        sym.begin_l = begin_l;
                        sym.end_l = end_l;
                        sym.expression_l = expression_l;
                    });
                }
                _ => unreachable!(),
            }
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(Some(begin_t.loc), &parts, Some(end_t.loc));
        Dsym::new_in(self.blob, |dsym| {
            dsym.parts = parts;
            dsym.begin_l = begin_l;
            dsym.end_l = end_l;
            dsym.expression_l = expression_l;
        })
    }

    // Executable strings

    pub(crate) fn xstring_compose(
        &self,
        begin_t: &'b Token<'b>,
        parts: &'b NodeList<'b>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let begin_l = self.loc(begin_t);

        let mut begin = begin_t.token_value.iter();
        if begin.len() >= 2 && begin.next() == Some(b'<') && begin.next() == Some(b'<') {
            let heredoc_body_l = collection_expr(&parts).unwrap_or_else(|| self.loc(end_t));
            let heredoc_end_l = self.loc(end_t);
            let expression_l = begin_l;

            XHeredoc::new_in(self.blob, |xheredoc| {
                xheredoc.parts = parts;
                xheredoc.heredoc_body_l = heredoc_body_l;
                xheredoc.heredoc_end_l = heredoc_end_l;
                xheredoc.expression_l = expression_l;
            })
        } else {
            let end_l = self.loc(end_t);
            let expression_l = begin_l.join(end_l);

            Xstr::new_in(self.blob, |xstr| {
                xstr.parts = parts;
                xstr.begin_l = begin_l;
                xstr.end_l = end_l;
                xstr.expression_l = expression_l;
            })
        }
    }

    // Indented (interpolated, noninterpolated, executable) strings

    pub(crate) fn heredoc_dedent(&self, node: &'b Node<'b>, dedent_level: i32) -> &'b Node<'b> {
        if dedent_level == 0 {
            return node;
        }

        let dedent_level: usize = dedent_level
            .try_into()
            .expect("dedent_level must be positive");

        let dedent_heredoc_parts = |parts: &'b NodeList<'b>| -> &'b NodeList<'b> {
            let out = self.blob.alloc_ref::<NodeList>();

            for part in parts.iter() {
                match part {
                    Node::Str(Str {
                        value,
                        begin_l,
                        end_l,
                        expression_l,
                        ..
                    }) => {
                        let value = Self::dedent_string(value, dedent_level);
                        if !value.is_empty() {
                            let node = Str::new_in(self.blob, |str_| {
                                str_.value = value;
                                str_.begin_l = *begin_l;
                                str_.end_l = *end_l;
                                str_.expression_l = *expression_l;
                            });
                            out.push(node);
                        }
                    }
                    Node::Begin(_)
                    | Node::Gvar(_)
                    | Node::BackRef(_)
                    | Node::NthRef(_)
                    | Node::Ivar(_)
                    | Node::Cvar(_) => out.push(part),
                    other => {
                        unreachable!("unsupported heredoc child {}", other.str_type())
                    }
                }
            }
            out
        };

        match node {
            Node::Heredoc(Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
                ..
            }) => {
                let parts = dedent_heredoc_parts(parts);
                Heredoc::new_in(self.blob, |heredoc| {
                    heredoc.parts = parts;
                    heredoc.heredoc_body_l = *heredoc_body_l;
                    heredoc.heredoc_end_l = *heredoc_end_l;
                    heredoc.expression_l = *expression_l;
                })
            }
            Node::XHeredoc(XHeredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
                ..
            }) => {
                let parts = dedent_heredoc_parts(parts);
                XHeredoc::new_in(self.blob, |xheredoc| {
                    xheredoc.parts = parts;
                    xheredoc.heredoc_body_l = *heredoc_body_l;
                    xheredoc.heredoc_end_l = *heredoc_end_l;
                    xheredoc.expression_l = *expression_l;
                })
            }
            _ => {
                unreachable!("unsupported heredoc_dedent argument {}", node.str_type())
            }
        }
    }

    const TAB_WIDTH: usize = 8;

    pub(crate) fn dedent_string(s: &'b Bytes<'b>, width: usize) -> &'b Bytes<'b> {
        let mut col: usize = 0;
        let mut i: usize = 0;
        let len = s.len();

        loop {
            if !(i < len && col < width) {
                break;
            }

            if s.byte_at(i).unwrap() == b' ' {
                col += 1;
            } else if s.byte_at(i).unwrap() == b'\t' {
                let n = Self::TAB_WIDTH * (col / Self::TAB_WIDTH + 1);
                if n > Self::TAB_WIDTH {
                    break;
                }
                col = n;
            } else {
                break;
            }

            i += 1;
        }

        // Bytes::new(Vec::from(&s.as_raw()[i..]))
        todo!()
    }

    // Regular expressions

    pub(crate) fn regexp_options(&self, regexp_end_t: &'b Token<'b>) -> Option<&'b Node<'b>> {
        if regexp_end_t.loc.end() - regexp_end_t.loc.begin() == 1 {
            // no regexp options, only trailing "/"
            return None;
        }
        let expression_l = self.loc(regexp_end_t).adjust_begin(1);
        let options = regexp_end_t.token_value;
        let mut options_pool = [0; 100];
        let mut options_count = 0;
        for option in regexp_end_t.token_value.iter().skip(1) {
            if !options_pool[..options_count].contains(&option) {
                options_pool[options_count] = option;
                options_count += 1;
            }
        }
        let options = &mut options_pool[..options_count];
        options.sort();

        Some(RegOpt::new_in(self.blob, |reg_opt| {
            if !options.is_empty() {
                let options = core::str::from_utf8(options).unwrap();
                let options = self.blob.push_str(options);
                let bytes = self.blob.alloc_ref::<Bytes>();
                bytes.append_borrowed(options, self.blob);

                reg_opt.options = Some(bytes);
            }
            reg_opt.expression_l = expression_l;
        }))
    }

    pub(crate) fn regexp_compose(
        &self,
        begin_t: &'b Token<'b>,
        parts: &'b NodeList<'b>,
        end_t_l: Loc,
        options: Option<&'b Node<'b>>,
    ) -> &'b Node<'b> {
        let begin_l = self.loc(begin_t);
        let end_l = end_t_l.resize(1);
        let expression_l =
            begin_l.join(maybe_boxed_node_expr(options.as_deref()).unwrap_or(end_t_l));

        match options.as_deref() {
            Some(Node::RegOpt(RegOpt {
                options,
                expression_l,
                ..
            })) => self.validate_static_regexp(
                &parts,
                options.as_ref().and_then(|b| b.as_whole_string()),
                *expression_l,
            ),
            None => self.validate_static_regexp(&parts, None, expression_l),
            _ => unreachable!("must be Option<RegOpt>"),
        }

        Regexp::new_in(self.blob, |regexp| {
            regexp.parts = parts;
            regexp.options = options;
            regexp.begin_l = begin_l;
            regexp.end_l = end_l;
            regexp.expression_l = expression_l;
        })
    }

    // Arrays

    pub(crate) fn array(
        &self,
        begin_t: Option<&'b Token<'b>>,
        elements: &'b NodeList<'b>,
        end_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(
            begin_t.as_ref().map(|t| t.loc),
            &elements,
            end_t.as_ref().map(|t| t.loc),
        );

        Array::new_in(self.blob, |array| {
            array.elements = elements;
            array.begin_l = begin_l;
            array.end_l = end_l;
            array.expression_l = expression_l;
        })
    }

    pub(crate) fn splat(&self, star_t: &'b Token<'b>, value: Option<&'b Node<'b>>) -> &'b Node<'b> {
        let operator_l = self.loc(star_t);
        let expression_l = operator_l.maybe_join(maybe_boxed_node_expr(value.as_deref()));

        Splat::new_in(self.blob, |splat| {
            splat.value = value;
            splat.operator_l = operator_l;
            splat.expression_l = expression_l;
        })
    }

    pub(crate) fn word(&self, parts: &'b NodeList<'b>) -> &'b Node<'b> {
        if parts.len() == 1 {
            let part = parts.iter().next().unwrap();
            if matches!(part, Node::Str(_) | Node::Dstr(_)) {
                return part;
            }
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(None, &parts, None);

        Dstr::new_in(self.blob, |dstr| {
            dstr.parts = parts;
            dstr.begin_l = begin_l;
            dstr.end_l = end_l;
            dstr.expression_l = expression_l;
        })
    }

    pub(crate) fn words_compose(
        &self,
        begin_t: &'b Token<'b>,
        elements: &'b NodeList<'b>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let begin_l = self.loc(begin_t);
        let end_l = self.loc(end_t);
        let expression_l = begin_l.join(end_l);
        Array::new_in(self.blob, |array| {
            array.elements = elements;
            array.begin_l = Some(begin_l);
            array.end_l = Some(end_l);
            array.expression_l = expression_l;
        })
    }

    pub(crate) fn symbols_compose(
        &self,
        begin_t: &'b Token<'b>,
        parts: &'b NodeList<'b>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let composed = self.blob.alloc_ref::<NodeList>();
        for part in parts.iter() {
            match part {
                Node::Str(Str {
                    value,
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) => {
                    self.validate_sym_value(&value, *expression_l);
                    composed.push(Sym::new_in(self.blob, |sym| {
                        sym.name = value;
                        sym.begin_l = *begin_l;
                        sym.end_l = *end_l;
                        sym.expression_l = *expression_l;
                    }));
                }
                Node::Dstr(Dstr {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) => composed.push(Dsym::new_in(self.blob, |dsym| {
                    dsym.parts = parts;
                    dsym.begin_l = *begin_l;
                    dsym.end_l = *end_l;
                    dsym.expression_l = *expression_l;
                })),
                _ => composed.push(part),
            }
        }

        let begin_l = self.loc(begin_t);
        let end_l = self.loc(end_t);
        let expression_l = begin_l.join(end_l);
        Array::new_in(self.blob, |array| {
            array.elements = composed;
            array.begin_l = Some(begin_l);
            array.end_l = Some(end_l);
            array.expression_l = expression_l;
        })
    }

    // Hashes

    pub(crate) fn pair(
        &self,
        key: &'b Node<'b>,
        assoc_t: &'b Token<'b>,
        value: &'b Node<'b>,
    ) -> &'b Node<'b> {
        let operator_l = self.loc(assoc_t);
        let expression_l = join_exprs(&key, &value);

        Pair::new_in(self.blob, |pair| {
            pair.key = key;
            pair.value = value;
            pair.operator_l = operator_l;
            pair.expression_l = expression_l;
        })
    }

    pub(crate) fn pair_keyword(&self, key_t: &'b Token<'b>, value: &'b Node<'b>) -> &'b Node<'b> {
        let key_loc = self.loc(key_t);
        let key_l = key_loc.adjust_end(-1);
        let colon_l = key_loc.with_begin(key_loc.end() - 1);
        let expression_l = key_loc.join(value.expression());

        self.validate_sym_value(key_t.token_value, key_l);

        Pair::new_in(self.blob, |pair| {
            pair.key = Sym::new_in(self.blob, |sym| {
                sym.name = key_t.token_value;
                sym.begin_l = None;
                sym.end_l = None;
                sym.expression_l = key_l;
            });
            pair.value = value;
            pair.operator_l = colon_l;
            pair.expression_l = expression_l;
        })
    }

    pub(crate) fn pair_quoted(
        &self,
        begin_t: &'b Token<'b>,
        parts: &'b NodeList<'b>,
        end_t: &'b Token<'b>,
        value: &'b Node<'b>,
    ) -> &'b Node<'b> {
        let end_l = self.loc(end_t);

        let quote_loc = Loc::new(end_l.end() - 2, end_l.end() - 1);

        let colon_l = end_l.with_begin(end_l.end() - 1);

        let end_t: &'b Token<'b> =
            Token::new(end_t.token_type, end_t.token_value, quote_loc, self.blob);
        let expression_l = self.loc(begin_t).join(value.expression());

        Pair::new_in(self.blob, |pair| {
            pair.key = self.symbol_compose(begin_t, parts, end_t);
            pair.value = value;
            pair.operator_l = colon_l;
            pair.expression_l = expression_l;
        })
    }

    pub(crate) fn pair_label(&self, key_t: &'b Token<'b>) -> &'b Node<'b> {
        let key_l = self.loc(key_t);
        let value_l = key_l.adjust_end(-1);

        let label = key_t.token_value;
        let value = if label.is_lowercase() {
            Lvar::new_in(self.blob, |lvar| {
                lvar.name = label;
                lvar.expression_l = value_l;
            })
        } else {
            Const::new_in(self.blob, |const_| {
                const_.scope = None;
                const_.name = label;
                const_.double_colon_l = None;
                const_.name_l = value_l;
                const_.expression_l = value_l;
            })
        };

        self.pair_keyword(key_t, self.accessible(value))
    }

    pub(crate) fn kwsplat(&self, dstar_t: &'b Token<'b>, value: &'b Node<'b>) -> &'b Node<'b> {
        let operator_l = self.loc(dstar_t);
        let expression_l = value.expression().join(operator_l);

        Kwsplat::new_in(self.blob, |kwsplat| {
            kwsplat.value = value;
            kwsplat.operator_l = operator_l;
            kwsplat.expression_l = expression_l;
        })
    }

    pub(crate) fn associate(
        &self,
        begin_t: Option<&'b Token<'b>>,
        pairs: &'b NodeList<'b>,
        end_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        for i in 0..pairs.len() {
            for j in i + 1..pairs.len() {
                let key1 = if let Node::Pair(Pair { key, .. }) = pairs.node_at(i).unwrap() {
                    key
                } else {
                    // kwsplat
                    continue;
                };
                let key2 = if let Node::Pair(Pair { key, .. }) = pairs.node_at(j).unwrap() {
                    key
                } else {
                    // kwsplat
                    continue;
                };

                fn reg_opts_are_equal(left: Option<&Node>, right: Option<&Node>) -> bool {
                    match (left, right) {
                        (None, None) => true,
                        (
                            Some(Node::RegOpt(RegOpt { options: left, .. })),
                            Some(Node::RegOpt(RegOpt { options: right, .. })),
                        ) => {
                            left.map(|b| b.as_whole_string()) == right.map(|b| b.as_whole_string())
                        }
                        _ => false,
                    }
                }

                fn keys_are_equal(left: &Node, right: &Node) -> bool {
                    match (left, right) {
                        // sym
                        (
                            Node::Sym(Sym { name: name1, .. }),
                            Node::Sym(Sym { name: name2, .. }),
                        ) if name1.as_whole_string() == name2.as_whole_string() => true,

                        // str
                        (
                            Node::Str(Str { value: value1, .. }),
                            Node::Str(Str { value: value2, .. }),
                        ) if value1.as_whole_string() == value2.as_whole_string() => true,

                        // int
                        (
                            Node::Int(Int { value: value1, .. }),
                            Node::Int(Int { value: value2, .. }),
                        ) if value1.as_whole_string() == value2.as_whole_string() => true,

                        // float
                        (
                            Node::Float(Float { value: value1, .. }),
                            Node::Float(Float { value: value2, .. }),
                        ) if value1.as_whole_string() == value2.as_whole_string() => true,

                        // rational
                        (
                            Node::Rational(Rational { value: value1, .. }),
                            Node::Rational(Rational { value: value2, .. }),
                        ) if value1.as_whole_string() == value2.as_whole_string() => true,

                        // complex
                        (
                            Node::Complex(Complex { value: value1, .. }),
                            Node::Complex(Complex { value: value2, .. }),
                        ) if value1.as_whole_string() == value2.as_whole_string() => true,

                        // regexp
                        (
                            Node::Regexp(Regexp {
                                parts: parts1,
                                options: options1,
                                ..
                            }),
                            Node::Regexp(Regexp {
                                parts: parts2,
                                options: options2,
                                ..
                            }),
                        ) if reg_opts_are_equal(*options1, *options2) => {
                            parts1.len() == parts2.len()
                                && parts1
                                    .iter()
                                    .zip(parts2.iter())
                                    .all(|(child1, child2)| keys_are_equal(child1, child2))
                        }

                        _ => false,
                    }
                }

                let do_warn = keys_are_equal(key1, key2);

                if do_warn {
                    self.warn(DiagnosticMessage::DuplicateHashKey {}, key2.expression());
                }
            }
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(
            begin_t.as_ref().map(|t| t.loc),
            &pairs,
            end_t.as_ref().map(|t| t.loc),
        );

        Hash::new_in(self.blob, |hash| {
            hash.pairs = pairs;
            hash.begin_l = begin_l;
            hash.end_l = end_l;
            hash.expression_l = expression_l;
        })
    }

    // Ranges

    pub(crate) fn range_inclusive(
        &self,
        left: Option<&'b Node<'b>>,
        dot2_t: &'b Token<'b>,
        right: Option<&'b Node<'b>>,
    ) -> &'b Node<'b> {
        let operator_l = self.loc(dot2_t);
        let expression_l = operator_l
            .maybe_join(maybe_boxed_node_expr(left.as_deref()))
            .maybe_join(maybe_boxed_node_expr(right.as_deref()));

        Irange::new_in(self.blob, |irange| {
            irange.left = left;
            irange.right = right;
            irange.operator_l = operator_l;
            irange.expression_l = expression_l;
        })
    }

    pub(crate) fn range_exclusive(
        &self,
        left: Option<&'b Node<'b>>,
        dot3_t: &'b Token<'b>,
        right: Option<&'b Node<'b>>,
    ) -> &'b Node<'b> {
        let operator_l = self.loc(dot3_t);
        let expression_l = operator_l
            .maybe_join(maybe_boxed_node_expr(left.as_deref()))
            .maybe_join(maybe_boxed_node_expr(right.as_deref()));

        Erange::new_in(self.blob, |erange| {
            erange.left = left;
            erange.right = right;
            erange.operator_l = operator_l;
            erange.expression_l = expression_l;
        })
    }

    //
    // Access
    //

    pub(crate) fn self_(&self, token: &'b Token<'b>) -> &'b Node<'b> {
        Self_::new_in(self.blob, |self_| {
            self_.expression_l = self.loc(token);
        })
    }

    pub(crate) fn lvar(&self, token: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(token);
        Lvar::new_in(self.blob, |lvar| {
            lvar.name = token.token_value;
            lvar.expression_l = expression_l;
        })
    }

    pub(crate) fn ivar(&self, token: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(token);
        Ivar::new_in(self.blob, |ivar| {
            ivar.name = token.token_value;
            ivar.expression_l = expression_l;
        })
    }

    pub(crate) fn gvar(&self, token: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(token);
        Gvar::new_in(self.blob, |gvar| {
            gvar.name = token.token_value;
            gvar.expression_l = expression_l;
        })
    }

    pub(crate) fn cvar(&self, token: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(token);
        Cvar::new_in(self.blob, |cvar| {
            cvar.name = token.token_value;
            cvar.expression_l = expression_l;
        })
    }

    pub(crate) fn back_ref(&self, token: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(token);
        BackRef::new_in(self.blob, |back_ref| {
            back_ref.name = token.token_value;
            back_ref.expression_l = expression_l;
        })
    }

    const MAX_NTH_REF: usize = 0b111111111111111111111111111111;

    pub(crate) fn nth_ref(&self, token: &'b Token<'b>) -> &'b Node<'b> {
        let expression_l = self.loc(token);
        let name = &token.as_whole_str()[1..];
        let parsed = name.parse::<usize>();

        if parsed.is_err() || parsed.map(|n| n > Self::MAX_NTH_REF) == Ok(true) {
            self.warn(
                DiagnosticMessage::NthRefIsTooBig { nth_ref: name },
                expression_l,
            )
        }

        NthRef::new_in(self.blob, |nth_ref| {
            let name_without_dollar = self.blob.alloc_ref::<Bytes>();
            name_without_dollar.append_borrowed(name, self.blob);
            nth_ref.name = name_without_dollar;
            nth_ref.expression_l = expression_l;
        })
    }

    pub(crate) fn accessible(&self, node: &'b Node<'b>) -> &'b Node<'b> {
        match node {
            Node::Lvar(Lvar {
                name, expression_l, ..
            }) => {
                let name_s = name.as_whole_string().unwrap();

                if name_s.ends_with('?') || name_s.ends_with('!') {
                    self.error(
                        DiagnosticMessage::InvalidIdToGet {
                            identifier: self.blob.push_str(name_s),
                        },
                        *expression_l,
                    );
                }

                // Numbered parameters are not declared anywhere,
                // so they take precedence over method calls in numblock contexts
                if self.try_declare_numparam(name_s, *expression_l) {
                    return Lvar::new_in(self.blob, |lvar| {
                        lvar.name = *name;
                        lvar.expression_l = *expression_l;
                    });
                }

                if !self.static_env.is_declared(name_s) {
                    return Send::new_in(self.blob, |send| {
                        send.recv = None;
                        send.method_name = name;
                        send.dot_l = None;
                        send.selector_l = Some(*expression_l);
                        send.begin_l = None;
                        send.end_l = None;
                        send.operator_l = None;
                        send.expression_l = *expression_l;
                    });
                }

                if let Some(current_arg) = self.current_arg_stack.top() {
                    if current_arg == name_s {
                        self.error(
                            DiagnosticMessage::CircularArgumentReference { arg_name: name_s },
                            *expression_l,
                        );
                    }
                }

                node
            }
            _ => node,
        }
    }

    pub(crate) fn const_(&self, name_t: &'b Token<'b>) -> &'b Node<'b> {
        let name_l = self.loc(name_t);
        let expression_l = name_l;

        Const::new_in(self.blob, |const_| {
            const_.scope = None;
            const_.name = name_t.token_value;
            const_.double_colon_l = None;
            const_.name_l = name_l;
            const_.expression_l = expression_l;
        })
    }

    pub(crate) fn const_global(
        &self,
        t_colon3: &'b Token<'b>,
        name_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let scope = Cbase::new_in(self.blob, |cbase| {
            cbase.expression_l = self.loc(t_colon3);
        });

        let name_l = self.loc(name_t);
        let expression_l = scope.expression().join(name_l);
        let double_colon_l = self.loc(t_colon3);

        Const::new_in(self.blob, |const_| {
            const_.scope = Some(scope);
            const_.name = name_t.token_value;
            const_.double_colon_l = Some(double_colon_l);
            const_.name_l = name_l;
            const_.expression_l = expression_l;
        })
    }

    pub(crate) fn const_fetch(
        &self,
        scope: &'b Node<'b>,
        t_colon2: &'b Token<'b>,
        name_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let scope: &'b Node<'b> = scope;
        let name_l = self.loc(name_t);
        let expression_l = scope.expression().join(name_l);
        let double_colon_l = self.loc(t_colon2);

        Const::new_in(self.blob, |const_| {
            const_.scope = Some(scope);
            const_.name = name_t.token_value;
            const_.double_colon_l = Some(double_colon_l);
            const_.name_l = name_l;
            const_.expression_l = expression_l;
        })
    }

    pub(crate) fn __encoding__(&self, encoding_t: &'b Token<'b>) -> &'b Node<'b> {
        Encoding::new_in(self.blob, |encoding| {
            encoding.expression_l = self.loc(encoding_t);
        })
    }

    //
    // Assignments
    //

    pub(crate) fn assignable(&self, node: &'b Node<'b>) -> Result<&'b Node<'b>, ()> {
        let node = match node {
            Node::Cvar(Cvar {
                name, expression_l, ..
            }) => Cvasgn::new_in(self.blob, |cvasgn| {
                cvasgn.name = name;
                cvasgn.value = None;
                cvasgn.name_l = *expression_l;
                cvasgn.operator_l = None;
                cvasgn.expression_l = *expression_l;
            }),
            Node::Ivar(Ivar {
                name, expression_l, ..
            }) => Ivasgn::new_in(self.blob, |ivasgn| {
                ivasgn.name = name;
                ivasgn.value = None;
                ivasgn.name_l = *expression_l;
                ivasgn.operator_l = None;
                ivasgn.expression_l = *expression_l;
            }),
            Node::Gvar(Gvar {
                name, expression_l, ..
            }) => Gvasgn::new_in(self.blob, |gvasgn| {
                gvasgn.name = name;
                gvasgn.value = None;
                gvasgn.name_l = *expression_l;
                gvasgn.operator_l = None;
                gvasgn.expression_l = *expression_l;
            }),
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
                ..
            }) => {
                if self.context.in_def() {
                    self.error(
                        DiagnosticMessage::DynamicConstantAssignment {},
                        *expression_l,
                    );
                    return Err(());
                }
                Casgn::new_in(self.blob, |casgn| {
                    casgn.scope = *scope;
                    casgn.name = name;
                    casgn.value = None;
                    casgn.double_colon_l = *double_colon_l;
                    casgn.name_l = *name_l;
                    casgn.operator_l = None;
                    casgn.expression_l = *expression_l;
                })
            }
            Node::Lvar(Lvar {
                name, expression_l, ..
            }) => {
                let name_s = name.as_whole_string().unwrap();
                self.check_assignment_to_numparam(name_s, *expression_l)?;
                self.check_reserved_for_numparam(name_s, *expression_l)?;

                self.static_env.declare(name_s, self.blob);

                Lvasgn::new_in(self.blob, |lvasgn| {
                    lvasgn.name = name;
                    lvasgn.value = None;
                    lvasgn.name_l = *expression_l;
                    lvasgn.operator_l = None;
                    lvasgn.expression_l = *expression_l;
                })
            }
            Node::MatchVar(MatchVar {
                name,
                name_l,
                expression_l,
                ..
            }) => {
                let name_s = name.as_whole_string().unwrap();
                self.check_assignment_to_numparam(name_s, *name_l)?;
                self.check_reserved_for_numparam(name_s, *name_l)?;

                MatchVar::new_in(self.blob, |match_var| {
                    match_var.name = name;
                    match_var.name_l = *name_l;
                    match_var.expression_l = *expression_l;
                })
            }
            Node::Self_(Self_ { expression_l, .. }) => {
                self.error(DiagnosticMessage::CantAssignToSelf {}, *expression_l);
                return Err(());
            }
            Node::Nil(Nil { expression_l, .. }) => {
                self.error(DiagnosticMessage::CantAssignToNil {}, *expression_l);
                return Err(());
            }
            Node::True(True { expression_l, .. }) => {
                self.error(DiagnosticMessage::CantAssignToTrue {}, *expression_l);
                return Err(());
            }
            Node::False(False { expression_l, .. }) => {
                self.error(DiagnosticMessage::CantAssignToFalse {}, *expression_l);
                return Err(());
            }
            Node::File(File { expression_l, .. }) => {
                self.error(DiagnosticMessage::CantAssignToFile {}, *expression_l);
                return Err(());
            }
            Node::Line(Line { expression_l, .. }) => {
                self.error(DiagnosticMessage::CantAssignToLine {}, *expression_l);
                return Err(());
            }
            Node::Encoding(Encoding { expression_l, .. }) => {
                self.error(DiagnosticMessage::CantAssignToEncoding {}, *expression_l);
                return Err(());
            }
            Node::BackRef(BackRef {
                name, expression_l, ..
            }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable {
                        var_name: name.as_whole_string().unwrap(),
                    },
                    *expression_l,
                );
                return Err(());
            }
            Node::NthRef(NthRef {
                name, expression_l, ..
            }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable {
                        var_name: self
                            .blob
                            .push_str(&format!("${}", name.as_whole_string().unwrap())),
                    },
                    *expression_l,
                );
                return Err(());
            }
            other => unreachable!("{:?} can't be used in assignment", other),
        };

        Ok(node)
    }

    pub(crate) fn const_op_assignable(&self, node: &'b Node<'b>) -> &'b Node<'b> {
        match node {
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
                ..
            }) => Casgn::new_in(self.blob, |casgn| {
                casgn.scope = *scope;
                casgn.name = name;
                casgn.value = None;
                casgn.double_colon_l = *double_colon_l;
                casgn.name_l = *name_l;
                casgn.operator_l = None;
                casgn.expression_l = *expression_l;
            }),
            other => {
                unreachable!("unsupported const_op_assignable argument: {:?}", other)
            }
        }
    }

    pub(crate) fn assign(
        &self,
        lhs: &'b Node<'b>,
        eql_t: &'b Token<'b>,
        new_rhs: &'b Node<'b>,
    ) -> &'b Node<'b> {
        let op_l = Some(self.loc(eql_t));
        let expr_l = join_exprs(&lhs, &new_rhs);

        match lhs {
            Node::Cvasgn(Cvasgn {
                name,
                value,
                name_l,
                expression_l,
                operator_l,
                ..
            }) => Cvasgn::new_in(self.blob, |cvasgn| {
                cvasgn.name = *name;
                cvasgn.value = Some(new_rhs);
                cvasgn.name_l = *name_l;
                cvasgn.operator_l = op_l;
                cvasgn.expression_l = expr_l;
            }),
            Node::Ivasgn(Ivasgn {
                name,
                value,
                name_l,
                expression_l,
                operator_l,
                ..
            }) => Ivasgn::new_in(self.blob, |ivasgn| {
                ivasgn.name = *name;
                ivasgn.value = Some(new_rhs);
                ivasgn.name_l = *name_l;
                ivasgn.operator_l = op_l;
                ivasgn.expression_l = expr_l;
            }),
            Node::Gvasgn(Gvasgn {
                name,
                value,
                name_l,
                expression_l,
                operator_l,
                ..
            }) => Gvasgn::new_in(self.blob, |gvasgn| {
                gvasgn.name = *name;
                gvasgn.value = Some(new_rhs);
                gvasgn.name_l = *name_l;
                gvasgn.operator_l = op_l;
                gvasgn.expression_l = expr_l;
            }),
            Node::Lvasgn(Lvasgn {
                name,
                value,
                name_l,
                expression_l,
                operator_l,
                ..
            }) => Lvasgn::new_in(self.blob, |lvasgn| {
                lvasgn.name = *name;
                lvasgn.value = Some(new_rhs);
                lvasgn.name_l = *name_l;
                lvasgn.operator_l = op_l;
                lvasgn.expression_l = expr_l;
            }),
            Node::Casgn(Casgn {
                scope,
                name,
                value,
                double_colon_l,
                name_l,
                operator_l,
                expression_l,
                ..
            }) => Casgn::new_in(self.blob, |casgn| {
                casgn.scope = *scope;
                casgn.name = *name;
                casgn.value = Some(new_rhs);
                casgn.double_colon_l = *double_colon_l;
                casgn.name_l = *name_l;
                casgn.operator_l = op_l;
                casgn.expression_l = expr_l;
            }),
            Node::IndexAsgn(IndexAsgn {
                recv,
                indexes,
                value,
                begin_l,
                end_l,
                operator_l,
                expression_l,
                ..
            }) => IndexAsgn::new_in(self.blob, |index_asgn| {
                index_asgn.recv = *recv;
                index_asgn.indexes = *indexes;
                index_asgn.value = Some(new_rhs);
                index_asgn.begin_l = *begin_l;
                index_asgn.end_l = *end_l;
                index_asgn.operator_l = op_l;
                index_asgn.expression_l = expr_l;
            }),
            Node::Send(Send {
                recv,
                method_name,
                args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                operator_l,
                expression_l,
                ..
            }) => Send::new_in(self.blob, |send| {
                send.recv = *recv;
                send.method_name = *method_name;
                if args.is_empty() {
                    send.args.push(new_rhs);
                } else {
                    unreachable!("can't assign to method call with args")
                }
                send.dot_l = *dot_l;
                send.selector_l = *selector_l;
                send.begin_l = *begin_l;
                send.end_l = *end_l;
                send.operator_l = op_l;
                send.expression_l = expr_l;
            }),
            Node::CSend(CSend {
                recv,
                method_name,
                args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                operator_l,
                expression_l,
                ..
            }) => CSend::new_in(self.blob, |csend| {
                csend.recv = *recv;
                csend.method_name = *method_name;
                if args.is_empty() {
                    csend.args.push(new_rhs);
                } else {
                    unreachable!("can't assign to method call with args")
                }
                csend.dot_l = *dot_l;
                csend.selector_l = *selector_l;
                csend.begin_l = *begin_l;
                csend.end_l = *end_l;
                csend.operator_l = op_l;
                csend.expression_l = expr_l;
            }),
            other => unreachable!("{:?} can't be used in assignment", other),
        }
    }

    pub(crate) fn op_assign(
        &self,
        lhs: &'b Node<'b>,
        op_t: &'b Token<'b>,
        rhs: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // let operator_l = self.loc(op_t);
        // let mut operator = value(op_t);
        // operator.pop();
        // let expression_l = join_exprs(&lhs, &rhs);

        // match &*lhs {
        //     Node::Gvasgn(_)
        //     | Node::Ivasgn(_)
        //     | Node::Lvasgn(_)
        //     | Node::Cvasgn(_)
        //     | Node::Casgn(_)
        //     | Node::Send(_)
        //     | Node::CSend(_) => {
        //         // ignore
        //     }
        //     Node::Index(_) => match *lhs {
        //         Node::Index(Index {
        //             recv,
        //             indexes,
        //             begin_l,
        //             end_l,
        //             expression_l,
        //         }) => {
        //             lhs = Box::new(Node::IndexAsgn(IndexAsgn {
        //                 recv,
        //                 indexes,
        //                 value: None,
        //                 begin_l,
        //                 end_l,
        //                 operator_l: None,
        //                 expression_l,
        //             }));
        //         }
        //         _ => unreachable!(),
        //     },
        //     Node::BackRef(BackRef { name, expression_l }) => {
        //         self.error(
        //             DiagnosticMessage::CantSetVariable {
        //                 var_name: self.blob.push_str(name),
        //             },
        //             *expression_l,
        //         );
        //         return Err(());
        //     }
        //     Node::NthRef(NthRef { name, expression_l }) => {
        //         self.error(
        //             DiagnosticMessage::CantSetVariable {
        //                 var_name: self.blob.push_str(&format!("${}", name)),
        //             },
        //             *expression_l,
        //         );
        //         return Err(());
        //     }
        //     _ => unreachable!("unsupported op_assign lhs {:?}", lhs),
        // }

        // let recv: &'b Node<'b> = lhs;
        // let value: &'b Node<'b> = rhs;

        // let result = match &operator[..] {
        //     "&&" => Node::AndAsgn(AndAsgn {
        //         recv,
        //         value,
        //         operator_l,
        //         expression_l,
        //     }),
        //     "||" => Node::OrAsgn(OrAsgn {
        //         recv,
        //         value,
        //         operator_l,
        //         expression_l,
        //     }),
        //     _ => Node::OpAsgn(OpAsgn {
        //         recv,
        //         operator,
        //         value,
        //         operator_l,
        //         expression_l,
        //     }),
        // };

        // Ok(Box::new(result))
        todo!()
    }

    pub(crate) fn multi_lhs(
        &self,
        begin_t: Option<&'b Token<'b>>,
        items: &'b NodeList<'b>,
        end_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(
            begin_t.as_ref().map(|t| t.loc),
            &items,
            end_t.as_ref().map(|t| t.loc),
        );

        Mlhs::new_in(self.blob, |mlhs| {
            mlhs.items = items;
            mlhs.begin_l = begin_l;
            mlhs.end_l = end_l;
            mlhs.expression_l = expression_l;
        })
    }

    pub(crate) fn multi_assign(
        &self,
        lhs: &'b Node<'b>,
        eql_t: &'b Token<'b>,
        rhs: &'b Node<'b>,
    ) -> &'b Node<'b> {
        let operator_l = self.loc(eql_t);
        let expression_l = join_exprs(&lhs, &rhs);

        Masgn::new_in(self.blob, |masgn| {
            masgn.lhs = lhs;
            masgn.rhs = rhs;
            masgn.operator_l = operator_l;
            masgn.expression_l = expression_l;
        })
    }

    //
    // Class and module definition
    //

    pub(crate) fn def_class(
        &self,
        class_t: &'b Token<'b>,
        name: &'b Node<'b>,
        lt_t: Option<&'b Token<'b>>,
        superclass: Option<&'b Node<'b>>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let keyword_l = self.loc(class_t);
        let end_l = self.loc(end_t);
        let operator_l = self.maybe_loc(lt_t);
        let expression_l = keyword_l.join(end_l);

        Class::new_in(self.blob, |class| {
            class.name = name;
            class.superclass = superclass;
            class.body = body;
            class.keyword_l = keyword_l;
            class.operator_l = operator_l;
            class.end_l = end_l;
            class.expression_l = expression_l;
        })
    }

    pub(crate) fn def_sclass(
        &self,
        class_t: &'b Token<'b>,
        lshift_t: &'b Token<'b>,
        expr: &'b Node<'b>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let keyword_l = self.loc(class_t);
        let end_l = self.loc(end_t);
        let operator_l = self.loc(lshift_t);
        let expression_l = keyword_l.join(end_l);

        SClass::new_in(self.blob, |sclass| {
            sclass.expr = expr;
            sclass.body = body;
            sclass.keyword_l = keyword_l;
            sclass.operator_l = operator_l;
            sclass.end_l = end_l;
            sclass.expression_l = expression_l;
        })
    }

    pub(crate) fn def_module(
        &self,
        module_t: &'b Token<'b>,
        name: &'b Node<'b>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let keyword_l = self.loc(module_t);
        let end_l = self.loc(end_t);
        let expression_l = keyword_l.join(end_l);

        Module::new_in(self.blob, |module| {
            module.name = name;
            module.body = body;
            module.keyword_l = keyword_l;
            module.end_l = end_l;
            module.expression_l = expression_l;
        })
    }

    //
    // Method (un)definition
    //

    pub(crate) fn def_method(
        &self,
        def_t: &'b Token<'b>,
        name_t: &'b Token<'b>,
        args: Option<&'b Node<'b>>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        let name_l = self.loc(name_t);
        let keyword_l = self.loc(def_t);
        let end_l = self.loc(end_t);
        let expression_l = keyword_l.join(end_l);

        self.check_reserved_for_numparam(name_t.as_whole_str(), name_l)?;

        Ok(Def::new_in(self.blob, |def| {
            def.name = name_t.token_value;
            def.args = args;
            def.body = body;
            def.keyword_l = keyword_l;
            def.name_l = name_l;
            def.end_l = Some(end_l);
            def.assignment_l = None;
            def.expression_l = expression_l;
        }))
    }

    pub(crate) fn def_endless_method(
        &self,
        def_t: &'b Token<'b>,
        name_t: &'b Token<'b>,
        args: Option<&'b Node<'b>>,
        assignment_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        // let body_l = maybe_boxed_node_expr(body.as_deref())
        //     .unwrap_or_else(|| unreachable!("endless method always has a body"));

        // let keyword_l = self.loc(def_t);
        // let expression_l = keyword_l.join(&body_l);
        // let name_l = self.loc(name_t);
        // let assignment_l = self.loc(assignment_t);

        // let name = value(name_t);
        // self.check_reserved_for_numparam(name.as_str(), name_l)?;

        // Ok(Box::new(Node::Def(Def {
        //     name,
        //     args,
        //     body,
        //     keyword_l,
        //     name_l,
        //     end_l: None,
        //     assignment_l: Some(assignment_l),
        //     expression_l,
        // })))
        todo!()
    }

    pub(crate) fn def_singleton(
        &self,
        def_t: &'b Token<'b>,
        definee: &'b Node<'b>,
        dot_t: &'b Token<'b>,
        name_t: &'b Token<'b>,
        args: Option<&'b Node<'b>>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        let keyword_l = self.loc(def_t);
        let operator_l = self.loc(dot_t);
        let name_l = self.loc(name_t);
        let end_l = self.loc(end_t);
        let expression_l = keyword_l.join(end_l);

        let name = name_t.token_value;
        self.check_reserved_for_numparam(name.as_whole_string().unwrap(), name_l)?;

        Ok(Defs::new_in(self.blob, |defs| {
            defs.definee = definee;
            defs.name = name;
            defs.args = args;
            defs.body = body;
            defs.keyword_l = keyword_l;
            defs.operator_l = operator_l;
            defs.name_l = name_l;
            defs.assignment_l = None;
            defs.end_l = Some(end_l);
            defs.expression_l = expression_l;
        }))
    }

    pub(crate) fn def_endless_singleton(
        &self,
        def_t: &'b Token<'b>,
        definee: &'b Node<'b>,
        dot_t: &'b Token<'b>,
        name_t: &'b Token<'b>,
        args: Option<&'b Node<'b>>,
        assignment_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        // let body_l = maybe_boxed_node_expr(body.as_deref())
        //     .unwrap_or_else(|| unreachable!("endless method always has body"));

        // let keyword_l = self.loc(def_t);
        // let operator_l = self.loc(dot_t);
        // let name_l = self.loc(name_t);
        // let assignment_l = self.loc(assignment_t);
        // let expression_l = keyword_l.join(&body_l);

        // let name = value(name_t);
        // self.check_reserved_for_numparam(name.as_str(), name_l)?;

        // Ok(Box::new(Node::Defs(Defs {
        //     definee,
        //     name,
        //     args,
        //     body,
        //     keyword_l,
        //     operator_l,
        //     name_l,
        //     assignment_l: Some(assignment_l),
        //     end_l: None,
        //     expression_l,
        // })))
        todo!()
    }

    pub(crate) fn undef_method(
        &self,
        undef_t: &'b Token<'b>,
        names: &'b NodeList<'b>,
    ) -> &'b Node<'b> {
        // let keyword_l = self.loc(undef_t);
        // let expression_l = keyword_l.maybe_join(&collection_expr(&names));
        // Box::new(Node::Undef(Undef {
        //     names,
        //     keyword_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn alias(
        &self,
        alias_t: &'b Token<'b>,
        to: &'b Node<'b>,
        from: &'b Node<'b>,
    ) -> &'b Node<'b> {
        let keyword_l = self.loc(alias_t);
        let expression_l = keyword_l.join(from.expression());
        Alias::new_in(self.blob, |alias| {
            alias.to = to;
            alias.from = from;
            alias.keyword_l = keyword_l;
            alias.expression_l = expression_l;
        })
    }

    //
    // Formal arguments
    //

    pub(crate) fn args(
        &self,
        begin_t: Option<&'b Token<'b>>,
        args: &'b NodeList<'b>,
        end_t: Option<&'b Token<'b>>,
    ) -> Option<&'b Node<'b>> {
        self.check_duplicate_args(&args, &mut HashMap::new());
        self.validate_no_forward_arg_after_restarg(&args);

        if begin_t.is_none() && args.is_empty() && end_t.is_none() {
            return None;
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(
            begin_t.as_ref().map(|t| t.loc),
            &args,
            end_t.as_ref().map(|t| t.loc),
        );

        Some(Args::new_in(self.blob, |args_node| {
            args_node.args = args;
            args_node.expression_l = expression_l;
            args_node.begin_l = begin_l;
            args_node.end_l = end_l;
        }))
    }

    pub(crate) fn forward_arg(&self, dots_t: &'b Token<'b>) -> &'b Node<'b> {
        ForwardArg::new_in(self.blob, |farg| {
            farg.expression_l = self.loc(dots_t);
        })
    }

    pub(crate) fn arg(&self, name_t: &'b Token<'b>) -> Result<&'b Node<'b>, ()> {
        let name_l = self.loc(name_t);

        self.check_reserved_for_numparam(name_t.as_whole_str(), name_l)?;

        Ok(Arg::new_in(self.blob, |arg| {
            arg.name = name_t.token_value;
            arg.expression_l = name_l;
        }))
    }

    pub(crate) fn optarg(
        &self,
        name_t: &'b Token<'b>,
        eql_t: &'b Token<'b>,
        default: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        let operator_l = self.loc(eql_t);
        let name_l = self.loc(name_t);
        let expression_l = self.loc(name_t).join(default.expression());

        let name = name_t.token_value;
        self.check_reserved_for_numparam(name.as_whole_string().unwrap(), name_l)?;

        Ok(Optarg::new_in(self.blob, |optarg| {
            optarg.name = name;
            optarg.default = default;
            optarg.name_l = name_l;
            optarg.operator_l = operator_l;
            optarg.expression_l = expression_l;
        }))
    }

    pub(crate) fn restarg(
        &self,
        star_t: &'b Token<'b>,
        name_t: Option<&'b Token<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        let (name, name_l) = if let Some(name_t) = name_t {
            let name_l = self.loc(name_t);
            let name = name_t.token_value;
            self.check_reserved_for_numparam(name.as_whole_string().unwrap(), name_l)?;
            (Some(name), Some(name_l))
        } else {
            (None, None)
        };

        let operator_l = self.loc(star_t);
        let expression_l = operator_l.maybe_join(name_l);

        Ok(Restarg::new_in(self.blob, |restarg| {
            restarg.name = name;
            restarg.operator_l = operator_l;
            restarg.name_l = name_l;
            restarg.expression_l = expression_l;
        }))
    }

    pub(crate) fn kwarg(&self, name_t: &'b Token<'b>) -> Result<&'b Node<'b>, ()> {
        let name_l = self.loc(name_t);
        let name = name_t.token_value;
        self.check_reserved_for_numparam(name.as_whole_string().unwrap(), name_l)?;

        let expression_l = name_l;
        let name_l = expression_l.adjust_end(-1);

        Ok(Kwarg::new_in(self.blob, |kwarg| {
            kwarg.name = name;
            kwarg.name_l = name_l;
            kwarg.expression_l = expression_l;
        }))
    }

    pub(crate) fn kwoptarg(
        &self,
        name_t: &'b Token<'b>,
        default: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        let name_l = self.loc(name_t);
        let name = name_t.token_value;
        self.check_reserved_for_numparam(name.as_whole_string().unwrap(), name_l)?;

        let label_l = name_l;
        let name_l = label_l.adjust_end(-1);
        let expression_l = default.expression().join(label_l);

        Ok(Kwoptarg::new_in(self.blob, |kwoptarg| {
            kwoptarg.name = name;
            kwoptarg.default = default;
            kwoptarg.name_l = name_l;
            kwoptarg.expression_l = expression_l;
        }))
    }

    pub(crate) fn kwrestarg(
        &self,
        dstar_t: &'b Token<'b>,
        name_t: Option<&'b Token<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        let (name, name_l) = if let Some(name_t) = name_t {
            let name_l = self.loc(name_t);
            let name = name_t.token_value;
            self.check_reserved_for_numparam(name.as_whole_string().unwrap(), name_l)?;
            (Some(name), Some(name_l))
        } else {
            (None, None)
        };

        let operator_l = self.loc(dstar_t);
        let expression_l = operator_l.maybe_join(name_l);

        Ok(Kwrestarg::new_in(self.blob, |kwrestarg| {
            kwrestarg.name = name;
            kwrestarg.operator_l = operator_l;
            kwrestarg.name_l = name_l;
            kwrestarg.expression_l = expression_l;
        }))
    }

    pub(crate) fn kwnilarg(&self, dstar_t: &'b Token<'b>, nil_t: &'b Token<'b>) -> &'b Node<'b> {
        let dstar_l = self.loc(dstar_t);
        let nil_l = self.loc(nil_t);
        let expression_l = dstar_l.join(nil_l);
        Kwnilarg::new_in(self.blob, |kwnilarg| {
            kwnilarg.name_l = nil_l;
            kwnilarg.expression_l = expression_l;
        })
    }

    pub(crate) fn shadowarg(&self, name_t: &'b Token<'b>) -> Result<&'b Node<'b>, ()> {
        // let name_l = self.loc(name_t);
        // let name = value(name_t);
        // self.check_reserved_for_numparam(name.as_str(), name_l)?;

        // Ok(Box::new(Node::Shadowarg(Shadowarg {
        //     name,
        //     expression_l: name_l,
        // })))
        todo!()
    }

    pub(crate) fn blockarg(
        &self,
        amper_t: &'b Token<'b>,
        name_t: Option<&'b Token<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        let name_l = self.maybe_loc(name_t);
        let name = maybe_value(name_t);
        if let (Some(name_l), Some(name)) = (name_l, name) {
            self.check_reserved_for_numparam(name.as_whole_string().unwrap(), name_l)?;
        }

        let operator_l = self.loc(amper_t);
        let expression_l = operator_l.maybe_join(name_l);

        Ok(Blockarg::new_in(self.blob, |blockarg| {
            blockarg.name = name;
            blockarg.operator_l = operator_l;
            blockarg.name_l = name_l;
            blockarg.expression_l = expression_l;
        }))
    }

    pub(crate) fn procarg0(&self, arg: &'b Node<'b>) -> &'b Node<'b> {
        match arg {
            Node::Mlhs(Mlhs {
                items,
                begin_l,
                end_l,
                expression_l,
                ..
            }) => Procarg0::new_in(self.blob, |procarg0| {
                procarg0.args = items;
                procarg0.begin_l = *begin_l;
                procarg0.end_l = *end_l;
                procarg0.expression_l = *expression_l;
            }),
            Node::Arg(Arg {
                name, expression_l, ..
            }) => Procarg0::new_in(self.blob, |procarg0| {
                procarg0.args.push(arg);
                procarg0.begin_l = None;
                procarg0.end_l = None;
                procarg0.expression_l = *expression_l;
            }),
            other => {
                unreachable!("unsupported procarg0 child {:?}", other)
            }
        }
    }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: Option<&'b Token<'b>>) -> MethodCallType {
        match dot_t.as_ref() {
            Some(token) if token.token_type == Lexer::tANDDOT => MethodCallType::CSend,
            _ => MethodCallType::Send,
        }
    }

    pub(crate) fn forwarded_args(&self, dots_t: &'b Token<'b>) -> &'b Node<'b> {
        // Box::new(Node::ForwardedArgs(ForwardedArgs {
        //     expression_l: self.loc(dots_t),
        // }))
        todo!()
    }

    pub(crate) fn call_method(
        &self,
        receiver: Option<&'b Node<'b>>,
        dot_t: Option<&'b Token<'b>>,
        selector_t: Option<&'b Token<'b>>,
        lparen_t: Option<&'b Token<'b>>,
        args: &'b NodeList<'b>,
        rparen_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        let begin_l = maybe_boxed_node_expr(receiver.as_deref())
            .or_else(|| self.maybe_loc(selector_t))
            .unwrap_or_else(|| unreachable!("can't compute begin_l"));
        let end_l = self
            .maybe_loc(rparen_t)
            .or_else(|| maybe_node_expr(args.last()))
            .or_else(|| self.maybe_loc(selector_t))
            .unwrap_or_else(|| unreachable!("can't compute end_l"));

        let expression_l = begin_l.join(end_l);

        let dot_l = self.maybe_loc(dot_t);
        let selector_l = self.maybe_loc(selector_t);
        let begin_l = self.maybe_loc(lparen_t);
        let end_l = self.maybe_loc(rparen_t);

        let method_name = selector_t.map(|t| t.token_value).unwrap_or_else(|| {
            let bytes = self.blob.alloc_ref::<Bytes>();
            bytes.append_borrowed("call", self.blob);
            bytes
        });

        self.rewrite_hash_args_to_kwargs(&args);

        match self.call_type_for_dot(dot_t) {
            MethodCallType::Send => Send::new_in(self.blob, |send| {
                send.recv = receiver;
                send.method_name = method_name;
                send.args = args;
                send.dot_l = dot_l;
                send.selector_l = selector_l;
                send.begin_l = begin_l;
                send.end_l = end_l;
                send.operator_l = None;
                send.expression_l = expression_l;
            }),

            MethodCallType::CSend => CSend::new_in(self.blob, |csend| {
                csend.recv = receiver.expect("csend node must have a receiver");
                csend.method_name = method_name;
                csend.args = args;
                csend.dot_l = dot_l.expect("csend node must have &.");
                csend.selector_l = selector_l;
                csend.begin_l = begin_l;
                csend.end_l = end_l;
                csend.operator_l = None;
                csend.expression_l = expression_l;
            }),
        }
    }

    pub(crate) fn call_lambda(&self, lambda_t: &'b Token<'b>) -> &'b Node<'b> {
        // Box::new(Node::Lambda(Lambda {
        //     expression_l: self.loc(lambda_t),
        // }))
        todo!()
    }

    pub(crate) fn block(
        &self,
        method_call: &'b Node<'b>,
        begin_t: &'b Token<'b>,
        block_args: ArgsType,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // let block_body = body;

        // let validate_block_and_block_arg = |args: &'b NodeList<'b>| {
        //     if let Some(last_arg) = args.last() {
        //         match last_arg {
        //             Node::BlockPass(_) | Node::ForwardedArgs(_) => {
        //                 self.error(
        //                     DiagnosticMessage::BlockAndBlockArgGiven {},
        //                     *last_arg.expression(),
        //                 );
        //                 Err(())
        //             }
        //             _ => Ok(()),
        //         }
        //     } else {
        //         Ok(())
        //     }
        // };

        // match &*method_call {
        //     Node::Yield(Yield { keyword_l, .. }) => {
        //         self.error(DiagnosticMessage::BlockGivenToYield {}, *keyword_l);
        //         return Err(());
        //     }
        //     Node::Send(Send { args, .. }) => {
        //         validate_block_and_block_arg(args)?;
        //     }
        //     Node::CSend(CSend { args, .. }) => {
        //         validate_block_and_block_arg(args)?;
        //     }
        //     _ => {}
        // }

        // let rewrite_args_and_loc =
        //     |method_args: &'b NodeList<'b>,
        //      keyword_expression_l: Loc,
        //      block_args: ArgsType,
        //      block_body: Option<&'b Node<'b>>| {
        //         // Code like "return foo 1 do end" is reduced in a weird sequence.
        //         // Here, method_call is actually (return).
        //         let actual_send = method_args.into_iter().next().unwrap();

        //         let begin_l = self.loc(begin_t);
        //         let end_l = self.loc(end_t);
        //         let expression_l = actual_send.expression().join(&end_l);

        //         let block = match block_args {
        //             ArgsType::Args(args) => Node::Block(Block {
        //                 call: Box::new(actual_send),
        //                 args,
        //                 body: block_body,
        //                 begin_l,
        //                 end_l,
        //                 expression_l,
        //             }),
        //             ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
        //                 call: Box::new(actual_send),
        //                 numargs,
        //                 body: block_body.unwrap_or_else(|| {
        //                     Box::new(Node::Nil(Nil {
        //                         expression_l: Loc { begin: 0, end: 0 },
        //                     }))
        //                 }),
        //                 begin_l,
        //                 end_l,
        //                 expression_l,
        //             }),
        //         };

        //         let expr_l = keyword_expression_l.join(block.expression());

        //         (vec![block], expr_l)
        //     };

        // match &*method_call {
        //     Node::Send(_)
        //     | Node::CSend(_)
        //     | Node::Index(_)
        //     | Node::Super(_)
        //     | Node::ZSuper(_)
        //     | Node::Lambda(_) => {
        //         let begin_l = self.loc(begin_t);
        //         let end_l = self.loc(end_t);
        //         let expression_l = method_call.expression().join(&end_l);

        //         let result = match block_args {
        //             ArgsType::Args(args) => Node::Block(Block {
        //                 call: method_call,
        //                 args,
        //                 body: block_body,
        //                 begin_l,
        //                 end_l,
        //                 expression_l,
        //             }),
        //             ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
        //                 call: method_call,
        //                 numargs,
        //                 body: block_body.unwrap_or_else(|| {
        //                     Box::new(Node::Nil(Nil {
        //                         expression_l: Loc { begin: 0, end: 0 },
        //                     }))
        //                 }),
        //                 begin_l,
        //                 end_l,
        //                 expression_l,
        //             }),
        //         };
        //         return Ok(Box::new(result));
        //     }
        //     _ => {}
        // }

        // let method_call = method_call;
        // let result = match *method_call {
        //     Node::Return(Return {
        //         args,
        //         keyword_l,
        //         expression_l,
        //     }) => {
        //         let (args, expression_l) =
        //             rewrite_args_and_loc(args, expression_l, block_args, block_body);
        //         Node::Return(Return {
        //             args,
        //             keyword_l,
        //             expression_l,
        //         })
        //     }
        //     Node::Next(Next {
        //         args,
        //         keyword_l,
        //         expression_l,
        //     }) => {
        //         let (args, expression_l) =
        //             rewrite_args_and_loc(args, expression_l, block_args, block_body);
        //         Node::Next(Next {
        //             args,
        //             keyword_l,
        //             expression_l,
        //         })
        //     }
        //     Node::Break(Break {
        //         args,
        //         keyword_l,
        //         expression_l,
        //     }) => {
        //         let (args, expression_l) =
        //             rewrite_args_and_loc(args, expression_l, block_args, block_body);
        //         Node::Break(Break {
        //             args,
        //             keyword_l,
        //             expression_l,
        //         })
        //     }
        //     other => {
        //         unreachable!("unsupported method call {:?}", other)
        //     }
        // };

        // Ok(Box::new(result))
        todo!()
    }
    pub(crate) fn block_pass(
        &self,
        amper_t: &'b Token<'b>,
        value: Option<&'b Node<'b>>,
    ) -> &'b Node<'b> {
        let amper_l = self.loc(amper_t);
        let expression_l = amper_l.maybe_join(value.as_ref().map(|node| node.expression()));

        BlockPass::new_in(self.blob, |block_pass| {
            block_pass.value = value;
            block_pass.operator_l = amper_l;
            block_pass.expression_l = expression_l;
        })
    }

    pub(crate) fn attr_asgn(
        &self,
        receiver: &'b Node<'b>,
        dot_t: &'b Token<'b>,
        selector_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let dot_l = self.loc(dot_t);
        // let selector_l = self.loc(selector_t);
        // let expression_l = receiver.expression().join(&selector_l);
        // let receiver: &'b Node<'b> = receiver;

        // let method_name = value(selector_t) + "=";

        // match self.call_type_for_dot(Some(dot_t)) {
        //     MethodCallType::Send => Box::new(Node::Send(Send {
        //         recv: Some(receiver),
        //         method_name,
        //         args: vec![],
        //         dot_l: Some(dot_l),
        //         selector_l: Some(selector_l),
        //         begin_l: None,
        //         end_l: None,
        //         operator_l: None,
        //         expression_l,
        //     })),

        //     MethodCallType::CSend => Box::new(Node::CSend(CSend {
        //         recv: receiver,
        //         method_name,
        //         args: vec![],
        //         dot_l,
        //         selector_l: Some(selector_l),
        //         begin_l: None,
        //         end_l: None,
        //         operator_l: None,
        //         expression_l,
        //     })),
        // }
        todo!()
    }

    pub(crate) fn index(
        &self,
        recv: &'b Node<'b>,
        lbrack_t: &'b Token<'b>,
        indexes: &'b NodeList<'b>,
        rbrack_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let begin_l = self.loc(lbrack_t);
        // let end_l = self.loc(rbrack_t);
        // let expression_l = recv.expression().join(&end_l);

        // self.rewrite_hash_args_to_kwargs(&mut indexes);

        // Box::new(Node::Index(Index {
        //     recv,
        //     indexes,
        //     begin_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn index_asgn(
        &self,
        recv: &'b Node<'b>,
        lbrack_t: &'b Token<'b>,
        indexes: &'b NodeList<'b>,
        rbrack_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let begin_l = self.loc(lbrack_t);
        // let end_l = self.loc(rbrack_t);
        // let expression_l = recv.expression().join(&end_l);

        // Box::new(Node::IndexAsgn(IndexAsgn {
        //     recv,
        //     indexes,
        //     value: None,
        //     begin_l,
        //     end_l,
        //     operator_l: None,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn binary_op(
        &self,
        receiver: &'b Node<'b>,
        operator_t: &'b Token<'b>,
        arg: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // self.value_expr(&receiver)?;
        // self.value_expr(&arg)?;

        // let selector_l = Some(self.loc(operator_t));
        // let expression_l = join_exprs(&receiver, &arg);

        // Ok(Box::new(Node::Send(Send {
        //     recv: Some(receiver),
        //     method_name: value(operator_t),
        //     args: vec![*arg],
        //     dot_l: None,
        //     selector_l,
        //     begin_l: None,
        //     end_l: None,
        //     operator_l: None,
        //     expression_l,
        // })))
        todo!()
    }

    pub(crate) fn match_op(
        &self,
        receiver: &'b Node<'b>,
        match_t: &'b Token<'b>,
        arg: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // self.value_expr(&receiver)?;
        // self.value_expr(&arg)?;

        // let selector_l = self.loc(match_t);
        // let expression_l = join_exprs(&receiver, &arg);

        // let result = match self.static_regexp_captures(&receiver) {
        //     Some(captures) => {
        //         for capture in captures {
        //             self.static_env
        //                 .declare(self.blob.push_str(&capture), self.blob);
        //         }

        //         Node::MatchWithLvasgn(MatchWithLvasgn {
        //             re: receiver,
        //             value: arg,
        //             operator_l: selector_l,
        //             expression_l,
        //         })
        //     }
        //     None => Node::Send(Send {
        //         recv: Some(receiver),
        //         method_name: String::from("=~"),
        //         args: vec![*arg],
        //         dot_l: None,
        //         selector_l: Some(selector_l),
        //         begin_l: None,
        //         end_l: None,
        //         operator_l: None,
        //         expression_l,
        //     }),
        // };

        // Ok(Box::new(result))
        todo!()
    }

    pub(crate) fn unary_op(
        &self,
        op_t: &'b Token<'b>,
        receiver: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // self.value_expr(&receiver)?;

        // let selector_l = self.loc(op_t);
        // let expression_l = receiver.expression().join(&selector_l);

        // let op = value(op_t);
        // let method_name = if op == "+" || op == "-" { op + "@" } else { op };
        // Ok(Box::new(Node::Send(Send {
        //     recv: Some(receiver),
        //     method_name,
        //     args: vec![],
        //     dot_l: None,
        //     selector_l: Some(selector_l),
        //     begin_l: None,
        //     end_l: None,
        //     operator_l: None,
        //     expression_l,
        // })))
        todo!()
    }

    pub(crate) fn not_op(
        &self,
        not_t: &'b Token<'b>,
        begin_t: Option<&'b Token<'b>>,
        receiver: Option<&'b Node<'b>>,
        end_t: Option<&'b Token<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        // if let Some(receiver) = receiver {
        //     let receiver = receiver;
        //     self.value_expr(&receiver)?;

        //     let begin_l = self.loc(not_t);
        //     let end_l = self
        //         .maybe_loc(end_t)
        //         .unwrap_or_else(|| *receiver.expression());

        //     let expression_l = begin_l.join(&end_l);

        //     let selector_l = self.loc(not_t);
        //     let begin_l = self.maybe_loc(begin_t);
        //     let end_l = self.maybe_loc(end_t);

        //     Ok(Box::new(Node::Send(Send {
        //         recv: Some(Self::check_condition(receiver)),
        //         method_name: String::from("!"),
        //         args: vec![],
        //         dot_l: None,
        //         selector_l: Some(selector_l),
        //         begin_l,
        //         end_l,
        //         operator_l: None,
        //         expression_l,
        //     })))
        // } else {
        //     let CollectionMap {
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     } = self.collection_map(
        //         begin_t.as_ref().map(|t| t.loc),
        //         &[],
        //         end_t.as_ref().map(|t| t.loc),
        //     );

        //     let nil_node = Box::new(Node::Begin(Begin {
        //         statements: vec![],
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }));

        //     let selector_l = self.loc(not_t);
        //     let expression_l = nil_node.expression().join(&selector_l);
        //     Ok(Box::new(Node::Send(Send {
        //         recv: Some(nil_node),
        //         method_name: String::from("!"),
        //         args: vec![],
        //         dot_l: None,
        //         selector_l: Some(selector_l),
        //         begin_l: None,
        //         end_l: None,
        //         operator_l: None,
        //         expression_l,
        //     })))
        // }
        todo!()
    }

    //
    // Control flow
    //

    // Logical operations: and, or

    pub(crate) fn logical_op(
        &self,
        type_: LogicalOp,
        lhs: &'b Node<'b>,
        op_t: &'b Token<'b>,
        rhs: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // self.value_expr(&lhs)?;

        // let operator_l = self.loc(op_t);
        // let expression_l = join_exprs(&lhs, &rhs);
        // let lhs: &'b Node<'b> = lhs;
        // let rhs: &'b Node<'b> = rhs;

        // let result = match type_ {
        //     LogicalOp::And => Node::And(And {
        //         lhs,
        //         rhs,
        //         operator_l,
        //         expression_l,
        //     }),
        //     LogicalOp::Or => Node::Or(Or {
        //         lhs,
        //         rhs,
        //         operator_l,
        //         expression_l,
        //     }),
        // };
        // Ok(Box::new(result))
        todo!()
    }

    // Conditionals

    pub(crate) fn condition(
        &self,
        cond_t: &Token,
        cond: &'b Node<'b>,
        then_t: &'b Token<'b>,
        if_true: Option<&'b Node<'b>>,
        else_t: Option<&'b Token<'b>>,
        if_false: Option<&'b Node<'b>>,
        end_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        let end_l = self
            .maybe_loc(end_t)
            .or_else(|| maybe_boxed_node_expr(if_false.as_deref()))
            .or_else(|| self.maybe_loc(else_t))
            .or_else(|| maybe_boxed_node_expr(if_true.as_deref()))
            .unwrap_or_else(|| self.loc(then_t));

        let expression_l = self.loc(cond_t).join(end_l);
        let keyword_l = self.loc(cond_t);
        let begin_l = self.loc(then_t);
        let else_l = self.maybe_loc(else_t);
        let end_l = self.maybe_loc(end_t);

        If::new_in(self.blob, |if_| {
            if_.cond = Self::check_condition(cond);
            if_.if_true = if_true;
            if_.if_false = if_false;
            if_.keyword_l = keyword_l;
            if_.begin_l = begin_l;
            if_.else_l = else_l;
            if_.end_l = end_l;
            if_.expression_l = expression_l;
        })
    }

    pub(crate) fn condition_mod(
        &self,
        if_true: Option<&'b Node<'b>>,
        if_false: Option<&'b Node<'b>>,
        cond_t: &'b Token<'b>,
        cond: &'b Node<'b>,
    ) -> &'b Node<'b> {
        // let pre = match (if_true.as_ref(), if_false.as_ref()) {
        //     (None, None) => unreachable!("at least one of if_true/if_false is required"),
        //     (None, Some(if_false)) => if_false,
        //     (Some(if_true), None) => if_true,
        //     (Some(_), Some(_)) => unreachable!("only one of if_true/if_false is required"),
        // };

        // let expression_l = pre.expression().join(cond.expression());
        // let keyword_l = self.loc(cond_t);

        // Box::new(Node::IfMod(IfMod {
        //     cond: Self::check_condition(cond),
        //     if_true,
        //     if_false,
        //     keyword_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn ternary(
        &self,
        cond: &'b Node<'b>,
        question_t: &'b Token<'b>,
        if_true: &'b Node<'b>,
        colon_t: &'b Token<'b>,
        if_false: &'b Node<'b>,
    ) -> &'b Node<'b> {
        // let expression_l = join_exprs(&cond, &if_false);
        // let question_l = self.loc(question_t);
        // let colon_l = self.loc(colon_t);

        // Box::new(Node::IfTernary(IfTernary {
        //     cond,
        //     if_true,
        //     if_false,
        //     question_l,
        //     colon_l,
        //     expression_l,
        // }))
        todo!()
    }

    // Case matching

    pub(crate) fn when(
        &self,
        when_t: &'b Token<'b>,
        patterns: &'b NodeList<'b>,
        then_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
    ) -> &'b Node<'b> {
        // let begin_l = self.loc(then_t);

        // let expr_end_l = maybe_boxed_node_expr(body.as_deref())
        //     .or_else(|| maybe_node_expr(patterns.last()))
        //     .unwrap_or_else(|| self.loc(when_t));
        // let when_l = self.loc(when_t);
        // let expression_l = when_l.join(&expr_end_l);

        // Box::new(Node::When(When {
        //     patterns,
        //     body,
        //     keyword_l: when_l,
        //     begin_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn case(
        &self,
        case_t: &'b Token<'b>,
        expr: Option<&'b Node<'b>>,
        when_bodies: &'b NodeList<'b>,
        else_t: Option<&'b Token<'b>>,
        else_body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let keyword_l = self.loc(case_t);
        // let else_l = self.maybe_loc(else_t);
        // let end_l = self.loc(end_t);
        // let expression_l = keyword_l.join(&end_l);

        // Box::new(Node::Case(Case {
        //     expr,
        //     when_bodies,
        //     else_body,
        //     keyword_l,
        //     else_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    // Loops

    pub(crate) fn loop_(
        &self,
        loop_type: LoopType,
        keyword_t: &'b Token<'b>,
        cond: &'b Node<'b>,
        do_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let keyword_l = self.loc(keyword_t);
        // let begin_l = self.loc(do_t);
        // let end_l = self.loc(end_t);
        // let expression_l = self.loc(keyword_t).join(&end_l);

        // let cond = Self::check_condition(cond);

        // match loop_type {
        //     LoopType::While => Box::new(Node::While(While {
        //         cond,
        //         body,
        //         keyword_l,
        //         begin_l: Some(begin_l),
        //         end_l: Some(end_l),
        //         expression_l,
        //     })),
        //     LoopType::Until => Box::new(Node::Until(Until {
        //         cond,
        //         body,
        //         keyword_l,
        //         begin_l: Some(begin_l),
        //         end_l: Some(end_l),
        //         expression_l,
        //     })),
        // }
        todo!()
    }

    pub(crate) fn loop_mod(
        &self,
        loop_type: LoopType,
        body: &'b Node<'b>,
        keyword_t: &'b Token<'b>,
        cond: &'b Node<'b>,
    ) -> &'b Node<'b> {
        // let expression_l = body.expression().join(cond.expression());
        // let keyword_l = self.loc(keyword_t);

        // let cond = Self::check_condition(cond);

        // match (loop_type, &*body) {
        //     (LoopType::While, Node::KwBegin(_)) => Box::new(Node::WhilePost(WhilePost {
        //         cond,
        //         body,
        //         keyword_l,
        //         expression_l,
        //     })),
        //     (LoopType::While, _) => Box::new(Node::While(While {
        //         cond,
        //         body: Some(body),
        //         keyword_l,
        //         begin_l: None,
        //         end_l: None,
        //         expression_l,
        //     })),
        //     (LoopType::Until, Node::KwBegin(_)) => Box::new(Node::UntilPost(UntilPost {
        //         cond,
        //         body,
        //         keyword_l,
        //         expression_l,
        //     })),
        //     (LoopType::Until, _) => Box::new(Node::Until(Until {
        //         cond,
        //         body: Some(body),
        //         keyword_l,
        //         begin_l: None,
        //         end_l: None,
        //         expression_l,
        //     })),
        // }
        todo!()
    }

    pub(crate) fn for_(
        &self,
        for_t: &'b Token<'b>,
        iterator: &'b Node<'b>,
        in_t: &'b Token<'b>,
        iteratee: &'b Node<'b>,
        do_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let keyword_l = self.loc(for_t);
        // let operator_l = self.loc(in_t);
        // let begin_l = self.loc(do_t);
        // let end_l = self.loc(end_t);
        // let expression_l = keyword_l.join(&end_l);

        // Box::new(Node::For(For {
        //     iterator,
        //     iteratee,
        //     body,
        //     keyword_l,
        //     operator_l,
        //     begin_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    // Keywords

    pub(crate) fn keyword_cmd(
        &self,
        type_: KeywordCmd,
        keyword_t: &'b Token<'b>,
        lparen_t: Option<&'b Token<'b>>,
        args: &'b NodeList<'b>,
        rparen_t: Option<&'b Token<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        // let keyword_l = self.loc(keyword_t);

        // if type_ == KeywordCmd::Yield && !args.is_empty() {
        //     if let Some(Node::BlockPass(_)) = args.last() {
        //         self.error(DiagnosticMessage::BlockGivenToYield {}, keyword_l);
        //         return Err(());
        //     }
        // }

        // match type_ {
        //     KeywordCmd::Yield | KeywordCmd::Super => {
        //         self.rewrite_hash_args_to_kwargs(&mut args);
        //     }
        //     _ => {}
        // }

        // let begin_l = self.maybe_loc(lparen_t);
        // let end_l = self.maybe_loc(rparen_t);

        // let expr_end_l = end_l
        //     .or_else(|| maybe_node_expr(args.last()))
        //     .unwrap_or(keyword_l);

        // let expression_l = keyword_l.join(&expr_end_l);

        // let result = match type_ {
        //     KeywordCmd::Break => Node::Break(Break {
        //         args,
        //         keyword_l,
        //         expression_l,
        //     }),
        //     KeywordCmd::Defined => Node::Defined(Defined {
        //         value: Box::new(args.into_iter().next().unwrap()),
        //         keyword_l,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }),
        //     KeywordCmd::Next => Node::Next(Next {
        //         args,
        //         keyword_l,
        //         expression_l,
        //     }),
        //     KeywordCmd::Redo => Node::Redo(Redo { expression_l }),
        //     KeywordCmd::Retry => Node::Retry(Retry { expression_l }),
        //     KeywordCmd::Return => Node::Return(Return {
        //         args,
        //         keyword_l,
        //         expression_l,
        //     }),
        //     KeywordCmd::Super => Node::Super(Super {
        //         args,
        //         keyword_l,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }),
        //     KeywordCmd::Yield => Node::Yield(Yield {
        //         args,
        //         keyword_l,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }),
        //     KeywordCmd::Zsuper => Node::ZSuper(ZSuper { expression_l }),
        // };

        // Ok(Box::new(result))
        todo!()
    }

    // BEGIN, END

    pub(crate) fn preexe(
        &self,
        preexe_t: &'b Token<'b>,
        lbrace_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
        rbrace_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let keyword_l = self.loc(preexe_t);
        // let begin_l = self.loc(lbrace_t);
        // let end_l = self.loc(rbrace_t);
        // let expression_l = keyword_l.join(&end_l);

        // Box::new(Node::Preexe(Preexe {
        //     body,
        //     keyword_l,
        //     begin_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }
    pub(crate) fn postexe(
        &self,
        postexe_t: &'b Token<'b>,
        lbrace_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
        rbrace_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let keyword_l = self.loc(postexe_t);
        // let begin_l = self.loc(lbrace_t);
        // let end_l = self.loc(rbrace_t);
        // let expression_l = keyword_l.join(&end_l);

        // Box::new(Node::Postexe(Postexe {
        //     body,
        //     keyword_l,
        //     begin_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    // Exception handling

    pub(crate) fn rescue_body(
        &self,
        rescue_t: &'b Token<'b>,
        exc_list: Option<&'b Node<'b>>,
        assoc_t: Option<&'b Token<'b>>,
        exc_var: Option<&'b Node<'b>>,
        then_t: Option<&'b Token<'b>>,
        body: Option<&'b Node<'b>>,
    ) -> &'b Node<'b> {
        // let end_l = maybe_boxed_node_expr(body.as_deref())
        //     .or_else(|| self.maybe_loc(then_t))
        //     .or_else(|| maybe_boxed_node_expr(exc_var.as_deref()))
        //     .or_else(|| maybe_boxed_node_expr(exc_list.as_deref()))
        //     .unwrap_or_else(|| self.loc(rescue_t));

        // let expression_l = self.loc(rescue_t).join(&end_l);
        // let keyword_l = self.loc(rescue_t);
        // let assoc_l = self.maybe_loc(assoc_t);
        // let begin_l = self.maybe_loc(then_t);

        // Box::new(Node::RescueBody(RescueBody {
        //     exc_list,
        //     exc_var,
        //     body,
        //     keyword_l,
        //     assoc_l,
        //     begin_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn begin_body(
        &self,
        compound_stmt: Option<&'b Node<'b>>,
        rescue_bodies: &'b NodeList<'b>,
        else_: Option<(&'b Token<'b>, Option<&'b Node<'b>>)>,
        ensure: Option<(&'b Token<'b>, Option<&'b Node<'b>>)>,
    ) -> Option<&'b Node<'b>> {
        // let mut result: Option<&'b Node<'b>>;

        // if !rescue_bodies.is_empty() {
        //     if let Some((else_t, else_)) = else_ {
        //         let begin_l = maybe_boxed_node_expr(compound_stmt.as_deref())
        //             .or_else(|| maybe_node_expr(rescue_bodies.first()))
        //             .unwrap_or_else(|| unreachable!("can't compute begin_l"));

        //         let end_l =
        //             maybe_boxed_node_expr(else_.as_deref()).unwrap_or_else(|| self.loc(else_t));

        //         let expression_l = begin_l.join(&end_l);
        //         let else_l = self.loc(else_t);

        //         result = Some(Box::new(Node::Rescue(Rescue {
        //             body: compound_stmt,
        //             rescue_bodies,
        //             else_,
        //             else_l: Some(else_l),
        //             expression_l,
        //         })))
        //     } else {
        //         let begin_l = maybe_boxed_node_expr(compound_stmt.as_deref())
        //             .or_else(|| maybe_node_expr(rescue_bodies.first()))
        //             .unwrap_or_else(|| unreachable!("can't compute begin_l"));

        //         let end_l = maybe_node_expr(rescue_bodies.last())
        //             .unwrap_or_else(|| unreachable!("can't compute end_l"));

        //         let expression_l = begin_l.join(&end_l);
        //         let else_l = self.maybe_loc(None);

        //         result = Some(Box::new(Node::Rescue(Rescue {
        //             body: compound_stmt,
        //             rescue_bodies,
        //             else_: None,
        //             else_l,
        //             expression_l,
        //         })))
        //     }
        // } else if let Some((else_t, else_)) = else_ {
        //     let mut statements = vec![];

        //     if let Some(compound_stmt) = compound_stmt {
        //         match *compound_stmt {
        //             Node::Begin(Begin {
        //                 statements: stmts, ..
        //             }) => statements = stmts,
        //             other => statements.push(other),
        //         }
        //     }

        //     let parts = if else_.is_some() {
        //         vec![*else_.unwrap()]
        //     } else {
        //         vec![]
        //     };
        //     let CollectionMap {
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     } = self.collection_map(Some(else_t.loc), &parts, None);

        //     statements.push(Node::Begin(Begin {
        //         statements: parts,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }));

        //     let CollectionMap {
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     } = self.collection_map(None, &statements, None);

        //     result = Some(Box::new(Node::Begin(Begin {
        //         statements,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     })))
        // } else {
        //     result = compound_stmt;
        // }

        // if let Some((ensure_t, ensure)) = ensure {
        //     let ensure_body = ensure;
        //     let keyword_l = self.loc(ensure_t);

        //     let begin_l =
        //         maybe_boxed_node_expr(result.as_deref()).unwrap_or_else(|| self.loc(ensure_t));

        //     let end_l =
        //         maybe_node_expr(ensure_body.as_deref()).unwrap_or_else(|| self.loc(ensure_t));

        //     let expression_l = begin_l.join(&end_l);

        //     result = Some(Box::new(Node::Ensure(Ensure {
        //         body: result,
        //         ensure: ensure_body,
        //         keyword_l,
        //         expression_l,
        //     })))
        // }

        // result
        todo!()
    }

    //
    // Expression grouping
    //

    pub(crate) fn compstmt(&self, statements: &'b NodeList<'b>) -> Option<&'b Node<'b>> {
        if statements.is_empty() {
            None
        } else if statements.len() == 1 {
            statements.iter().next()
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(None, &statements, None);

            Some(Begin::new_in(self.blob, |begin| {
                begin.statements = statements;
                begin.begin_l = begin_l;
                begin.end_l = end_l;
                begin.expression_l = expression_l;
            }))
        }
    }

    pub(crate) fn begin(
        &self,
        begin_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        let new_begin_l = self.loc(begin_t);
        let new_end_l = self.loc(end_t);
        let new_expression_l = new_begin_l.join(new_end_l);

        let new_begin_l = Some(new_begin_l);
        let new_end_l = Some(new_end_l);

        if let Some(body) = body {
            match body {
                Node::Mlhs(Mlhs {
                    items,
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) => {
                    // Synthesized (begin) from compstmt "a; b" or (mlhs)
                    // from multi_lhs "(a, b) = *foo".
                    Mlhs::new_in(self.blob, |mlhs| {
                        mlhs.items = items;
                        mlhs.begin_l = new_begin_l;
                        mlhs.end_l = new_end_l;
                        mlhs.expression_l = new_expression_l;
                    })
                }
                Node::Begin(Begin {
                    begin_l,
                    end_l,
                    expression_l,
                    statements,
                    ..
                }) if begin_l.is_none() && end_l.is_none() => Begin::new_in(self.blob, |begin| {
                    begin.statements = statements;
                    begin.begin_l = new_begin_l;
                    begin.end_l = new_end_l;
                    begin.expression_l = new_expression_l;
                }),
                _ => Begin::new_in(self.blob, |begin| {
                    begin.statements.push(body);
                    begin.begin_l = new_begin_l;
                    begin.end_l = new_end_l;
                    begin.expression_l = new_expression_l;
                }),
            }
        } else {
            // A nil expression: `()'.
            Begin::new_in(self.blob, |begin| {
                begin.begin_l = new_begin_l;
                begin.end_l = new_end_l;
                begin.expression_l = new_expression_l;
            })
        }
    }

    pub(crate) fn begin_keyword(
        &self,
        begin_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let begin_l = self.loc(begin_t);
        // let end_l = self.loc(end_t);
        // let expression_l = begin_l.join(&end_l);

        // let begin_l = Some(begin_l);
        // let end_l = Some(end_l);

        // if let Some(body) = body {
        //     let body = *body;
        //     match body {
        //         Node::Begin(Begin { statements, .. }) => {
        //             // Synthesized (begin) from compstmt "a; b".
        //             Box::new(Node::KwBegin(KwBegin {
        //                 statements,
        //                 begin_l,
        //                 end_l,
        //                 expression_l,
        //             }))
        //         }
        //         other => Box::new(Node::KwBegin(KwBegin {
        //             statements: vec![other],
        //             begin_l,
        //             end_l,
        //             expression_l,
        //         })),
        //     }
        // } else {
        //     // A nil expression: `begin end'.
        //     Box::new(Node::KwBegin(KwBegin {
        //         statements: vec![],
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }))
        // }
        todo!()
    }

    //
    // Pattern matching
    //

    pub(crate) fn case_match(
        &self,
        case_t: &'b Token<'b>,
        expr: &'b Node<'b>,
        in_bodies: &'b NodeList<'b>,
        else_t: Option<&'b Token<'b>>,
        else_body: Option<&'b Node<'b>>,
        end_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let else_body = match (else_t.as_ref(), else_body.as_ref()) {
        //     (Some(else_t), None) => Some(Box::new(Node::EmptyElse(EmptyElse {
        //         expression_l: self.loc(else_t),
        //     }))),
        //     _ => else_body,
        // };

        // let keyword_l = self.loc(case_t);
        // let else_l = self.maybe_loc(else_t);
        // let end_l = self.loc(end_t);
        // let expression_l = self.loc(case_t).join(&end_l);

        // Box::new(Node::CaseMatch(CaseMatch {
        //     expr,
        //     in_bodies,
        //     else_body,
        //     keyword_l,
        //     else_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn match_pattern(
        &self,
        value: &'b Node<'b>,
        assoc_t: &'b Token<'b>,
        pattern: &'b Node<'b>,
    ) -> &'b Node<'b> {
        // let operator_l = self.loc(assoc_t);
        // let expression_l = join_exprs(&value, &pattern);

        // Box::new(Node::MatchPattern(MatchPattern {
        //     value,
        //     pattern,
        //     operator_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn match_pattern_p(
        &self,
        value: &'b Node<'b>,
        in_t: &'b Token<'b>,
        pattern: &'b Node<'b>,
    ) -> &'b Node<'b> {
        // let operator_l = self.loc(in_t);
        // let expression_l = join_exprs(&value, &pattern);

        // Box::new(Node::MatchPatternP(MatchPatternP {
        //     value,
        //     pattern,
        //     operator_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn in_pattern(
        &self,
        in_t: &'b Token<'b>,
        pattern: &'b Node<'b>,
        guard: Option<&'b Node<'b>>,
        then_t: &'b Token<'b>,
        body: Option<&'b Node<'b>>,
    ) -> &'b Node<'b> {
        // let keyword_l = self.loc(in_t);
        // let begin_l = self.loc(then_t);

        // let expression_l = maybe_boxed_node_expr(body.as_deref())
        //     .or_else(|| maybe_boxed_node_expr(guard.as_deref()))
        //     .unwrap_or_else(|| *pattern.expression())
        //     .join(&keyword_l);

        // Box::new(Node::InPattern(InPattern {
        //     pattern,
        //     guard,
        //     body,
        //     keyword_l,
        //     begin_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn if_guard(&self, if_t: &'b Token<'b>, cond: &'b Node<'b>) -> &'b Node<'b> {
        // let keyword_l = self.loc(if_t);
        // let expression_l = keyword_l.join(cond.expression());

        // Box::new(Node::IfGuard(IfGuard {
        //     cond,
        //     keyword_l,
        //     expression_l,
        // }))
        todo!()
    }
    pub(crate) fn unless_guard(&self, unless_t: &'b Token<'b>, cond: &'b Node<'b>) -> &'b Node<'b> {
        let keyword_l = self.loc(unless_t);
        let expression_l = keyword_l.join(cond.expression());

        // Box::new(Node::UnlessGuard(UnlessGuard {
        //     cond,
        //     keyword_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn match_var(&self, name_t: &'b Token<'b>) -> Result<&'b Node<'b>, ()> {
        // let name_l = self.loc(name_t);
        // let expression_l = name_l;

        // self.check_lvar_name(name_t.as_whole_str(), name_l)?;
        // self.check_duplicate_pattern_variable(name_t.as_whole_str(), name_l)?;
        // self.static_env.declare(name_t.as_whole_str(), self.blob);

        // Ok(Box::new(Node::MatchVar(MatchVar {
        //     name: name_t.to_string().unwrap(),
        //     name_l,
        //     expression_l,
        // })))
        todo!()
    }

    pub(crate) fn match_hash_var(&self, name_t: &'b Token<'b>) -> Result<&'b Node<'b>, ()> {
        // let expression_l = self.loc(name_t);
        // let name_l = expression_l.adjust_end(-1);

        // let name = value(name_t);

        // self.check_lvar_name(name_t.as_whole_str(), name_l)?;
        // self.check_duplicate_pattern_variable(name_t.as_whole_str(), name_l)?;
        // self.static_env.declare(name_t.as_whole_str(), self.blob);

        // Ok(Box::new(Node::MatchVar(MatchVar {
        //     name,
        //     name_l,
        //     expression_l,
        // })))
        todo!()
    }
    pub(crate) fn match_hash_var_from_str(
        &self,
        begin_t: &'b Token<'b>,
        strings: &'b NodeList<'b>,
        end_t: &'b Token<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // if strings.len() != 1 {
        //     self.error(
        //         DiagnosticMessage::SymbolLiteralWithInterpolation {},
        //         self.loc(begin_t).join(&self.loc(end_t)),
        //     );
        //     return Err(());
        // }

        // let string = strings.remove(0);
        // let result = match string {
        //     Node::Str(Str {
        //         value,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }) => {
        //         let name = value.to_string_lossy();
        //         let mut name_l = expression_l;

        //         let name_s = self.blob.push_str(&name);

        //         self.check_lvar_name(name_s, name_l)?;
        //         self.check_duplicate_pattern_variable(name_s, name_l)?;

        //         self.static_env.declare(name_s, self.blob);

        //         if let Some(begin_l) = begin_l.as_ref() {
        //             let begin_d: i32 = begin_l
        //                 .size()
        //                 .try_into()
        //                 .expect("failed to convert usize loc into i32, is it too big?");
        //             name_l = name_l.adjust_begin(begin_d)
        //         }

        //         if let Some(end_l) = end_l.as_ref() {
        //             let end_d: i32 = end_l
        //                 .size()
        //                 .try_into()
        //                 .expect("failed to convert usize loc into i32, is it too big?");
        //             name_l = name_l.adjust_end(-end_d)
        //         }

        //         let expression_l = self.loc(begin_t).join(&expression_l).join(&self.loc(end_t));
        //         Box::new(Node::MatchVar(MatchVar {
        //             name,
        //             name_l,
        //             expression_l,
        //         }))
        //     }
        //     Node::Begin(Begin { statements, .. }) => {
        //         self.match_hash_var_from_str(begin_t, statements, end_t)?
        //     }
        //     _ => {
        //         self.error(
        //             DiagnosticMessage::SymbolLiteralWithInterpolation {},
        //             self.loc(begin_t).join(&self.loc(end_t)),
        //         );
        //         return Err(());
        //     }
        // };

        // Ok(result)
        todo!()
    }

    pub(crate) fn match_rest(
        &self,
        star_t: &'b Token<'b>,
        name_t: Option<&'b Token<'b>>,
    ) -> Result<&'b Node<'b>, ()> {
        // let name = if let Some(name_t) = name_t {
        //     Some(self.match_var(name_t)?)
        // } else {
        //     None
        // };

        // let operator_l = self.loc(star_t);
        // let expression_l = operator_l.maybe_join(&maybe_boxed_node_expr(name.as_deref()));

        // Ok(Box::new(Node::MatchRest(MatchRest {
        //     name,
        //     operator_l,
        //     expression_l,
        // })))
        todo!()
    }

    pub(crate) fn hash_pattern(
        &self,
        lbrace_t: Option<&'b Token<'b>>,
        kwargs: &'b NodeList<'b>,
        rbrace_t: Option<&'b Token<'b>>,
    ) -> &'b Node<'b> {
        // let CollectionMap {
        //     begin_l,
        //     end_l,
        //     expression_l,
        // } = self.collection_map(
        //     lbrace_t.as_ref().map(|t| t.loc),
        //     &kwargs,
        //     rbrace_t.as_ref().map(|t| t.loc),
        // );

        // Box::new(Node::HashPattern(HashPattern {
        //     elements: kwargs,
        //     begin_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn array_pattern(
        &self,
        lbrack_l: Option<Loc>,
        elements: &'b NodeList<'b>,
        trailing_comma: Option<&'b Token<'b>>,
        rbrack_l: Option<Loc>,
    ) -> &'b Node<'b> {
        // let CollectionMap {
        //     begin_l,
        //     end_l,
        //     expression_l,
        // } = self.collection_map(lbrack_l, &elements, rbrack_l);

        // let expression_l = expression_l.maybe_join(&self.maybe_loc(trailing_comma));

        // if elements.is_empty() {
        //     return Box::new(Node::ArrayPattern(ArrayPattern {
        //         elements: vec![],
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }));
        // }

        // if trailing_comma.is_some() {
        //     Box::new(Node::ArrayPatternWithTail(ArrayPatternWithTail {
        //         elements,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }))
        // } else {
        //     Box::new(Node::ArrayPattern(ArrayPattern {
        //         elements,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }))
        // }
        todo!()
    }

    pub(crate) fn find_pattern(
        &self,
        lbrack_l: Option<Loc>,
        elements: &'b NodeList<'b>,
        rbrack_l: Option<Loc>,
    ) -> &'b Node<'b> {
        // let CollectionMap {
        //     begin_l,
        //     end_l,
        //     expression_l,
        // } = self.collection_map(lbrack_l, &elements, rbrack_l);

        // Box::new(Node::FindPattern(FindPattern {
        //     elements,
        //     begin_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn const_pattern(
        &self,
        const_: &'b Node<'b>,
        ldelim_t: &'b Token<'b>,
        pattern: &'b Node<'b>,
        rdelim_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let begin_l = self.loc(ldelim_t);
        // let end_l = self.loc(rdelim_t);
        // let expression_l = const_.expression().join(&self.loc(rdelim_t));

        // Box::new(Node::ConstPattern(ConstPattern {
        //     const_,
        //     pattern,
        //     begin_l,
        //     end_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn pin(&self, pin_t: &'b Token<'b>, var: &'b Node<'b>) -> &'b Node<'b> {
        // let operator_l = self.loc(pin_t);
        // let expression_l = var.expression().join(&operator_l);

        // Box::new(Node::Pin(Pin {
        //     var,
        //     selector_l: operator_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn match_alt(
        &self,
        lhs: &'b Node<'b>,
        pipe_t: &'b Token<'b>,
        rhs: &'b Node<'b>,
    ) -> &'b Node<'b> {
        // let operator_l = self.loc(pipe_t);
        // let expression_l = join_exprs(&lhs, &rhs);

        // Box::new(Node::MatchAlt(MatchAlt {
        //     lhs,
        //     rhs,
        //     operator_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn match_as(
        &self,
        value: &'b Node<'b>,
        assoc_t: &'b Token<'b>,
        as_: &'b Node<'b>,
    ) -> &'b Node<'b> {
        // let operator_l = self.loc(assoc_t);
        // let expression_l = join_exprs(&value, &as_);

        // Box::new(Node::MatchAs(MatchAs {
        //     value,
        //     as_,
        //     operator_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn match_nil_pattern(
        &self,
        dstar_t: &'b Token<'b>,
        nil_t: &'b Token<'b>,
    ) -> &'b Node<'b> {
        // let operator_l = self.loc(dstar_t);
        // let name_l = self.loc(nil_t);
        // let expression_l = operator_l.join(&name_l);

        // Box::new(Node::MatchNilPattern(MatchNilPattern {
        //     operator_l,
        //     name_l,
        //     expression_l,
        // }))
        todo!()
    }

    pub(crate) fn match_pair(
        &self,
        p_kw_label: PKwLabel<'b>,
        value: &'b Node<'b>,
    ) -> Result<&'b Node<'b>, ()> {
        // let result = match p_kw_label {
        //     PKwLabel::PlainLabel(label_t) => {
        //         self.check_duplicate_pattern_key(label_t.as_whole_str(), self.loc(label_t))?;
        //         self.pair_keyword(label_t, value)
        //     }
        //     PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
        //         let label_loc = self.loc(begin_t).join(&self.loc(end_t));

        //         match Self::static_string(&parts) {
        //             Some(var_name) => {
        //                 let var_name = self.blob.push_str(&var_name);
        //                 self.check_duplicate_pattern_key(var_name, label_loc)?
        //             }
        //             _ => {
        //                 self.error(
        //                     DiagnosticMessage::SymbolLiteralWithInterpolation {},
        //                     label_loc,
        //                 );
        //                 return Err(());
        //             }
        //         }

        //         self.pair_quoted(begin_t, parts, end_t, value)
        //     }
        // };
        // Ok(result)
        todo!()
    }

    pub(crate) fn match_label(&self, p_kw_label: PKwLabel<'b>) -> Result<&'b Node<'b>, ()> {
        // match p_kw_label {
        //     PKwLabel::PlainLabel(label_t) => self.match_hash_var(label_t),
        //     PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
        //         self.match_hash_var_from_str(begin_t, parts, end_t)
        //     }
        // }
        todo!()
    }

    //
    // Verification
    //

    pub(crate) fn check_condition(cond: &'b Node<'b>) -> &'b Node<'b> {
        // let cond = cond;

        // match *cond {
        //     Node::Begin(Begin {
        //         statements,
        //         begin_l,
        //         end_l,
        //         expression_l,
        //     }) => {
        //         if statements.len() == 1 {
        //             let stmt = statements.into_iter().next().unwrap();
        //             let stmt = *Self::check_condition(Box::new(stmt));
        //             Box::new(Node::Begin(Begin {
        //                 statements: vec![stmt],
        //                 begin_l,
        //                 end_l,
        //                 expression_l,
        //             }))
        //         } else {
        //             Box::new(Node::Begin(Begin {
        //                 statements,
        //                 begin_l,
        //                 end_l,
        //                 expression_l,
        //             }))
        //         }
        //     }
        //     Node::And(And {
        //         lhs,
        //         rhs,
        //         operator_l,
        //         expression_l,
        //     }) => {
        //         let lhs = Self::check_condition(lhs);
        //         let rhs = Self::check_condition(rhs);
        //         Box::new(Node::And(And {
        //             lhs,
        //             rhs,
        //             operator_l,
        //             expression_l,
        //         }))
        //     }
        //     Node::Or(Or {
        //         lhs,
        //         rhs,
        //         operator_l,
        //         expression_l,
        //     }) => {
        //         let lhs = Self::check_condition(lhs);
        //         let rhs = Self::check_condition(rhs);
        //         Box::new(Node::Or(Or {
        //             lhs,
        //             rhs,
        //             operator_l,
        //             expression_l,
        //         }))
        //     }
        //     Node::Irange(Irange {
        //         left,
        //         right,
        //         operator_l,
        //         expression_l,
        //     }) => Box::new(Node::IFlipFlop(IFlipFlop {
        //         left: left.map(Self::check_condition),
        //         right: right.map(Self::check_condition),
        //         operator_l,
        //         expression_l,
        //     })),
        //     Node::Erange(Erange {
        //         left,
        //         right,
        //         operator_l,
        //         expression_l,
        //     }) => Box::new(Node::EFlipFlop(EFlipFlop {
        //         left: left.map(Self::check_condition),
        //         right: right.map(Self::check_condition),
        //         operator_l,
        //         expression_l,
        //     })),
        //     regexp if matches!(regexp, Node::Regexp(_)) => {
        //         let expression_l = *regexp.expression();

        //         Box::new(Node::MatchCurrentLine(MatchCurrentLine {
        //             re: Box::new(regexp),
        //             expression_l,
        //         }))
        //     }
        //     other => Box::new(other),
        // }
        todo!()
    }

    pub(crate) fn check_duplicate_args<'a>(
        &self,
        args: &'a NodeList<'a>,
        map: &mut HashMap<String, &'a Node>,
    ) {
        // for arg in args {
        //     match arg {
        //         Node::Arg(_)
        //         | Node::Optarg(_)
        //         | Node::Restarg(_)
        //         | Node::Kwarg(_)
        //         | Node::Kwoptarg(_)
        //         | Node::Kwrestarg(_)
        //         | Node::Shadowarg(_)
        //         | Node::Blockarg(_) => {
        //             self.check_duplicate_arg(arg, map);
        //         }
        //         Node::Mlhs(Mlhs { items, .. }) => {
        //             self.check_duplicate_args(items, map);
        //         }
        //         Node::Procarg0(Procarg0 { args, .. }) => {
        //             self.check_duplicate_args(args, map);
        //         }
        //         Node::ForwardArg(_) | Node::Kwnilarg(_) => {
        //             // ignore
        //         }
        //         _ => {
        //             unreachable!("unsupported arg type {:?}", arg)
        //         }
        //     }
        // }
        todo!()
    }

    fn arg_name<'a>(&self, node: &'a Node) -> Option<&'a str> {
        // match node {
        //     Node::Arg(Arg { name, .. })
        //     | Node::Optarg(Optarg { name, .. })
        //     | Node::Kwarg(Kwarg { name, .. })
        //     | Node::Kwoptarg(Kwoptarg { name, .. })
        //     | Node::Shadowarg(Shadowarg { name, .. }) => Some(name.as_str()),

        //     Node::Restarg(Restarg { name, .. })
        //     | Node::Kwrestarg(Kwrestarg { name, .. })
        //     | Node::Blockarg(Blockarg { name, .. }) => name.as_ref().map(|s| s.as_str()),
        //     _ => {
        //         unreachable!("unsupported arg {:?}", node)
        //     }
        // }
        todo!()
    }

    fn arg_name_loc(&self, node: &Node) -> Loc {
        // match node {
        //     Node::Arg(Arg {
        //         expression_l: output_l,
        //         ..
        //     })
        //     | Node::Optarg(Optarg {
        //         name_l: output_l, ..
        //     })
        //     | Node::Kwarg(Kwarg {
        //         name_l: output_l, ..
        //     })
        //     | Node::Kwoptarg(Kwoptarg {
        //         name_l: output_l, ..
        //     })
        //     | Node::Shadowarg(Shadowarg {
        //         expression_l: output_l,
        //         ..
        //     }) => *output_l,
        //     Node::Blockarg(Blockarg {
        //         name_l,
        //         expression_l,
        //         ..
        //     })
        //     | Node::Restarg(Restarg {
        //         name_l,
        //         expression_l,
        //         ..
        //     })
        //     | Node::Kwrestarg(Kwrestarg {
        //         name_l,
        //         expression_l,
        //         ..
        //     }) => name_l.unwrap_or(*expression_l),
        //     _ => unreachable!("unsupported arg {:?}", node),
        // }
        todo!()
    }

    pub(crate) fn check_duplicate_arg<'a>(
        &self,
        this_arg: &'a Node<'b>,
        // map: &mut HashMap<String, &'a Node<'b>>,
    ) {
        // let this_name = match self.arg_name(this_arg) {
        //     Some(name) => name,
        //     None => return,
        // };

        // let that_arg = map.get(this_name);

        // match that_arg {
        //     None => {
        //         map.insert(this_name.to_string(), this_arg);
        //     }
        //     Some(that_arg) => {
        //         let that_name = match self.arg_name(that_arg) {
        //             Some(name) => name,
        //             None => return,
        //         };
        //         if self.arg_name_collides(this_name, that_name) {
        //             self.error(
        //                 DiagnosticMessage::DuplicatedArgumentName {},
        //                 self.arg_name_loc(this_arg),
        //             )
        //         }
        //     }
        // }
        todo!()
    }

    pub(crate) fn check_assignment_to_numparam(&self, name: &str, loc: Loc) -> Result<(), ()> {
        let assigning_to_numparam = self.context.is_in_dynamic_block()
            && matches!(
                name,
                "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9"
            )
            && self.max_numparam_stack.has_numparams();

        if assigning_to_numparam {
            self.error(
                DiagnosticMessage::CantAssignToNumparam {
                    numparam: self.blob.push_str(name),
                },
                loc,
            );
            return Err(());
        }
        Ok(())
    }

    pub(crate) fn validate_no_forward_arg_after_restarg(&self, args: &'b NodeList<'b>) {
        let mut restarg = None;
        let mut forward_arg = None;
        for arg in args.iter() {
            match arg {
                Node::Restarg(_) => restarg = Some(arg),
                Node::ForwardArg(_) => forward_arg = Some(arg),
                _ => {}
            }
        }

        if restarg.is_none() {
            return;
        }

        if let Some(forward_arg) = forward_arg {
            self.error(
                DiagnosticMessage::ForwardArgAfterRestarg {},
                forward_arg.expression(),
            );
        }
    }

    pub(crate) fn check_reserved_for_numparam(&self, name: &str, loc: Loc) -> Result<(), ()> {
        match name {
            "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9" => {
                self.error(
                    DiagnosticMessage::ReservedForNumparam {
                        numparam: self.blob.push_str(name),
                    },
                    loc,
                );
                Err(())
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn arg_name_collides(&self, this_name: &str, that_name: &str) -> bool {
        &this_name[0..1] != "_" && this_name == that_name
    }

    pub(crate) fn check_lvar_name(&self, name: &str, loc: Loc) -> Result<(), ()> {
        let mut all_chars = name.chars();
        let first = all_chars
            .next()
            .expect("local variable name can't be empty");
        let mut rest = all_chars;

        if (first.is_lowercase() || first == '_') && rest.all(|c| c.is_alphanumeric() || c == '_') {
            Ok(())
        } else {
            self.error(DiagnosticMessage::KeyMustBeValidAsLocalVariable {}, loc);
            Err(())
        }
    }

    pub(crate) fn check_duplicate_pattern_variable(
        &self,
        name: &'b str,
        loc: Loc,
    ) -> Result<(), ()> {
        if name.starts_with('_') {
            return Ok(());
        }

        if self.pattern_variables.is_declared(name) {
            self.error(DiagnosticMessage::DuplicateVariableName {}, loc);
            return Err(());
        }

        self.pattern_variables.declare(name, self.blob);
        Ok(())
    }

    pub(crate) fn check_duplicate_pattern_key(&self, name: &'b str, loc: Loc) -> Result<(), ()> {
        if self.pattern_hash_keys.is_declared(name) {
            self.error(DiagnosticMessage::DuplicateKeyName {}, loc);
            return Err(());
        }

        self.pattern_hash_keys.declare(name, self.blob);
        Ok(())
    }

    //
    // Helpers
    //

    pub(crate) fn static_string(nodes: &NodeList<'b>) -> Option<&'b Bytes<'b>> {
        // let mut result = String::from("");

        // for node in nodes.iter() {
        //     match node {
        //         Node::Str(Str { value, .. }) => {
        //             let value = value.to_string_lossy();
        //             result.push_str(value.as_str())
        //         }
        //         Node::Begin(Begin { statements, .. }) => {
        //             if let Some(s) = Self::static_string(statements) {
        //                 result.push_str(&s)
        //             } else {
        //                 return None;
        //             }
        //         }
        //         _ => {
        //             return None;
        //         }
        //     }
        // }

        // Some(result)
        todo!()
    }

    #[cfg(feature = "onig")]
    pub(crate) fn build_static_regexp(
        &self,
        parts: &'b NodeList<'b>,
        options: Option<&String>,
        loc: Loc,
    ) -> Option<Regex> {
        let source = Self::static_string(parts)?;
        let mut reg_options = RegexOptions::REGEX_OPTION_NONE;
        reg_options |= RegexOptions::REGEX_OPTION_CAPTURE_GROUP;
        if let Some(options_s) = options.as_ref().map(|s| s.as_str()) {
            if options_s.as_bytes().contains(&b'x') {
                reg_options |= RegexOptions::REGEX_OPTION_EXTEND;
            }
        }

        let bytes = onig::EncodedBytes::ascii(source.as_bytes());

        match Regex::with_options_and_encoding(bytes, reg_options, onig::Syntax::ruby()) {
            Ok(regex) => Some(regex),
            Err(err) => {
                self.error(
                    DiagnosticMessage::RegexError {
                        error: String::from(err.description()),
                    },
                    loc,
                );
                None
            }
        }
    }

    #[cfg(feature = "onig")]
    pub(crate) fn validate_static_regexp(
        &self,
        parts: &'b NodeList<'b>,
        options: Option<&'b str>,
        loc: Loc,
    ) {
        self.build_static_regexp(parts, options, loc);
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn validate_static_regexp(
        &self,
        _parts: &'b NodeList<'b>,
        _options: Option<&'b str>,
        _loc: Loc,
    ) {
    }

    #[cfg(feature = "onig")]
    pub(crate) fn static_regexp_captures(&self, node: &Node) -> Option<Vec<String>> {
        if let Node::Regexp(Regexp {
            parts,
            options,
            expression_l,
            ..
        }) = node
        {
            let mut re_options = &None;
            if let Some(Node::RegOpt(RegOpt { options, .. })) = options.as_ref().map(|b| &**b) {
                re_options = options;
            };
            let regex = self.build_static_regexp(parts, re_options, expression_l)?;

            let mut result: Vec<String> = vec![];

            regex.foreach_name(|name, _| {
                result.push(name.to_string());
                true
            });

            return Some(result);
        }
        None
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn static_regexp_captures(&self, _node: &Node) -> Option<[&'b Bytes<'b>; 20]> {
        None
    }

    pub(crate) fn loc(&self, token: &Token) -> Loc {
        token.loc
    }

    pub(crate) fn maybe_loc(&self, token: Option<&'b Token<'b>>) -> Option<Loc> {
        token.map(|token| self.loc(token))
    }

    pub(crate) fn collection_map(
        &self,
        begin_l: Option<Loc>,
        parts: &NodeList<'b>,
        end_l: Option<Loc>,
    ) -> CollectionMap {
        let expression_l = collection_expr(parts);
        let expression_l = join_maybe_locs(expression_l, begin_l);
        let expression_l = join_maybe_locs(expression_l, end_l);
        let expression_l = expression_l.unwrap_or_else(|| {
            unreachable!("empty collection without begin_t/end_t, can't build source map")
        });

        CollectionMap {
            begin_l,
            end_l,
            expression_l,
        }
    }

    pub(crate) fn is_heredoc(&self, begin_t: Option<&'b Token<'b>>) -> bool {
        if let Some(begin_t) = begin_t.as_ref() {
            let mut begin = begin_t.token_value.iter();
            if begin.len() >= 2 && begin.next() == Some(b'<') && begin.next() == Some(b'<') {
                return true;
            }
        }
        false
    }

    pub(crate) fn heredoc_map(
        &self,
        begin_t: Option<&'b Token<'b>>,
        parts: &NodeList<'b>,
        end_t: Option<&'b Token<'b>>,
    ) -> HeredocMap {
        let begin_t = begin_t.as_ref().expect("bug: begin_t must be Some");
        let end_t = end_t.as_ref().expect("heredoc must have end_t");

        let heredoc_body_l = collection_expr(parts).unwrap_or_else(|| self.loc(end_t));
        let expression_l = self.loc(begin_t);
        let heredoc_end_l = self.loc(end_t);

        HeredocMap {
            heredoc_body_l,
            heredoc_end_l,
            expression_l,
        }
    }

    pub(crate) fn error(&self, message: DiagnosticMessage<'b>, loc: Loc) {
        let diagnostic = Diagnostic::new(ErrorLevel::Error, message, loc, self.blob);
        self.diagnostics.push(diagnostic)
    }

    pub(crate) fn warn(&self, message: DiagnosticMessage<'b>, loc: Loc) {
        let diagnostic = Diagnostic::new(ErrorLevel::Warning, message, loc, self.blob);
        self.diagnostics.push(diagnostic)
    }

    pub(crate) fn value_expr(&self, node: &'b Node<'b>) -> Result<(), ()> {
        if let Some(void_node) = Self::void_value(node) {
            self.error(
                DiagnosticMessage::VoidValueExpression {},
                void_node.expression(),
            );
            Err(())
        } else {
            Ok(())
        }
    }

    fn void_value(node: &'b Node<'b>) -> Option<&'b Node<'b>> {
        let check_stmts = |statements: &'b NodeList<'b>| {
            if let Some(last_stmt) = statements.last() {
                Self::void_value(last_stmt)
            } else {
                None
            }
        };

        let check_condition = |if_true: &'b Node<'b>, if_false: &'b Node<'b>| {
            if Self::void_value(if_true).is_some() && Self::void_value(if_false).is_some() {
                Some(if_true)
            } else {
                None
            }
        };

        let check_maybe_condition =
            |if_true: &Option<&'b Node<'b>>, if_false: &Option<&'b Node<'b>>| match (
                if_true.as_ref(),
                if_false.as_ref(),
            ) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(if_true), Some(if_false)) => check_condition(if_true, if_false),
            };

        match node {
            Node::Return(_) | Node::Break(_) | Node::Next(_) | Node::Redo(_) | Node::Retry(_) => {
                Some(node)
            }

            Node::MatchPattern(MatchPattern { value, .. })
            | Node::MatchPatternP(MatchPatternP { value, .. }) => Self::void_value(value),

            Node::Begin(Begin { statements, .. }) | Node::KwBegin(KwBegin { statements, .. }) => {
                check_stmts(statements)
            }

            Node::IfTernary(IfTernary {
                if_true, if_false, ..
            }) => check_condition(if_true, if_false),

            Node::If(If {
                if_true, if_false, ..
            })
            | Node::IfMod(IfMod {
                if_true, if_false, ..
            }) => check_maybe_condition(if_true, if_false),

            Node::And(And { lhs, .. }) | Node::Or(Or { lhs, .. }) => Self::void_value(lhs),

            _ => None,
        }
    }

    fn rewrite_hash_args_to_kwargs(&self, args: &'b NodeList<'b>) {
        let len = args.len();

        if len == 0 {
            return;
        }

        let last = args.last().unwrap();

        if matches!(last, Node::Hash(_)) {
            last.change_hash_type_to_kwargs();
            return;
        }

        if len == 1 {
            return;
        }

        let second_last = args.iter().nth(len - 2).unwrap();

        if matches!(last, Node::BlockPass(_)) && self.is_kwargs(second_last) {
            second_last.change_hash_type_to_kwargs()
        }
    }

    fn is_kwargs(&self, node: &Node) -> bool {
        matches!(
            node,
            Node::Hash(Hash {
                begin_l: None,
                end_l: None,
                ..
            })
        )
    }

    fn try_declare_numparam(&self, name: &'b str, loc: Loc) -> bool {
        match name.as_bytes()[..] {
            [b'_', n]
                if (b'1'..=b'9').contains(&n)
                    && !self.static_env.is_declared(name)
                    && self.context.is_in_dynamic_block() =>
            {
                if true {
                    /* definitely an implicit param */

                    if self.max_numparam_stack.has_ordinary_params() {
                        self.error(DiagnosticMessage::OrdinaryParamDefined {}, loc);
                    }

                    let iter = self.max_numparam_stack.iter().rev();

                    /* ignore current block scope */
                    let iter = iter.skip(1);

                    for outer_scope in iter {
                        if outer_scope.is_static {
                            /* found an outer scope that can't have numparams
                            like def/class/etc */
                            break;
                        } else {
                            let outer_scope_has_numparams = outer_scope.value.get() > 0;

                            if outer_scope_has_numparams {
                                self.error(DiagnosticMessage::NumparamUsed {}, loc);
                            } else {
                                /* for now it's ok, but an outer scope can also be a block
                                like proc { _1; proc { proc { proc { _2 }} }}
                                with numparams, so we need to continue */
                            }
                        }
                    }

                    self.static_env.declare(name, self.blob);
                    self.max_numparam_stack.register((n - b'0') as i32);

                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

pub(crate) fn maybe_node_expr(node: Option<&Node>) -> Option<Loc> {
    node.map(|node| node.expression())
}

pub(crate) fn maybe_boxed_node_expr(node: Option<&Node>) -> Option<Loc> {
    node.map(|node| node.expression())
}

pub(crate) fn collection_expr<'b>(nodes: &NodeList<'b>) -> Option<Loc> {
    join_maybe_exprs(nodes.first(), nodes.last())
}

pub(crate) fn maybe_value<'b>(token: Option<&'b Token<'b>>) -> Option<&'b Bytes<'b>> {
    token.map(|t| t.token_value)
}

pub(crate) fn join_exprs(lhs: &Node, rhs: &Node) -> Loc {
    lhs.expression().join(rhs.expression())
}

pub(crate) fn join_maybe_exprs(lhs: Option<&Node>, rhs: Option<&Node>) -> Option<Loc> {
    join_maybe_locs(maybe_node_expr(lhs), maybe_node_expr(rhs))
}

pub(crate) fn join_maybe_locs(lhs: Option<Loc>, rhs: Option<Loc>) -> Option<Loc> {
    match (lhs.as_ref(), rhs.as_ref()) {
        (None, None) => None,
        (None, Some(rhs)) => Some(*rhs),
        (Some(lhs), None) => Some(*lhs),
        (Some(lhs), Some(rhs)) => Some(lhs.join(*rhs)),
    }
}

pub(crate) struct CollectionMap {
    begin_l: Option<Loc>,
    end_l: Option<Loc>,
    expression_l: Loc,
}

pub(crate) struct HeredocMap {
    heredoc_body_l: Loc,
    heredoc_end_l: Loc,
    expression_l: Loc,
}

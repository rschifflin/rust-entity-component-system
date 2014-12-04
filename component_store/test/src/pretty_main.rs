#![feature(phase)]
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate "std" as std;

#[phase(plugin)]
extern crate component_store;
#[prelude_import]
use std::prelude::*;

use std::collections::HashMap;
pub struct Position;
#[automatically_derived]
impl ::std::clone::Clone for Position {
    #[inline]
    fn clone(&self) -> Position { match *self { Position => Position, } }
}
#[automatically_derived]
impl ::std::fmt::Show for Position {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Position =>
            match () {
                () => {
                    #[inline]
                    #[allow(dead_code)]
                    static __STATIC_FMTSTR: &'static [&'static str] =
                        &["Position"];
                    let __args_vec = &[];
                    let __args =
                        unsafe {
                            ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                       __args_vec)
                        };
                    __arg_0.write_fmt(&__args)
                }
            },
        }
    }
}

pub struct Color;
#[automatically_derived]
impl ::std::clone::Clone for Color {
    #[inline]
    fn clone(&self) -> Color { match *self { Color => Color, } }
}
#[automatically_derived]
impl ::std::fmt::Show for Color {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Color =>
            match () {
                () => {
                    #[inline]
                    #[allow(dead_code)]
                    static __STATIC_FMTSTR: &'static [&'static str] =
                        &["Color"];
                    let __args_vec = &[];
                    let __args =
                        unsafe {
                            ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                       __args_vec)
                        };
                    __arg_0.write_fmt(&__args)
                }
            },
        }
    }
}

pub struct Radius;
#[automatically_derived]
impl ::std::clone::Clone for Radius {
    #[inline]
    fn clone(&self) -> Radius { match *self { Radius => Radius, } }
}
#[automatically_derived]
impl ::std::fmt::Show for Radius {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Radius =>
            match () {
                () => {
                    #[inline]
                    #[allow(dead_code)]
                    static __STATIC_FMTSTR: &'static [&'static str] =
                        &["Radius"];
                    let __args_vec = &[];
                    let __args =
                        unsafe {
                            ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                       __args_vec)
                        };
                    __arg_0.write_fmt(&__args)
                }
            },
        }
    }
}

pub struct color_index<'a> {
    primary_index: HashMap<&'a str,
                           Color>,
}
#[automatically_derived]
impl <'a> ::std::fmt::Show for color_index<'a> {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            color_index { primary_index: ref __self_0_0 } =>
            match (&(*__self_0_0),) {
                (__arg0,) => {
                    #[inline]
                    #[allow(dead_code)]
                    static __STATIC_FMTSTR: &'static [&'static str] =
                        &["color_index { primary_index: ", " }"];
                    let __args_vec =
                        &[::std::fmt::argument(::std::fmt::Show::fmt,
                                               __arg0)];
                    let __args =
                        unsafe {
                            ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                       __args_vec)
                        };
                    __arg_0.write_fmt(&__args)
                }
            },
        }
    }
}
#[automatically_derived]
impl <'a> ::std::clone::Clone for color_index<'a> {
    #[inline]
    fn clone(&self) -> color_index<'a> {
        match *self {
            color_index { primary_index: ref __self_0_0 } =>
            color_index{primary_index:
                            ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
impl <'a> color_index<'a> {
    pub fn new() -> color_index<'a> {
        color_index{primary_index: HashMap::new(),}
    }
    pub fn find_all_colors(&self) -> Vec<&Color> {
        self.primary_index.values().collect()
    }
    pub fn find_color(&self, key: &'a str) -> Option<&Color> {
        self.primary_index.get(&key)
    }
    pub fn update_color(&mut self, key: &'a str, value: Color)
     -> Option<Color> {
        self.primary_index.insert(key, value)
    }
    pub fn remove_all_colors(&mut self) {
        self.primary_index = HashMap::new();
    }
    pub fn remove_color(&mut self, key: &'a str) {
        self.primary_index.remove(&key);
    }
}
pub struct position_index<'a> {
    primary_index: HashMap<&'a str, Position>,
}
#[automatically_derived]
impl <'a> ::std::fmt::Show for position_index<'a> {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            position_index { primary_index: ref __self_0_0 } =>
            match (&(*__self_0_0),) {
                (__arg0,) => {
                    #[inline]
                    #[allow(dead_code)]
                    static __STATIC_FMTSTR: &'static [&'static str] =
                        &["position_index { primary_index: ", " }"];
                    let __args_vec =
                        &[::std::fmt::argument(::std::fmt::Show::fmt,
                                               __arg0)];
                    let __args =
                        unsafe {
                            ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                       __args_vec)
                        };
                    __arg_0.write_fmt(&__args)
                }
            },
        }
    }
}
#[automatically_derived]
impl <'a> ::std::clone::Clone for position_index<'a> {
    #[inline]
    fn clone(&self) -> position_index<'a> {
        match *self {
            position_index { primary_index: ref __self_0_0 } =>
            position_index{primary_index:
                               ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
impl <'a> position_index<'a> {
    pub fn new() -> position_index<'a> {
        position_index{primary_index: HashMap::new(),}
    }
    pub fn find_all_positions(&self) -> Vec<&Position> {
        self.primary_index.values().collect()
    }
    pub fn find_position(&self, key: &'a str) -> Option<&Position> {
        self.primary_index.get(&key)
    }
    pub fn update_position(&mut self, key: &'a str, value: Position)
     -> Option<Position> {
        self.primary_index.insert(key, value)
    }
    pub fn remove_all_positions(&mut self) {
        self.primary_index = HashMap::new();
    }
    pub fn remove_position(&mut self, key: &'a str) {
        self.primary_index.remove(&key);
    }
}
pub struct ECS<'a> {
    pub colors: ColorIndex<'a>,
    pub positions: PositionIndex<'a>,
}
#[automatically_derived]
impl <'a> ::std::clone::Clone for ECS<'a> {
    #[inline]
    fn clone(&self) -> ECS<'a> {
        match *self {
            ECS { colors: ref __self_0_0, positions: ref __self_0_1 } =>
            ECS{colors: ::std::clone::Clone::clone(&(*__self_0_0)),
                positions: ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[automatically_derived]
impl <'a> ::std::fmt::Show for ECS<'a> {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ECS { colors: ref __self_0_0, positions: ref __self_0_1 } =>
            match (&(*__self_0_0), &(*__self_0_1)) {
                (__arg0, __arg1) => {
                    #[inline]
                    #[allow(dead_code)]
                    static __STATIC_FMTSTR: &'static [&'static str] =
                        &["ECS { colors: ", ", positions: ", " }"];
                    let __args_vec =
                        &[::std::fmt::argument(::std::fmt::Show::fmt, __arg0),
                          ::std::fmt::argument(::std::fmt::Show::fmt,
                                               __arg1)];
                    let __args =
                        unsafe {
                            ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                       __args_vec)
                        };
                    __arg_0.write_fmt(&__args)
                }
            },
        }
    }
}
impl <'a> ECS<'a> {
    pub fn new() -> ECS<'a> {
        ECS{colors: ColorIndex::new(), positions: PositionIndex::new(),}
    }
    pub fn color_foo() {
        match () {
            () => {
                #[inline]
                #[allow(dead_code)]
                static __STATIC_FMTSTR: &'static [&'static str] = &["Foo"];
                let __args_vec = &[];
                let __args =
                    unsafe {
                        ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                   __args_vec)
                    };
                ::std::io::stdio::println_args(&__args)
            }
        };
    }
    pub fn color_bar() {
        match () {
            () => {
                #[inline]
                #[allow(dead_code)]
                static __STATIC_FMTSTR: &'static [&'static str] = &["Bar"];
                let __args_vec = &[];
                let __args =
                    unsafe {
                        ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                   __args_vec)
                    };
                ::std::io::stdio::println_args(&__args)
            }
        };
    }
    pub fn position_foo() {
        match () {
            () => {
                #[inline]
                #[allow(dead_code)]
                static __STATIC_FMTSTR: &'static [&'static str] = &["Foo"];
                let __args_vec = &[];
                let __args =
                    unsafe {
                        ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                   __args_vec)
                    };
                ::std::io::stdio::println_args(&__args)
            }
        };
    }
    pub fn position_bar() {
        match () {
            () => {
                #[inline]
                #[allow(dead_code)]
                static __STATIC_FMTSTR: &'static [&'static str] = &["Bar"];
                let __args_vec = &[];
                let __args =
                    unsafe {
                        ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                   __args_vec)
                    };
                ::std::io::stdio::println_args(&__args)
            }
        };
    }
}
fn main() {
    let ecs = ECS::new();
    ECS::color_foo();
    ECS::position_bar();
    match (&ecs,) {
        (__arg0,) => {
            #[inline]
            #[allow(dead_code)]
            static __STATIC_FMTSTR: &'static [&'static str] =
                &["The result is "];
            let __args_vec =
                &[::std::fmt::argument(::std::fmt::Show::fmt, __arg0)];
            let __args =
                unsafe {
                    ::std::fmt::Arguments::new(__STATIC_FMTSTR, __args_vec)
                };
            ::std::io::stdio::println_args(&__args)
        }
    };
}

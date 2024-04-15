use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::LinkedList as StdLinkedList;
use fast_graph::LinkedList as FastLinkedList;
//use fast_graph::BasicLinkedList as BasicFastLinkedList;

macro_rules! fast_linked_list_simple_fn {
    ($g: tt, $str: tt,$x: expr, $fn:tt$(($arg:ident))+ ) => {
        $g.bench_function($str, |b| {
            let mut list = FastLinkedList::new();

            b.iter(|| {
                for i in 0..$x {
                    list.$fn(i);
                }
            });
        });
    };
    ($g: tt, $str: tt,$x: expr, $fn:ident() ) => {
        $g.bench_function($str, |b| {
            let mut list = FastLinkedList::new();

            b.iter(|| {
                for i in 0..$x {
                    list.$fn();
                }
            });
        });
    };
}


macro_rules! std_linked_list_simple_fn {
    ($g: tt, $str: tt,$x: expr, $fn:tt$(($arg:ident))+ ) => {
        $g.bench_function($str, |b| {
            let mut list = StdLinkedList::new();

            b.iter(|| {
                for i in 0..$x {
                    list.$fn(i);
                }
            });
        });
    };
    ($g: tt, $str: tt,$x: expr, $fn:ident() ) => {
        $g.bench_function($str, |b| {
            let mut list = FastLinkedList::new();

            b.iter(|| {
                for i in 0..$x {
                    list.$fn();
                }
            });
        });
    };
}


macro_rules! bench {
    ($g: tt, $s: tt, $x: expr, {$($inner:tt)+}) => {
        #[allow(redundant_semicolons, unused_variables)]

        $g.bench_function($s, |b| {
            #[allow(unused_variables, unused_mut)]
            let mut list = ();
            if $s.starts_with("fast_") {
                b.iter(|| {
                    let mut list = FastLinkedList::new();
                    list.extend(0..$x);
                    for i in 0..$x {
                        $($inner)*
                    }
                });
            } else {
                b.iter(|| {
                    let mut list = StdLinkedList::new();
                    list.extend(0..$x);
                    for i in 0..$x {
                        $($inner)*
                    }
                });
            }
            
        });
    };
    ($g: tt, $s: tt, $x: expr, {$($init:tt)+}, {$($inner:tt)+}) => {
        #[allow(redundant_semicolons, unused_variables)]
        $g.bench_function($s, |b| {
            #[allow(unused_variables, unused_mut)]
            b.iter(|| {
                $($init)*
                for i in 0..$x {
                    $($inner)*
                }
            });
        });
    };
}


pub fn bench_linked_list_push_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked_list_push_back");
    fast_linked_list_simple_fn!(group, "fast_linked_list_push_back_1000", 1000, push_back(i) );
    std_linked_list_simple_fn!(group, "std_linked_list_push_back_1000", 1000, push_back(i) );
    fast_linked_list_simple_fn!(group, "fast_linked_list_push_back_10000", 10000, push_back(i) );
    std_linked_list_simple_fn!(group, "std_linked_list_push_back_10000", 10000, push_back(i) );
    group.finish();
}

pub fn bench_linked_list_push_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked_list_push_front");
    fast_linked_list_simple_fn!(group, "fast_linked_list_push_front_1000", 1000, push_front(i) );
    std_linked_list_simple_fn!(group, "std_linked_list_push_front_1000", 1000, push_front(i) );
    fast_linked_list_simple_fn!(group, "fast_linked_list_push_front_10000", 10000, push_front(i) );
    std_linked_list_simple_fn!(group, "std_linked_list_push_front_10000", 10000, push_front(i) );
    group.finish();
}

pub fn bench_linked_list_pop_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked_list_pop_back");
    let mut list = StdLinkedList::<i32>::new();
    let i = 0;
    bench!(group, "fast_linked_list_pop_back_1000", 1000, 
        { if i > 0 { list.pop_back().expect("popback returned None"); } }
    );
    bench!(group, "std_linked_list_pop_back_1000", 1000, 
        { if i > 0 { list.pop_back().expect("popback returned None"); } }
    );
    bench!(group, "fast_linked_list_pop_back_10000", 10000, 
        { if i > 0 { list.pop_back().expect("popback returned None"); } }
    );
    bench!(group, "std_linked_list_pop_back_10000", 10000, 
        { if i > 0 { list.pop_back().expect("popback returned None"); } }
    );
    group.finish();
}

pub fn bench_linked_list_pop_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked_list_pop_front");
    let mut list = StdLinkedList::<i32>::new();
    let i = 0;
    bench!(group, "fast_linked_list_pop_front_1000", 1000, 
        { if i > 0 { list.pop_front().expect("popback returned None"); } }
    );
    bench!(group, "std_linked_list_pop_front_1000", 1000, 
        { if i > 0 { list.pop_front().expect("popback returned None"); } }
    );
    bench!(group, "fast_linked_list_pop_front_10000", 10000, 
        { if i > 0 { list.pop_front().expect("popback returned None"); } }
    );
    bench!(group, "std_linked_list_pop_front_10000", 10000, 
        { if i > 0 { list.pop_front().expect("popback returned None"); } }
    );
    group.finish();
}

pub fn bench_linked_list_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked_list_iter");
    bench!(group, "fast_linked_list_iter_1000", 1, 
        { let mut list = FastLinkedList::new(); list.extend(0..1000); },
        { let count = list.iter_next(list.head.unwrap()).count(); assert!(count == 1000); }
    );
    bench!(group, "std_linked_list_iter_1000", 1, 
        { let mut list = StdLinkedList::new(); list.extend(0..1000); },
        { let count = list.iter().count(); assert!(count == 1000);  }
    );
    bench!(group, "fast_linked_list_iter_10000", 1, 
        { let mut list = FastLinkedList::new(); list.extend(0..10000); },
        { let count = list.iter_next(list.head.unwrap()).count(); assert!(count == 10000); }
    );
    bench!(group, "std_linked_list_iter_10000", 1, 
        { let mut list = StdLinkedList::new(); list.extend(0..10000); },
        { let count = list.iter().count(); assert!(count == 10000);}
    );
    group.finish();
}




criterion_group!(
    benches,
    bench_linked_list_push_back,
    bench_linked_list_push_front,
    bench_linked_list_pop_back,
    bench_linked_list_pop_front,
    bench_linked_list_iter
);

criterion_main!(benches);
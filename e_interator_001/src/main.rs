use std::collections::HashMap;

fn main() {
    {
        for i in 1..23 {
            println!("{}", i);
        }
    }
    {
        let values = vec![1,2,3];
        for x in values {
            println!("{}", x);
        }
    }
    {
        let v: Vec<_> = (1..20).filter(|x| x%2 == 0).collect();
        println!("{:?}", v);
    }
    {
        let v = vec![1, 2, 3, 4, 5, 6];
        let v_take = v.iter()
            .cloned()
            .take(2)
            .collect::<Vec<_>>();
        assert_eq!(v_take, vec![1, 2]);

        let v_skip: Vec<_> = v.iter()
            .cloned()
            .skip(2)
            .collect();
        assert_eq!(v_skip, vec![3, 4, 5, 6]);
    }
    {
        let names = vec!["WaySLOG", "Mike", "Elton"];
        let scores = vec![60, 80, 100];
        let score_map: HashMap<_, _> = names.iter()
            .zip(scores.iter())
            .collect();
        println!("{:?}", score_map);
    }
    {
        let v = vec![1u64, 2, 3, 4, 5, 6];
        let val = v.iter()
            .enumerate()
            // 迭代生成标，并且每两个元素剔除一个
            .filter(|&(idx, _)| idx % 2 == 0)
            // 将下标去除,如果调用unzip获得最后结果的话，可以调用下面这句，终止链式调用
            // .unzip::<_,_, vec<_>, vec<_>>().1
            .map(|(idx, val)| val)
            // 累加 1+3+5 = 9
            .fold(0u64, |sum, acm| sum + acm);

        println!("{}", val);
    }
}

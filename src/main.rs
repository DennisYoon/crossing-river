#![allow(unused)]

use rand::Rng;
enum Side {
  Left,
  Right
}

fn main() {
  // 랜덤
  let mut rng = rand::thread_rng();

  // 현재 배 위치
  let mut turn = Side::Right;

  // 미션 완료 알고리즘
  let mut result = Vec::new();

  // 사람들
  let mut drivers_left = vec!["hunter", "bandit boss", "pirate boss"];
  let mut people_left = vec!["bandit1","dog", "bandit2", "pirate1", "pirate2"];

  let mut drivers_right = vec![];
  let mut people_right = vec![];

  while drivers_right.len() + people_right.len() != 8 {
    let cdl = drivers_left.clone();
    let cpl = people_left.clone();
    let cdr = drivers_right.clone();
    let cpr = people_right.clone();

    let s1k: &mut Vec<&str>;
    let s1: &mut Vec<&str>;
    let s2k: &mut Vec<&str>;
    let s2: &mut Vec<&str>;

    turn = if matches!(turn, Side::Right) {
      s1k = &mut drivers_left;
      s1 = &mut people_left;
      s2k = &mut drivers_right;
      s2 = &mut people_right;

      Side::Left
    } else {
      s2k = &mut drivers_left;
      s2 = &mut people_left;
      s1k = &mut drivers_right;
      s1 = &mut people_right;
    
      Side::Right
    };

    let moving1 = rng.gen_range(0..s1k.len());
    let name1 = s1k[moving1];

    // 배 움직이는 놈 이동
    s1k.remove(moving1);
    s2k.push(name1);

    let moving2 = rng.gen_range(0..(s1k.len() + s1.len() + 1));
    if moving2 != s1k.len() + s1.len() {
      let name2 = if moving2 >= s1k.len() {
        s1[moving2 - s1k.len()]
      } else {
        s1k[moving2]
      };
  
      // 배 같이 탄 놈 이동
      if moving2 >= s1k.len() {
        s1.remove(moving2 - s1k.len());
        s2.push(name2);
      } else {
        s1k.remove(moving2);
        s2k.push(name2);
      }
  
      result.push(format!("{}: {} & {}", if matches!(turn, Side::Left) {"→"} else {"←"}, name1, name2));
    } else {
      result.push(format!("{}: {}", if matches!(turn, Side::Left) {"→"} else {"←"}, name1));
    }
    

    // 움직였을 때 game over인지 확인
    let mut gameover = false;

    if (find(&s1k, "hunter") && !find(&s1, "dog") && s2k.len() + s2.len() != 1) ||
       (find(&s2k, "hunter") && !find(&s2, "dog") && s1k.len() + s1.len() != 1)
    {
      gameover = true;
      // 원인: 개와 사냥꾼의 분리
    }

    if (
      find(&s1k, "bandit boss") &&
      !find(&s1k, "pirate boss") &&
      (find(&s1, "pirate1") || find(&s1, "pirate2"))
    ) ||
    (
      find(&s2k, "bandit boss") &&
      !find(&s2k, "pirate boss") &&
      (find(&s2, "pirate1") || find(&s2, "pirate2"))
    ) {
      gameover = true;
      // 원인: 해적두목이 해적들 지키지 못함
    }

    if (
      find(&s1k, "pirate boss") &&
      !find(&s1k, "bandit boss") &&
      (find(&s1, "bandit1") || find(&s1, "bandit2"))
    ) ||
    (
      find(&s2k, "pirate boss") &&
      !find(&s2k, "bandit boss") &&
      (find(&s2, "bandit1") || find(&s2, "bandit2"))
    ) {
      gameover = true;
      // 원인: 해적두목이 해적들 지키지 못함
    }
    
    
    // game over시
    if gameover {
      result.pop();
      drivers_left = cdl;
      people_left = cpl;
      drivers_right = cdr;
      people_right = cpr;

      turn = if matches!(turn, Side::Right) {  
        Side::Left
      } else {
        Side::Right
      };
    } else {
      // println!("{:?}{:?}", drivers_left, people_left);
      // println!("{:?}{:?}", drivers_right, people_right);
      // println!();
    }
  } // right 쪽에 8객체 모두 이동!

  if result.len() >= 50 {
    main();
  } else {
    for (i, content) in result.iter().enumerate() {
      println!("[{}] {}", i + 1, content);
    }
  }
  
}

fn find(vector: &Vec<&str>, element: &str) -> bool {
  let result = vector.iter().find(|&x| *x == element);
  return match result {
    Some(_) => true,
    None => false
  };
}

fn some(element: Option<&str>) -> &str {
  return match element {
    Some(v) => v,
    None => ""
  };
}
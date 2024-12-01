use std::fs;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    //expect 434

    // 574
    // 575
    // 492

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());


    println!("Solution: {} Took {:?}", part1(input.clone()), start.elapsed());
    println!("Solution: {} Took {:?}", part2(input), start.elapsed());
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
  x: isize,
  y: isize,
  z: isize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Brick {
  start: Point,
  end: Point,
}

#[derive(Debug, Clone)]
struct Space {
  bricks: Vec<Brick>,
}

impl Point {
  fn from_str(s: &str) -> Point {
      let xyz: Vec<_> = s.split(",").collect();
      if xyz.len() != 3 {
          panic!("Invalid point: {}", s)
      }
      Point {
          x: xyz[0].parse().expect("Invalid x"),
          y: xyz[1].parse().expect("Invalid y"),
          z: xyz[2].parse().expect("Invalid z"),
      }
  }
}

impl Brick {
  fn from_str(s: &str) -> Brick {
      let (from, to) = s.split_once("~").expect("Invalid brick");
      let from = Point::from_str(from);
      let to = Point::from_str(to);
      Brick::new(from, to)
  }
  fn new(start: Point, end: Point) -> Brick {
      assert!(start.z <= end.z);
      assert!(start.x <= end.x);
      assert!(start.y <= end.y);
      Brick { start, end }
  }

  fn fall_down(&mut self) {
      self.start.z -= 1;
      self.end.z -= 1;
  }

  fn contains(&self, point: &Point) -> bool {
      point.x >= self.start.x
          && point.x <= self.end.x
          && point.y >= self.start.y
          && point.y <= self.end.y
          && point.z >= self.start.z
          && point.z <= self.end.z
  }

  fn _cube_iter(&self) -> BrickIterator {
      let step;
      if self.start.x != self.end.x {
          step = Point { x: 1, y: 0, z: 0 };
      } else if self.start.y != self.end.y {
          step = Point { x: 0, y: 1, z: 0 };
      } else {
          step = Point { x: 0, y: 0, z: 1 };
      }
      BrickIterator {
          brick: *self,
          current: self.start,
          step,
      }
  }

  fn xy_cube_iter(&self) -> BrickIterator {
      let step;
      if self.start.x == self.end.x && self.start.y == self.end.y {
          step = Point { x: 1, y: 1, z: 1 };
      } else if self.start.x != self.end.x {
          step = Point { x: 1, y: 0, z: 0 };
      } else {
          step = Point { x: 0, y: 1, z: 0 };
      }
      BrickIterator {
          brick: *self,
          current: self.start,
          step,
      }
  }
}

struct BrickIterator {
  brick: Brick,
  current: Point,
  step: Point,
}

impl Iterator for BrickIterator {
  type Item = Point;

  fn next(&mut self) -> Option<Point> {
      if !self.brick.contains(&self.current) {
          return None;
      }
      let ret = self.current;
      self.current.x += self.step.x;
      self.current.y += self.step.y;
      self.current.z += self.step.z;
      Some(ret)
  }
}

impl Space {
  fn new() -> Space {
      Space { bricks: Vec::new() }
  }

  fn add_brick(&mut self, brick: Brick) {
      self.bricks.push(brick);
  }

  fn is_empty_xy(&self, point: &Point) -> bool {
      self.bricks
          .iter()
          .all(|brick| point.z > 0 && !brick.contains(point))
  }

  fn can_brick_fall(&self, brick: &Brick) -> bool {
      let mut brick = brick.clone();
      brick.fall_down();
      for point in brick.xy_cube_iter() {
          if !self.is_empty_xy(&point) {
              return false;
          }
      }
      true
  }

  fn next_brick_to_fall(&self) -> Option<usize> {
      for (index, brick) in self.bricks.iter().enumerate() {
          if self.can_brick_fall(brick) {
              return Some(index);
          }
      }
      None
  }

  fn fall_bricks_down(&mut self) -> usize {
      let mut affected = vec![];
      while let Some(idx) = self.next_brick_to_fall() {
          if !affected.contains(&idx) {
              affected.push(idx);
          }
          while self.can_brick_fall(&self.bricks[idx]) {
              self.bricks[idx].fall_down();
          }
      }
      affected.len()
  }

  fn _show(&self) {
      for brick in &self.bricks {
          println!("{:?}", brick);
          for point in brick._cube_iter() {
              println!("-- {:?}", point);
          }
          println!();
      }
  }
}

fn parse_input(str: &str) -> Space {
  let mut space = Space::new();
  str.lines().for_each(|line| {
      space.add_brick(Brick::from_str(line));
  });
  space
}

// totally unoptimized
pub fn part1(input: String) -> u64 {
  let mut space = parse_input(&input);
  space.fall_bricks_down();

  let mut cnt = 0;
  for (i, _) in space.bricks.iter().enumerate() {
      println!("{}/{}", i, space.bricks.len());
      let mut space2 = space.clone();
      space2.bricks.remove(i);
      if space2.next_brick_to_fall().is_none() {
          cnt += 1;
      }
  }
  cnt as u64
}

pub fn part2(input: String) -> u64 {
  let mut space = parse_input(&input);
  space.fall_bricks_down();

  let mut cnt = 0;
  for (i, _) in space.bricks.iter().enumerate() {
      let mut space2 = space.clone();
      space2.bricks.remove(i);
      let fallen = space2.fall_bricks_down();
      println!("{}/{} -> {}", i, space.bricks.len(), fallen);
      cnt += fallen;
  }
  cnt as u64
}

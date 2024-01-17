use regex::Regex;

type Coord = (i64, i64, i64);

#[derive(Debug)]
struct Nanobot {
  position: Coord,
  range: i64,
}

impl Nanobot {
  fn parse(text: &str) -> Self {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let a: Vec<i64> = re.captures(text).unwrap().iter().skip(1)
      .map(|m| m.unwrap().as_str().parse::<i64>().unwrap()).collect();
    Nanobot { position: (a[0], a[1], a[2]), range: a[3] }
  }

  fn distance(&self, other: &Nanobot) -> i64 {
    (self.position.0 - other.position.0).abs() +
    (self.position.1 - other.position.1).abs() +
    (self.position.2 - other.position.2).abs()
  }
}

fn in_range_largest(bots: &[Nanobot]) -> usize {
  let max_range = bots.iter().map(|b| b.range).max().unwrap();
  let bot = bots.iter().filter(|b| b.range == max_range).next().unwrap();
  bots.iter().filter(|b| bot.distance(b) <= bot.range).count()
}

pub fn run(content: &str) {
  let bots = content.lines().map(Nanobot::parse).collect::<Vec<_>>();
  let res1 = in_range_largest(&bots);
  println!("{}", res1);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

  #[test]
  fn small() {
    let bots = TEST.lines().map(super::Nanobot::parse).collect::<Vec<_>>();
    assert_eq!(super::in_range_largest(&bots), 7);
  }
}

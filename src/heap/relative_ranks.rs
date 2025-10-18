#[test]
fn runner() {
    let result = Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]);
    println!("{:?}", result);
}

struct Solution;

impl Solution {

    const GOLD: &'static str = "Gold Medal";
    const SILVER: &'static str = "Silver Medal";
    const BRONZE: &'static str = "Bronze Medal";

    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut heap = Heap::new(score.len());
        for (i, value) in score.iter().enumerate() {
            heap.add(Player::new(i, *value));
        }

        let mut result: Vec<Option<String>> = vec![None; score.len()];
        let mut counter = 0;
        loop {
            let option = heap.get_max();
            if option.is_none() {
                break;
            }

            let player = option.unwrap();
            counter += 1;
            result[player.index] = Some(Self::convert(counter));
        }

        result.into_iter().map(|x| x.unwrap()).collect()
    }

    fn convert(counter: i32) -> String {
        if counter == 1 {
            return String::from(Self::GOLD);
        }
        if counter == 2 {
            return String::from(Self::SILVER);
        }
        if counter == 3 {
            return String::from(Self::BRONZE);
        }

        counter.to_string()
    }
}

struct Heap {
    arr: Vec<Option<Player>>,
    size: usize
}

impl Heap {

    fn new(players_count: usize) -> Self {
        Heap {
            arr: vec![None; players_count + 1],
            size: 0
        }
    }

    fn add(&mut self, player: Player) {
        self.size += 1;
        self.arr[self.size] = Some(player);

        let mut index = self.size;
        loop {
            if index < 2 {
                break;
            }

            let parent_index = index / 2;
            if self.arr[parent_index].as_ref().unwrap().rank < self.arr[index].as_ref().unwrap().rank {
                self.arr.swap(parent_index, index);
            } else {
                break;
            }

            index = parent_index;
        }
    }

    fn get_max(&mut self) -> Option<Player> {
        if self.size == 0 {
            return None;
        }

        let max: Player = self.arr[1].take().unwrap();
        self.arr[1] = self.arr[self.size].take();
        self.size -= 1;

        let mut index = 1;
        loop {
            if index > self.size {
                break;
            }

            let mut max_index = index;
            let left_index = index*2;
            let right_index = index*2 + 1;
            if (left_index <= self.size) && (self.arr[left_index].as_ref().unwrap().rank > self.arr[max_index].as_ref().unwrap().rank) {
                max_index = left_index;
            }

            if (right_index <= self.size) && (self.arr[right_index].as_ref().unwrap().rank > self.arr[max_index].as_ref().unwrap().rank) {
                max_index = right_index;
            }

            if index != max_index {
                self.arr.swap(max_index, index);
                index = max_index;
            } else {
                break;
            }
        }

        Some(max)
    }
}

#[derive(Clone)]
struct Player {
    index: usize,
    rank: i32
}

impl Player {
    fn new(index: usize, rank: i32) -> Player {
        Player {
            index,
            rank
        }
    }
}
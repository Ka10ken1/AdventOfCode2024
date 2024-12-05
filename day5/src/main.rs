use std::{collections::HashMap, fs::read_to_string, path::Path};

fn main() {
    let path = Path::new("src/data.txt");
    let lines : Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();

    let mut page_ordering : Vec<String> = Vec::new();
    let mut pages_to_produce : Vec<String> = Vec::new();
    for line in lines {
        if line.contains("|") {
            page_ordering.push(line);
        }
        else if line.contains(",") {
            pages_to_produce.push(line);
        }
    }

    let mp : HashMap<i32,Vec<i32>> = page_ordering.iter()
        .map(|line|{
            let parts : Vec<i32> =  line.split('|')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            (parts[1], parts[0])
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert_with(Vec::new).push(value);
            acc
        });

    let pages_arr : Vec<Vec<i32>> = pages_to_produce.iter()
        .map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let mut good_page : Vec<Vec<i32>> = Vec::new();
    let mut incorrect_updates: Vec<Vec<i32>> = Vec::new();
    
    'outer: for page in pages_arr {
        let n = page.len();
        for i in 0..n {
            for j in  i+1..n {
                if let Some(value) = mp.get(&page[i]) {
                    if value.contains(&page[j]){
                        incorrect_updates.push(page);
                        continue 'outer;
                    }
                }
            }
        }
        good_page.push(page);
    }

    let mut reordered_updates: Vec<Vec<i32>> = Vec::new();
    for update in incorrect_updates {
        reordered_updates.push(topo(&update, &mp));
    }

    let mut sum = 0;

    // part 1
    // for row in good_page {
    //     let n = row.len();
    //     sum += row[n/2];
    // }

    for row in reordered_updates {
        let n = row.len();
        sum += row[n/2];
    }


    println!("{:?}", sum);

}

fn topo(pages: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();

    for &page in pages {
        in_degree.entry(page).or_insert(0);
        if let Some(predecessors) = rules.get(&page) {
            for &pred in predecessors {
                if pages.contains(&pred) {
                    adj_list.entry(pred).or_insert_with(Vec::new).push(page);
                    *in_degree.entry(page).or_insert(0) += 1;
                }
            }
        }
    }

    let mut queue: Vec<i32> = in_degree.iter().filter(|&(_, &deg)| deg == 0).map(|(&k, _)| k).collect();
    let mut sorted: Vec<i32> = Vec::new();

    while let Some(node) = queue.pop() {
        sorted.push(node);
        if let Some(neighbors) = adj_list.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }

    sorted
}

use csv;

fn main() {
    let file_path = "input.txt";
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path);
    let mut locations_one: Vec<i32> = Vec::new();
    let mut locations_two: Vec<i32> = Vec::new();
    for result in rdr.unwrap().records() {
        let record = result.unwrap();
        let mut locats = record[0].split_whitespace();
        let locat_one = locats.next().unwrap_or("").parse::<i32>().unwrap();
        let locat_two = locats.next().unwrap_or("").parse::<i32>().unwrap();
        locations_one.push(locat_one);
        locations_two.push(locat_two);
    }
    let total_distance = calculate_total_distance(locations_one.clone(), locations_two.clone());
    println!("Total distance: {}", total_distance);
    let similarity = calculate_similarity(locations_one.clone(), locations_two.clone());
    println!("Similarity: {}", similarity);
}

fn calculate_similarity(mut locations_one: Vec<i32>, mut locations_two: Vec<i32>) -> u32 {
    locations_one.sort();
    locations_two.sort();
    let mut similarity = 0;
    for locat in locations_one {
        let increment: i32 = locations_two
            .iter()
            .filter(|&x| x == &locat)
            .count()
            .try_into()
            .unwrap();
        similarity = similarity + increment * locat;
        println!("Increment: {}", increment);
    }
    return similarity.try_into().unwrap();
}

fn calculate_total_distance(mut locations_one: Vec<i32>, mut locations_two: Vec<i32>) -> u32 {
    locations_one.sort();
    locations_two.sort();
    let zipped_locations = locations_one.into_iter().zip(locations_two.into_iter());
    let mut total_distance = 0;
    for location in zipped_locations {
        let diff = location.0.abs_diff(location.1);
        total_distance = total_distance + diff;
    }
    return total_distance;
}

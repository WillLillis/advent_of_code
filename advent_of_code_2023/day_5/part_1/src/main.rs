use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum GardenItem {
    Seed(usize),
    Soil(usize),
    Fertilizer(usize),
    Water(usize),
    Light(usize),
    Temperature(usize),
    Humidity(usize),
    Location(usize),
}

#[derive(Debug, Clone)]
struct RangeTranslation {
    source_range: Range<usize>,
    dest_range: Range<usize>,
}

#[derive(Debug, Clone)]
struct Map {
    ranges: Vec<RangeTranslation>,
}

impl FromIterator<RangeTranslation> for Map {
    fn from_iter<T: IntoIterator<Item = RangeTranslation>>(iter: T) -> Self {
        Map {
            ranges: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fert: Map,
    fert_to_water: Map,
    water_to_light: Map,
    light_to_temp: Map,
    temp_to_humid: Map,
    humid_to_loc: Map,
}

impl Almanac {
    pub fn translate_seeds(&self) -> Vec<GardenItem> {
        self.seeds
            .iter()
            .map(|seed| self.translate_seed(*seed))
            .collect()
    }

    fn translate_seed(&self, seed: usize) -> GardenItem {
        let seed = GardenItem::Seed(seed);
        let soil = self.use_map(seed);
        let fertilizer = self.use_map(soil);
        let water = self.use_map(fertilizer);
        let light = self.use_map(water);
        let temperature = self.use_map(light);
        let humidity = self.use_map(temperature);
        let location = self.use_map(humidity);

        location
    }

    fn use_map(&self, input: GardenItem) -> GardenItem {
        match input {
            GardenItem::Seed(seed) => {
                GardenItem::Soil(Self::do_translate(seed, &self.seed_to_soil))
            }
            GardenItem::Soil(soil) => {
                GardenItem::Fertilizer(Self::do_translate(soil, &self.soil_to_fert))
            }
            GardenItem::Fertilizer(fert) => {
                GardenItem::Water(Self::do_translate(fert, &self.fert_to_water))
            }
            GardenItem::Water(water) => {
                GardenItem::Light(Self::do_translate(water, &self.water_to_light))
            }
            GardenItem::Light(light) => {
                GardenItem::Temperature(Self::do_translate(light, &self.light_to_temp))
            }
            GardenItem::Temperature(temp) => {
                GardenItem::Humidity(Self::do_translate(temp, &self.temp_to_humid))
            }
            GardenItem::Humidity(humid) => {
                GardenItem::Location(Self::do_translate(humid, &self.humid_to_loc))
            }
            GardenItem::Location(_) => {
                panic!("No translation exists for location items!");
            }
        }
    }

    fn do_translate(val: usize, trans: &Map) -> usize {
        for range in trans.ranges.iter() {
            if range.source_range.contains(&val) {
                return range.dest_range.start + (val - range.source_range.start);
            }
        }
        val
    }
}

fn get_almanac(input: &str) -> Almanac {
    // seeds listed first
    let seeds = input
        .lines()
        .into_iter()
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .into_iter()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let seed_to_soil = input
        .lines()
        .into_iter()
        .skip_while(|line| !line.contains("seed-to-soil"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            println!("Line: {}", line);
            let nums: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            assert!(nums.len() == 3);
            let dest_start = nums[0];
            let src_start = nums[1];
            let range_len = nums[2];

            RangeTranslation {
                source_range: (src_start..src_start + range_len),
                dest_range: (dest_start..dest_start + range_len),
            }
        })
        .collect();

    let soil_to_fert = input
        .lines()
        .into_iter()
        .skip_while(|line| !line.contains("soil-to-fertilizer"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            assert!(nums.len() == 3);
            let dest_start = nums[0];
            let src_start = nums[1];
            let range_len = nums[2];

            RangeTranslation {
                source_range: (src_start..src_start + range_len),
                dest_range: (dest_start..dest_start + range_len),
            }
        })
        .collect();

    let fert_to_water = input
        .lines()
        .into_iter()
        .skip_while(|line| !line.contains("fertilizer-to-water"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            assert!(nums.len() == 3);
            let dest_start = nums[0];
            let src_start = nums[1];
            let range_len = nums[2];

            RangeTranslation {
                source_range: (src_start..src_start + range_len),
                dest_range: (dest_start..dest_start + range_len),
            }
        })
        .collect();

    let water_to_light = input
        .lines()
        .into_iter()
        .skip_while(|line| !line.contains("water-to-light"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            assert!(nums.len() == 3);
            let dest_start = nums[0];
            let src_start = nums[1];
            let range_len = nums[2];

            RangeTranslation {
                source_range: (src_start..src_start + range_len),
                dest_range: (dest_start..dest_start + range_len),
            }
        })
        .collect();

    let light_to_temp = input
        .lines()
        .into_iter()
        .skip_while(|line| !line.contains("light-to-temperature"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            assert!(nums.len() == 3);
            let dest_start = nums[0];
            let src_start = nums[1];
            let range_len = nums[2];

            RangeTranslation {
                source_range: (src_start..src_start + range_len),
                dest_range: (dest_start..dest_start + range_len),
            }
        })
        .collect();

    let temp_to_humid = input
        .lines()
        .into_iter()
        .skip_while(|line| !line.contains("temperature-to-humidity"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            assert!(nums.len() == 3);
            let dest_start = nums[0];
            let src_start = nums[1];
            let range_len = nums[2];

            RangeTranslation {
                source_range: (src_start..src_start + range_len),
                dest_range: (dest_start..dest_start + range_len),
            }
        })
        .collect();

    let humid_to_loc = input
        .lines()
        .into_iter()
        .skip_while(|line| !line.contains("humidity-to-location"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<usize> = line
                .split_whitespace()
                .into_iter()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            assert!(nums.len() == 3);
            let dest_start = nums[0];
            let src_start = nums[1];
            let range_len = nums[2];

            RangeTranslation {
                source_range: (src_start..src_start + range_len),
                dest_range: (dest_start..dest_start + range_len),
            }
        })
        .collect();

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humid,
        humid_to_loc,
    }
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file.");

    let almanac = get_almanac(&input);
    println!("{:#?}", almanac);
    let locations = almanac.translate_seeds();
    println!("{:#?}", locations);
    println!("Lowest location: {:#?}", locations.iter().min().unwrap());
}

use std::cmp::{min, Ordering};
use std::ops::Range;

pub(super) struct SeedData {
    pub seed: u64,
    pub soil: u64,
    pub fertilizer: u64,
    pub water: u64,
    pub light: u64,
    pub temperature: u64,
    pub humidity: u64,
    pub location: u64,
}

impl SeedData {
    fn empty() -> Self {
        Self {
            seed: 0,
            soil: 0,
            fertilizer: 0,
            water: 0,
            light: 0,
            temperature: 0,
            humidity: 0,
            location: 0,
        }
    }

    fn get_prop_mut(&mut self, category: Category) -> &mut u64 {
        match category {
            Category::Seed => &mut self.seed,
            Category::Soil => &mut self.soil,
            Category::Fertilizer => &mut self.fertilizer,
            Category::Water => &mut self.water,
            Category::Light => &mut self.light,
            Category::Temperature => &mut self.temperature,
            Category::Humidity => &mut self.humidity,
            Category::Location => &mut self.location,
        }
    }

    fn fill_props(&mut self, category: Category, value: u64, category_maps: &[CategoryMap]) {
        *self.get_prop_mut(category) = value;

        let Some(category_map) = category_maps.get(category as usize) else {
            return;
        };

        let destination_category = category_map.destination;
        let destination_value = category_map.map_value(value);

        self.fill_props(destination_category, destination_value, category_maps)
    }

    fn build(initial_data: u64, initial_category: Category, category_maps: &[CategoryMap]) -> Self {
        let mut result = Self::empty();
        result.fill_props(initial_category, initial_data, category_maps);

        result
    }

    pub fn build_for_seed(seed: u64, category_maps: &[CategoryMap]) -> Self {
        Self::build(seed, Category::Seed, category_maps)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Category {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) struct MapRange {
    pub source_start: u64,
    pub destination_start: u64,
    pub length: u64,
}

impl MapRange {
    /// Exclusive end of source range
    fn source_end(&self) -> u64 {
        self.source_start + self.length
    }

    fn map_value_unchecked(&self, source_value: u64) -> u64 {
        let offset = source_value - self.source_start;
        self.destination_start + offset
    }

    // Returns Equal if the value lies within the range.
    // Otherwise, returns Less is the range is less than the value,
    // and Greater if the range is greater than the value
    fn compare_to_source_value(&self, source_value: u64) -> Ordering {
        match self.source_start.cmp(&source_value) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => {
                if source_value < self.source_end() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct CategoryMap {
    pub source: Category,
    pub destination: Category,
    pub ranges: Vec<MapRange>,
}

impl CategoryMap {
    fn map_value(&self, source_val: u64) -> u64 {
        if let Ok(index) = self
            .ranges
            .binary_search_by(|test_range| test_range.compare_to_source_value(source_val))
        {
            self.ranges[index].map_value_unchecked(source_val)
        } else {
            source_val
        }
    }

    /// Maps the given source range to one or more destination ranges,
    /// which are pushed into the provided output vector.
    pub fn map_range_into(&self, source_range: &Range<u64>, output: &mut Vec<Range<u64>>) {
        let mut source_start = source_range.start;
        while source_start < source_range.end {
            match self
                .ranges
                .binary_search_by(|test_range| test_range.compare_to_source_value(source_start))
            {
                Ok(index) => {
                    let mapping_range = &self.ranges[index];
                    let source_end = min(source_range.end, mapping_range.source_end());

                    let destination_start = mapping_range.map_value_unchecked(source_start);
                    let destination_end = mapping_range.map_value_unchecked(source_end);
                    output.push(destination_start..destination_end);
                    source_start = source_end;
                }

                Err(index) => match self.ranges.get(index) {
                    Some(next_mapping_range) => {
                        let source_end = min(source_range.end, next_mapping_range.source_start);

                        // These values map to themselves.
                        output.push(source_start..source_end);
                        source_start = source_end;
                    }

                    None => {
                        // All remaining values map to themselves.
                        let source_end = source_range.end;

                        output.push(source_start..source_end);
                        break;
                    }
                },
            }
        }
    }
}

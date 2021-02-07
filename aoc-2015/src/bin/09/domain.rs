use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    str::Lines,
};

use itertools::{Itertools, MinMaxResult};

pub struct Route<'r> {
    from: &'r str,
    to: &'r str,
    dist: usize,
}

impl<'r> From<&'r str> for Route<'r> {
    fn from(s: &'r str) -> Self {
        let mut it = s.split_whitespace();

        let from = it.next().expect("Could not get the `from` destination");
        let to = it.nth(1).expect("Could not get the `to` destination");
        let dist = it
            .nth(1)
            .expect("Could not get the distance")
            .parse()
            .unwrap();

        Route { from, to, dist }
    }
}

pub struct RouteMap<'r>(pub Vec<Route<'r>>);

impl<'r> From<Lines<'r>> for RouteMap<'r> {
    fn from(lines: Lines<'r>) -> Self {
        let mut routes = Vec::with_capacity(lines.clone().count());
        for line in lines {
            routes.push(Route::from(line));
        }
        RouteMap(routes)
    }
}

impl<'r> FromIterator<Route<'r>> for MinMaxResult<usize> {
    fn from_iter<T: IntoIterator<Item = Route<'r>>>(iter: T) -> Self {
        let mut cities = HashSet::new();
        let mut dists = HashMap::new();

        for route in iter {
            cities.insert(route.from);
            cities.insert(route.to);
            dists.insert((route.from, route.to), route.dist);
            dists.insert((route.to, route.from), route.dist);
        }

        cities
            .iter()
            .permutations(cities.len())
            .map(|route| {
                route
                    .as_slice()
                    .windows(2)
                    // Unwrapping here is fine, this can never panic because of the inserts in the
                    // for loop above.
                    .map(|cities| dists.get(&(cities[0], cities[1])).unwrap())
                    .sum()
            })
            .minmax()
    }
}

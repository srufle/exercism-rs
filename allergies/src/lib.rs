// Inspired by
// https://exercism.org/tracks/rust/exercises/allergies/solutions/amcoder
pub struct Allergies {
    score: u32,
}

// Added values to enum, nicely encapsulates what I was previously doing in a method
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

// Created list of allergens
use Allergen::*;
const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        dbg!(score);
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = self.score;
        let allergen = *allergen as u32;
        score & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect::<Vec<Allergen>>()
    }
}

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        dbg!(score);
        Allergies { score }
    }

    fn is_allergic(score: u32, place: i32, val: u32) -> bool {
        (score & (1 << place)) == val
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = self.score;
        use Allergen::*;
        let is_allergic = match allergen {
            Eggs => Allergies::is_allergic(score, 0, 1),
            Peanuts => Allergies::is_allergic(score, 1, 2),
            Shellfish => Allergies::is_allergic(score, 2, 4),
            Strawberries => Allergies::is_allergic(score, 3, 8),
            Tomatoes => Allergies::is_allergic(score, 4, 16),
            Chocolate => Allergies::is_allergic(score, 5, 32),
            Pollen => Allergies::is_allergic(score, 6, 64),
            Cats => Allergies::is_allergic(score, 7, 128),
        };
        is_allergic
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        let allergens = [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ];
        allergens
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect::<Vec<Allergen>>()
    }
}

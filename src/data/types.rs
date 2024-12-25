#[derive(Debug)]
enum Solubility {
    Fat,
    Water,
}

pub struct Vitamin {
    pub name: &'static str,
    pub lower_limit: f32,
    pub upper_limit: f32,
    pub solubility: Solubility,
    pub notes: &'static str,
}

// Vitamin C, some B1, some B3, some B6, some B9, and some B12 can be reduced if exposed to water (such as boiling and discarding the water)
// Vitamin some B1, some B5, some B6, and some B9 can be reduced if exposed to heat
const static VITAMINS: [&'static Vitamin; 13] = [
    &Vitamin {
        name: "Vitamin A",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Fat,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B1",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B2",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B3",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B5",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B6",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B7",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B9",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin B12",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin C",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin D",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Fat,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin E",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Fat,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin K1",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Fat,
        notes: "",
    },
    &Vitamin {
        name: "Vitamin K2",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Fat,
        notes: "",
    },
    &Vitamin {
        name: "Choline",
        lower_limit: 0.0,
        upper_limit: 0.0,
        solubility: Solubility::Water,
        notes: "",
    },
];

pub enum unit {
    Cups,
    Tablespoons,
    Teaspoons,
    Piece,
    Oz,
    FlOz,
}

pub struct VitaminAmount {
    pub vitamin: &'static Vitamins,
    pub amount: f32,
}

#[derive(Debug)]
pub struct Nutrients {
    pub calories: f32,
    pub vitamins: Vec<VitaminAmount>,
    pub minerals: Vec<&'static str>, // todo: add minerals
}

#[derive(Debug)]
pub struct Ingredient {
    pub name: &'static str,
    pub nutrients_per_cup: Option<&'static Nutrients>,
    pub composition: Option<&'static [Ingredient]>,
    pub alternatives: Option<&'static [Ingredient]>,
}

#[derive(Debug)]
pub struct MenuIngredient {
    pub ingredient: &'static Ingredient,
    pub amount: f32,
    pub unit: &'static unit,
}

#[derive(Debug)]
pub struct Step {
    pub description: &'static str,
}

#[derive(Debug)]
pub struct Menu {
    pub name: &'static str,
    pub ingredients: &'static [MenuIngredient],
    pub steps: &'static [Step],
}

#[derive(Debug)]
pub struct SavedMenuIngredient {
    pub ingredient: &'static str,
    pub amount: f32,
    pub unit: &'static unit,
}

#[derive(Debug)]
pub struct SavedMenu {
    pub name: &'static str,
    pub ingredients: &'static [SavedMenuIngredient],
    pub steps: &'static [Step],
}